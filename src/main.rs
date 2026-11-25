use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::collections::VecDeque;
use std::io::{self, stdout, Stdout, Write};

// ---------------------------------------------------------------- matrices

#[derive(Clone, Copy)]
struct Mat {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

fn dot(x1: i64, y1: i64, x2: i64, y2: i64) -> Option<i64> {
    x1.checked_mul(y1)?.checked_add(x2.checked_mul(y2)?)
}

impl Mat {
    const IDENTITY: Mat = Mat { a: 1, b: 0, c: 0, d: 1 };

    /// self * o, or None if any intermediate value overflows i64.
    fn checked_mul(self, o: Mat) -> Option<Mat> {
        Some(Mat {
            a: dot(self.a, o.a, self.b, o.c)?,
            b: dot(self.a, o.b, self.b, o.d)?,
            c: dot(self.c, o.a, self.d, o.c)?,
            d: dot(self.c, o.b, self.d, o.d)?,
        })
    }
}

const L: Mat = Mat { a: 1, b: 0, c: 1, d: 1 };
const R: Mat = Mat { a: 1, b: 1, c: 0, d: 1 };
const L_INV: Mat = Mat { a: 1, b: 0, c: -1, d: 1 };
const R_INV: Mat = Mat { a: 1, b: -1, c: 0, d: 1 };

const L_SWP: Mat = Mat { a: 1, b: 1, c: 1, d: 0 };
const R_SWP: Mat = Mat { a: 0, b: 1, c: 1, d: 1 };
const L_SWP_INV: Mat = Mat { a: 0, b: 1, c: 1, d: -1 };
const R_SWP_INV: Mat = Mat { a: -1, b: 1, c: 1, d: 0 };

// ------------------------------------------------------------ steps & moves

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn flip(self) -> Side {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}

/// One factor of the path: L, R, or an inverse, in plain or swap flavor.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Step {
    side: Side, // which matrix: L or R
    swap: bool,
    inverse: bool,
}

impl Step {
    fn matrix(self) -> Mat {
        match (self.swap, self.side, self.inverse) {
            (false, Side::Left, false) => L,
            (false, Side::Right, false) => R,
            (false, Side::Left, true) => L_INV,
            (false, Side::Right, true) => R_INV,
            (true, Side::Left, false) => L_SWP,
            (true, Side::Right, false) => R_SWP,
            (true, Side::Left, true) => L_SWP_INV,
            (true, Side::Right, true) => R_SWP_INV,
        }
    }

    fn glyph(self) -> char {
        match (self.swap, self.side, self.inverse) {
            (false, Side::Left, false) => '←',  // L
            (false, Side::Right, false) => '→', // R
            (false, Side::Left, true) => '↚',   // L⁻¹
            (false, Side::Right, true) => '↛',  // R⁻¹
            (true, Side::Left, false) => '⇐',   // Ls
            (true, Side::Right, false) => '⇒',  // Rs
            (true, Side::Left, true) => '⇍',    // Ls⁻¹
            (true, Side::Right, true) => '⇏',   // Rs⁻¹
        }
    }

    /// True if self * other (in either order) is the identity.
    fn cancels(self, other: Step) -> bool {
        self.side == other.side && self.swap == other.swap && self.inverse != other.inverse
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Up,
    Left,
    Right,
    Down,
}

impl Dir {
    const ALL: [Dir; 4] = [Dir::Up, Dir::Left, Dir::Right, Dir::Down];

