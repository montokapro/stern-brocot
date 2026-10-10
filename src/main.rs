use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use std::io::{stdout, Write};

#[derive(Clone, Copy)]
struct Mat {
    a: i64,
    b: i64,
    c: i64,
    d: i64,
}

impl Mat {
    fn mul(self, other: Mat) -> Mat {
        Mat {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
        }
    }
}

// Tree navigation matrices
const L: Mat = Mat { a: 1, b: 0, c: 1, d: 1 };
const R: Mat = Mat { a: 1, b: 1, c: 0, d: 1 };
const L_INV: Mat = Mat { a: 1, b: 0, c: -1, d: 1 };
const R_INV: Mat = Mat { a: 1, b: -1, c: 0, d: 1 };

#[derive(Clone, Copy)]
enum Mode {
    Simple,
    Matrix,
    Decimal
}

fn center_pad(s: &str, width: usize) -> String {
    let len = s.chars().count();
    if len >= width {
        format!("{:width$}", s)
    } else {
        let left = (width - len) / 2;
        let right = width - len - left;
        format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
    }
}

fn format_matrix(m: Mat, is_center: bool, mode: Mode) -> Vec<String> {
    let top_border = if is_center { "┏         ┓" } else { "┌         ┐" };
    let bot_border = if is_center { "┗         ┛" } else { "└         ┘" };

    let mut lines = vec![top_border.to_string()];

    match mode {
        Mode::Simple => {
            let num = m.a + m.b;
            let den = m.c + m.d;

            // Normalize fraction signs
            let (disp_num, disp_den) = if den < 0 {
                (-num, -den)
            } else {
                (num, den)
            };

            if disp_den == 0 {
                let inf = if disp_num < 0 { "-∞" } else { "∞" };
                lines.push("           ".to_string());
                lines.push(center_pad(inf, 11));
                lines.push("           ".to_string());
            } else if disp_den == 1 {
                lines.push("           ".to_string());
                lines.push(center_pad(&disp_num.to_string(), 11));
                lines.push("           ".to_string());
            } else {
                lines.push(center_pad(&disp_num.to_string(), 11));
                lines.push("-----------".to_string());
                lines.push(center_pad(&disp_den.to_string(), 11));
            }
        }
        Mode::Matrix => {
            let a = center_pad(&m.a.to_string(), 5);
            let b = center_pad(&m.b.to_string(), 5);
            let c = center_pad(&m.c.to_string(), 5);
            let d = center_pad(&m.d.to_string(), 5);
            lines.push(format!("{} {}", a, b));
            lines.push("           ".to_string());
            lines.push(format!("{} {}", c, d));
        }
        Mode::Decimal => {
            let num = m.a + m.b;
            let den = m.c + m.d;
            let float = num as f32 / den as f32;

            if den == 0 {
                let inf = if num < 0 { "-∞" } else { "∞" };
                lines.push("           ".to_string());
                lines.push(center_pad(inf, 11));
                lines.push("           ".to_string());
            } else {
                lines.push("           ".to_string());
                lines.push(center_pad(&float.to_string(), 11));
                lines.push("           ".to_string());
            }
        }
    }

    lines.push(bot_border.to_string());
    lines
}

fn main() -> std::io::Result<()> {
    let mut stdout = stdout();
    enable_raw_mode()?;
    execute!(stdout, Hide, Clear(ClearType::All))?;

    let mut current_matrix = Mat { a: 1, b: 0, c: 0, d: 1 };
    let mut mode = Mode::Simple;
    let mut path: Vec<char> = Vec::new();

    const OFFSET_X: u16 = 4;
    const OFFSET_Y: u16 = 3;

    loop {
        execute!(stdout, Clear(ClearType::All))?;

        let path_str: String = path.iter().collect();
        execute!(
            stdout,
            MoveTo(OFFSET_X, 1),
            Print(format!("Path: {}", path_str))
        )?;

        let m = current_matrix;
        let nodes = vec![
            // Row 1
            (0, 0, m.mul(R_INV).mul(R_INV), false),
            (14, 0, m.mul(R_INV).mul(L_INV), false),
            (42, 0, m.mul(L_INV).mul(R_INV), false),
            (56, 0, m.mul(L_INV).mul(L_INV), false),
            // Row 2
            (0, 6, m.mul(R_INV).mul(L), false),
            (14, 6, m.mul(R_INV), false),
            (42, 6, m.mul(L_INV), false),
            (56, 6, m.mul(L_INV).mul(R), false),
            // Row 3 (Center Focus)
            (28, 12, m, true),
            // Row 4
            (0, 18, m.mul(L).mul(R_INV), false),
            (14, 18, m.mul(L), false),
            (42, 18, m.mul(R), false),
            (56, 18, m.mul(R).mul(L_INV), false),
            // Row 5
            (0, 24, m.mul(L).mul(L), false),
            (14, 24, m.mul(L).mul(R), false),
            (42, 24, m.mul(R).mul(L), false),
            (56, 24, m.mul(R).mul(R), false),
        ];

        // Arrow structural wiring
        let arrows = vec![
            // Center
            ('↖', (2 * 14) - 2, 11),
            ('↗', (3 * 14) - 2, 11),
            ('↙', (2 * 14) - 2, 17),
            ('↘', (3 * 14) - 2, 17),
            // Top left
            ('↖', (1 * 14) - 2, 5),
            ('↗', (1 * 14) + 5, 5),
            ('↙', (1 * 14) - 2, 8),
            // Top right
            ('↖', (4 * 14) - 9, 5),
            ('↗', (4 * 14) - 2, 5),
            ('↘', (4 * 14) - 2, 8),
            // Bottom left
            ('↙', (1 * 14) - 2, 23),
            ('↘', (1 * 14) + 5, 23),
            ('↖', (1 * 14) - 2, 20),
            // Bottom right
            ('↙', (4 * 14) - 9, 23),
            ('↘', (4 * 14) - 2, 23),
            ('↗', (4 * 14) - 2, 20)
        ];

        // Draw Nodes
        for (x, y, mat, is_center) in nodes {
            let lines = format_matrix(mat, is_center, mode);
            for (i, line) in lines.iter().enumerate() {
                execute!(
                    stdout,
                    MoveTo(x + OFFSET_X, y + OFFSET_Y + i as u16),
                    Print(line)
                )?;
            }
        }

        // Draw Structural Arrows
        for (ch, x, y) in arrows {
            execute!(stdout, MoveTo(x + OFFSET_X, y + OFFSET_Y), Print(ch))?;
        }

        execute!(
            stdout,
            MoveTo(OFFSET_X, 33),
            Print("Navigation: [Left/Right] Downward | [Shift+Left/Right] Upward | [Space] Toggle View | [Q/Esc] Quit")
        )?;

        stdout.flush()?;

        // Input Processing Loop
        if let Event::Key(key) = read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => {
                        mode = match mode {
                            Mode::Simple => Mode::Matrix,
                            Mode::Matrix => Mode::Decimal,
                            Mode::Decimal => Mode::Simple
                        }
                    }
                    KeyCode::Left => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            current_matrix = current_matrix.mul(R_INV); // Up-Left Parent
                            path.push('↖');
                        } else {
                            current_matrix = current_matrix.mul(L); // Down-Left Child
                            path.push('↙');
                        }
                    }
                    KeyCode::Right => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            current_matrix = current_matrix.mul(L_INV); // Up-Right Parent
                            path.push('↗');
                        } else {
                            current_matrix = current_matrix.mul(R); // Down-Right Child
                            path.push('↘');
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    execute!(stdout, Show)?;
    disable_raw_mode()?;
    Ok(())
}