    fn opposite(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    /// Up/Down prepend to the path (Calkin-Wilf), Left/Right append (Stern-Brocot).
    fn prepends(self) -> bool {
        matches!(self, Dir::Up | Dir::Down)
    }

    /// The matrix a move uses.
    /// Horizontal moves: Left/Right use L/R. Inverse mode flips the arrows, so
    /// Left/Right use the inverse of the opposite side.
    /// Vertical moves (prepended, Calkin-Wilf) use the transpose of that matrix.
    /// Lᵀ = R and Rᵀ = L (inverses too), so plain mode flips L<->R. The swap
    /// matrices are symmetric, so swap mode keeps the letter.
    fn step(self, swap: bool, inverse: bool) -> Step {
        let horizontal = match (self, inverse) {
            (Dir::Left | Dir::Up, false) | (Dir::Right | Dir::Down, true) => Side::Left,
            _ => Side::Right,
        };
        let side = if self.prepends() && !swap {
            horizontal.flip()
        } else {
            horizontal
        };
        Step { side, swap, inverse }
    }
}

// ------------------------------------------------------------------- state

#[derive(Clone, Copy)]
enum View {
    Matrix,
    Rational,
    Decimal,
    Minkowski,
    ExplainPath,
    ExplainMatrix,
}

impl View {
    fn next(self) -> View {
        match self {
            View::Matrix => View::Rational,
            View::Rational => View::Decimal,
            View::Decimal => View::Minkowski,
	    View::Minkowski => View::ExplainPath,
            View::ExplainPath => View::ExplainMatrix,
            View::ExplainMatrix => View::Matrix,
        }
    }

    fn name(self) -> &'static str {
        match self {
            View::Matrix => "matrix",
            View::Rational => "rational",
            View::Decimal => "decimal",
            View::Minkowski => "minkowski",
            View::ExplainPath => "explain path",
            View::ExplainMatrix => "explain matrix",
        }
    }
}

struct State {
    matrix: Mat,
    path: VecDeque<Step>,
    view: View,
    swap: bool,
    inverse: bool,
}

impl State {
    fn new() -> State {
        State {
            matrix: Mat::IDENTITY,
            path: VecDeque::new(),
            view: View::Matrix,
            swap: false,
            inverse: false,
        }
    }

    /// Back to the root node. View, swap and inverse settings are kept.
    fn reset(&mut self) {
        self.matrix = Mat::IDENTITY;
        self.path.clear();
    }

    /// The matrix reached by moving in `dir`, or None on overflow.
    fn target(&self, dir: Dir) -> Option<Mat> {
        let step = dir.step(self.swap, self.inverse).matrix();
        if dir.prepends() {
            step.checked_mul(self.matrix)
        } else {
            self.matrix.checked_mul(step)
        }
    }

    fn go(&mut self, dir: Dir) {
        let Some(next) = self.target(dir) else {
            return; // would overflow: ignore the move
        };
        let step = dir.step(self.swap, self.inverse);
        self.matrix = next;

        if dir.prepends() {
            if self.path.front().is_some_and(|s| s.cancels(step)) {
                self.path.pop_front();
            } else {
                self.path.push_front(step);
            }
        } else if self.path.back().is_some_and(|s| s.cancels(step)) {
            self.path.pop_back();
        } else {
            self.path.push_back(step);
        }
    }
}

// ------------------------------------------------------- symbolic formulas

/// Coefficients of (a, b, c, d) for one matrix entry.
type Lin = [i64; 4];

/// The four entries of the neighbor in `dir`, as combinations of a, b, c, d.
fn symbolic_target(dir: Dir, swap: bool, inverse: bool) -> [Lin; 4] {
    let s = dir.step(swap, inverse).matrix();
    let (a, b, c, d): (Lin, Lin, Lin, Lin) =
        ([1, 0, 0, 0], [0, 1, 0, 0], [0, 0, 1, 0], [0, 0, 0, 1]);
    let comb = |k1: i64, x: Lin, k2: i64, y: Lin| -> Lin {
        let mut r = [0; 4];
        for i in 0..4 {
            r[i] = k1 * x[i] + k2 * y[i];
        }
        r
    };
    if dir.prepends() {
        // S * M
        [
            comb(s.a, a, s.b, c),
            comb(s.a, b, s.b, d),
            comb(s.c, a, s.d, c),
            comb(s.c, b, s.d, d),
        ]
    } else {
        // M * S
        [
            comb(s.a, a, s.c, b),
            comb(s.b, a, s.d, b),
            comb(s.a, c, s.c, d),
            comb(s.b, c, s.d, d),
        ]
    }
}

/// Formats a combination with positive terms first, e.g. "b - a", "a + c".
fn fmt_lin(l: Lin) -> String {
    const VARS: [char; 4] = ['a', 'b', 'c', 'd'];
    let mut terms: Vec<(i64, char)> = Vec::new();
    for i in 0..4 {
        if l[i] > 0 {
            terms.push((l[i], VARS[i]));
        }
    }
    for i in 0..4 {
        if l[i] < 0 {
            terms.push((l[i], VARS[i]));
        }
    }
    if terms.is_empty() {
        return "0".to_string();
    }
    let mut s = String::new();
    for (idx, (k, v)) in terms.iter().enumerate() {
        let coef = if k.abs() == 1 {
            String::new()
        } else {
            k.abs().to_string()
        };
        if idx == 0 {
            if *k < 0 {
                s.push('-');
            }
        } else if *k < 0 {
            s.push_str(" - ");
        } else {
            s.push_str(" + ");
        }
        s.push_str(&coef);
        s.push(*v);
    }
    s
}

// --------------------------------------------------------------- rendering

const OFFSET_X: usize = 4;
const OFFSET_Y: usize = 4;
const MIN_WIDTH: usize = 11;

fn center_pad(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        return s.to_string();
    }
    let left = (width - len) / 2;
    let right = width - len - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

/// (numerator, denominator) of (a+b)/(c+d), with the sign moved to the numerator.
fn frac(m: Mat) -> (i128, i128) {
    let n = m.a as i128 + m.b as i128;
    let d = m.c as i128 + m.d as i128;
    if d < 0 {
        (-n, -d)
    } else {
        (n, d)
    }
}

fn fmt_decimal(x: f64, max_len: usize) -> String {
    for prec in (0..=9).rev() {
        let s = format!("{:.*}", prec, x);
        if s.len() <= max_len {
            return s;
        }
    }
    format!("{:.3e}", x)
}

/// Width this node needs in the given view.
fn content_width(m: Mat, view: View) -> usize {
    match view {
        View::Matrix => {
            let longest = [m.a, m.b, m.c, m.d]
                .iter()
                .map(|v| v.to_string().len())
                .max()
                .unwrap_or(1);
            2 * longest + 1
        }
        View::Rational => {
            let (n, d) = frac(m);
            n.to_string().len().max(d.to_string().len()) + 2
        }
        View::Decimal | View::Minkowski | View::ExplainPath | View::ExplainMatrix => 0,
    }
}

/// Wraps a 3-line body in the box borders.
fn frame(body: Vec<String>, bold: bool, w: usize) -> Vec<String> {
    let (tl, tr, bl, br) = if bold {
        ('┏', '┓', '┗', '┛')
    } else {
        ('┌', '┐', '└', '┘')
    };
    let gap = " ".repeat(w - 2);
    let mut lines = vec![format!("{tl}{gap}{tr}")];
    lines.extend(body);
    lines.push(format!("{bl}{gap}{br}"));
    lines
}

/// Two columns of cells, used by both the matrix and explain views.
fn matrix_body(cells: [String; 4], w: usize) -> Vec<String> {
    let lw = (w - 1) / 2;
    let rw = w - 1 - lw;
    vec![
        format!("{} {}", center_pad(&cells[0], lw), center_pad(&cells[1], rw)),
        " ".repeat(w),
        format!("{} {}", center_pad(&cells[2], lw), center_pad(&cells[3], rw)),
    ]
}

/// Minkowski question mark of num/den (den > 0).
/// Closed form of the mediant recursion ?((p+r)/(q+s)) = (?(p/q) + ?(r/s)) / 2,
/// evaluated from the continued fraction of the fractional part.
fn minkowski(num: i128, den: i128) -> f64 {
    let int = num.div_euclid(den);
    let (mut p, mut q) = (den, num.rem_euclid(den)); // 1/frac = p/q
    let mut sum = 0.0;
    let mut sign = 1.0;
    let mut exp: i128 = 0;
    while q != 0 {
        let a = p / q;
        (p, q) = (q, p % q);
        exp += a;
        if exp > 1100 {
            break; // remaining terms are below f64 precision
        }
        sum += sign * 2.0 * 0.5f64.powi(exp as i32);
        sign = -sign;
    }
    int as f64 + sum
}

fn render_node(m: Option<Mat>, bold: bool, view: View, w: usize) -> Vec<String> {
    let blank = " ".repeat(w);

    let body = match (m, view) {
	(_, View::ExplainPath) => vec![blank.clone(), center_pad("M", w), blank.clone()],
        (_, View::ExplainMatrix) => matrix_body(
            ["a", "b", "c", "d"].map(String::from),
            w,
        ),
        (None, _) => vec![blank.clone(), center_pad("overflow", w), blank.clone()],
        (Some(m), View::Matrix) => matrix_body(
            [m.a, m.b, m.c, m.d].map(|v| v.to_string()),
            w,
        ),
        (Some(m), _) => {
            let (n, d) = frac(m);
            if d == 0 {
                let inf = if n < 0 { "-∞" } else { "∞" };
                vec![blank.clone(), center_pad(inf, w), blank.clone()]
            } else if let View::Decimal = view {
                let s = fmt_decimal(n as f64 / d as f64, MIN_WIDTH);
                vec![blank.clone(), center_pad(&s, w), blank.clone()]
            } else if let View::Minkowski = view {
                let s = fmt_decimal(minkowski(n, d) as f64 / d as f64, MIN_WIDTH);
                vec![blank.clone(), center_pad(&s, w), blank.clone()]
            } else if d == 1 {
                vec![blank.clone(), center_pad(&n.to_string(), w), blank.clone()]
            } else {
                vec![
                    center_pad(&n.to_string(), w),
                    format!("╶{}╴", "─".repeat(w - 2)),
                    center_pad(&d.to_string(), w),
                ]
            }
        }
    };

    frame(body, bold, w)
}

fn arrow(dir: Dir, swap: bool) -> char {
    match (dir, swap) {
        (Dir::Up, false) => '↑',
        (Dir::Down, false) => '↓',
        (Dir::Left, false) => '←',
        (Dir::Right, false) => '→',
        (Dir::Up, true) => '⇑',
        (Dir::Down, true) => '⇓',
        (Dir::Left, true) => '⇐',
        (Dir::Right, true) => '⇒',
    }
}

fn path_display(path: &VecDeque<Step>) -> String {
    let glyphs: Vec<char> = path.iter().map(|s| s.glyph()).collect();
    if glyphs.len() <= 60 {
        glyphs.into_iter().collect()
    } else {
        let head: String = glyphs[..29].iter().collect();
        let tail: String = glyphs[glyphs.len() - 30..].iter().collect();
        format!("{head}…{tail}")
    }
}

fn put(out: &mut impl Write, x: usize, y: usize, s: &str) -> io::Result<()> {
    queue!(
        out,
        MoveTo((x + OFFSET_X) as u16, (y + OFFSET_Y) as u16),
        Print(s)
    )?;
    Ok(())
}

fn draw(out: &mut Stdout, st: &State) -> io::Result<()> {
    queue!(out, Clear(ClearType::All))?;
    queue!(
        out,
        MoveTo(OFFSET_X as u16, 0),
        Print(format!(
            "View: {} | Swap: {} | Inverse: {}",
            st.view.name(),
            if st.swap { "on" } else { "off" },
            if st.inverse { "on" } else { "off" }
        ))
    )?;
    queue!(
        out,
        MoveTo(OFFSET_X as u16, 2),
        Print(format!("Path: {}", path_display(&st.path)))
    )?;

    let explain = matches!(st.view, View::ExplainPath | View::ExplainMatrix);
    let neighbors: Vec<(Dir, Option<Mat>)> =
        Dir::ALL.iter().map(|&d| (d, st.target(d))).collect();

    // All boxes share one width so the layout stays aligned.
    let w = if explain {
        MIN_WIDTH
    } else {
        std::iter::once(st.matrix)
            .chain(neighbors.iter().filter_map(|(_, m)| *m))
            .map(|m| content_width(m, st.view))
            .max()
            .unwrap_or(0)
            .max(MIN_WIDTH)
    };

    let col = w + 3;
    let node_pos = |dir: Dir| match dir {
        Dir::Up => (col, 0),
        Dir::Left => (0, 6),
        Dir::Right => (2 * col, 6),
        Dir::Down => (col, 12),
    };
    let arrow_pos = |dir: Dir| match dir {
        Dir::Up => (col + w / 2, 5),
        Dir::Down => (col + w / 2, 11),
        Dir::Left => (w + 1, 8),
        Dir::Right => (2 * w + 4, 8),
    };

    // Center node
    for (i, line) in render_node(Some(st.matrix), true, st.view, w).iter().enumerate() {
        put(out, col, 6 + i, line)?;
    }

    // Neighbors and the arrows leading to them. Inverse mode flips the arrows.
    for (dir, m) in neighbors {
        let (x, y) = node_pos(dir);
        let lines = match st.view {
            View::ExplainMatrix => {
                let cells = symbolic_target(dir, st.swap, st.inverse).map(fmt_lin);
                frame(matrix_body(cells, w), false, w)
            }
            View::ExplainPath => {
                // Up/Down multiply on the left, Left/Right on the right
                let g = dir.step(st.swap, st.inverse).glyph();
                let expr = if dir.prepends() {
                    format!("{g} × M")
                } else {
                    format!("M × {g}")
                };
                let blank = " ".repeat(w);
                frame(vec![blank.clone(), center_pad(&expr, w), blank], false, w)
            }
            _ => render_node(m, false, st.view, w),
        };
        for (i, line) in lines.iter().enumerate() {
            put(out, x, y + i, line)?;
        }
        let shown = if st.inverse { dir.opposite() } else { dir };
        let (ax, ay) = arrow_pos(dir);
        put(out, ax, ay, &arrow(shown, st.swap).to_string())?;
    }

    queue!(
        out,
        MoveTo(OFFSET_X as u16, 22),
        Print("[←↑↓→] Move | [V] View | [S] Swap | [I] Inverse | [R] Reset | [Q] Quit")
    )?;

    out.flush()
}

// -------------------------------------------------------------------- main

struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = execute!(stdout(), Show, MoveTo(0, 0), Clear(ClearType::All));
        let _ = disable_raw_mode();
    }
}

fn main() -> io::Result<()> {
    let mut out = stdout();
    enable_raw_mode()?;
    execute!(out, Hide, Clear(ClearType::All))?;
    let _guard = TerminalGuard;

    let mut st = State::new();

    loop {
        draw(&mut out, &st)?;

        if let Event::Key(key) = read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => break,
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                KeyCode::Char('v') | KeyCode::Char('V') => st.view = st.view.next(),
                KeyCode::Char('s') | KeyCode::Char('S') => st.swap = !st.swap,
                KeyCode::Char('i') | KeyCode::Char('I') => st.inverse = !st.inverse,
                KeyCode::Char('r') | KeyCode::Char('R') => st.reset(),
                KeyCode::Left => st.go(Dir::Left),
                KeyCode::Right => st.go(Dir::Right),
                KeyCode::Up => st.go(Dir::Up),
                KeyCode::Down => st.go(Dir::Down),
                _ => {}
            }
        }
    }

    Ok(())
}