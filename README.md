# stern-brocot

Explore the Stern–Brocot, Calkin–Wilf, and related trees as a graph. Inverse moves extend the graph past the root to zero, negative numbers and ∞.

## Execution

You need Rust 1.70+ and a robust terminal font.

```
cargo run
```

## Controls

- **← →**: Move left or right. Multiplies on the right (Stern–Brocot) and appends to the path.
- **↑ ↓**: Move up or down. Multiplies on the left (Calkin–Wilf) and prepends to the path.
- **V**: Cycle the view: matrix → rational → decimal → explain path → explain matrix.
- **S**: Toggle swap mode.
- **I**: Toggle inverse mode.
- **R**: Reset to the identity matrix. Keeps the view, swap and inverse settings.
- **Q** / **Esc**: Quit.

The app starts in matrix view with swap and inverse off.

```
    View: matrix | Swap: off | Inverse: off

    Path:

                  ┌         ┐
                    1     1  
                             
                    0     1  
                  └         ┘
                       ↑
    ┌         ┐   ┏         ┓   ┌         ┐
      1     0       1     0       1     1  
                ←             →
      1     1       0     1       0     1  
    └         ┘   ┗         ┛   └         ┘
                       ↓
                  ┌         ┐
                    1     0  
                             
                    1     1  
                  └         ┘

    [←↑↓→] Move | [V] View | [S] Swap | [I] Inverse | [R] Reset | [Q] Quit
```

The bold box in the middle is the current node. The four boxes around it show where each arrow key takes you. The path above lists the moves that produced the current matrix.

## How it works

A node in the graph is defined by its current path from the root to that node. It has many representations, such as a matrix or a rational. The columns are the rationals `a/c` and `b/d`, and the value is their mediant which lies between the two rationals.

```
┏         ┓   ┏         ┓
  a     b        a + b
            ≈ ╶─────────╴
  c     d        c + d
┗         ┛   ┗         ┛
```

The root is the identity matrix or rational `1/1`, with bounds `1/0` and `0/1`.

```
┏         ┓     ┏         ┓
  1     0  
             ≈       1
  0     1  
┗         ┛     ┗         ┛
```

- **←**: `M × L`. The left column becomes the sum of the two columns.
- **→**: `M × R`. The right column becomes the sum of the two columns.
- **↑**: `R × M`. The top row becomes the sum of the two rows.
- **↓**: `L × M`. The bottom row becomes the sum of the two rows.

Left and right traverse the Stern–Brocot tree. Up and down traverse the Calkin–Wilf tree. From a positive node, right and up lead to a larger number, and left and down lead to a smaller one.

### The path

The path shows the directions taken in order. Left and right append a direction, and up and down prepend a direction. An inverse move undoes the direction at the end it touches and shortens the path instead of growing it.

- **Plain:** `L` ◣, `R` ◥, `L⁻¹` ◺, `R⁻¹` ◹
- **Swap:** `Ls` ◤, `Rs` ◢, `Ls⁻¹` ◸, `Rs⁻¹` ◿

## Modes

### Swap (S)

Before replacing, swap the two columns (← →) or the two rows (↑ ↓). The arrows become double arrows. Swapping changes the sign of the determinant, so orientation alternates with every move. At the root, ← now leads to 2 and → leads to 1/2, the reverse of plain mode.

### Inverse (I)

All arrows are flipped to point back toward the center, and each move multiplies by an inverse matrix. A move in inverse mode is undone by the opposite arrow key in the matching normal mode. For example, ← in inverse mode is undone by → in normal mode.

### Which matrix each key uses

Each mode has its own pair of matrices. The glyph under a matrix is what the path shows when you use it.

**Plain**
```
     L                 R
┌         ┐       ┌         ┐
  1     0           1     1

  1     1           0     1
└         ┘       └         ┘
     ◣                 ◥
```
← `L`, → `R`, ↑ `R`, ↓ `L`

**Inverse**
```
    L⁻¹               R⁻¹
┌         ┐       ┌         ┐
  1     0           1    -1

 -1     1           0     1
└         ┘       └         ┘
     ◺                 ◹
```
← `R⁻¹`, → `L⁻¹`, ↑ `L⁻¹`, ↓ `R⁻¹`

**Swap**
```
    Ls                Rs
┌         ┐       ┌         ┐
  1     1           0     1

  1     0           1     1
└         ┘       └         ┘
     ◤                 ◢
```
← `Ls`, → `Rs`, ↑ `Ls`, ↓ `Rs`

**Swap + inverse**
```
   Ls⁻¹              Rs⁻¹
┌         ┐       ┌         ┐
  0     1          -1     1

  1    -1           1     0
└         ┘       └         ┘
     ◸                 ◿
```
← `Rs⁻¹`, → `Ls⁻¹`, ↑ `Rs⁻¹`, ↓ `Ls⁻¹`

The swap matrices are symmetric, so their transposes are themselves. In all other cases the left and right matrices are transposes of each other.

## Views

### Matrix

The raw matrix.
```
┏         ┓
  3     2  
           
  1     1  
┗         ┛
```

### Rational

The value `(a + b) / (c + d)` as a fraction. Some information is lost here, because many matrices have the same value.
```
┏         ┓
     5
╶─────────╴
     2
┗         ┛
```

### Decimal

The value rounded to nine places. Large values lose digits to fit the box and fall back to scientific notation. Division by zero shows as `∞` or `-∞`.
```
┏         ┓

2.500000000

┗         ┛
```

### Minkowski

The value passed through the Minkowski question mark function `?(x)`. It is defined by `?(0) = 0`, `?(1) = 1` and `?(mediant) = (?(left) + ?(right)) / 2`. Rationals map to dyadic rationals, and the function is extended with `?(x + 1) = ?(x) + 1`.

Here is the node reached by pressing ←, whose value is 1/2:
```
    View: minkowski | Swap: off | Inverse: off

    Path: ◣

                  ┌         ┐

                  1.500000000

                  └         ┘
                       ↑
    ┌         ┐   ┏         ┓   ┌         ┐

    0.250000000 ← 0.500000000 → 0.750000000

    └         ┘   ┗         ┛   └         ┘
                       ↓
                  ┌         ┐

                  0.250000000

                  └         ┘

    [←↑↓→] Move | [V] View | [S] Swap | [I] Inverse | [R] Reset | [Q] Quit
```

Left and down both reach 1/3, which becomes 0.25. Right reaches 2/3, which becomes 0.75. Up reaches 3/2, which becomes `1 + ?(1/2) = 1.5`.

### Explain path

A static view that shows how each neighbor is reached from the current matrix `M`, using the path symbols. It respects the swap and inverse toggles.
```
    View: explain path | Swap: off | Inverse: off

    Path:

                  ┌         ┐
                             
                     ◥ × M   
                             
                  └         ┘
                       ↑
    ┌         ┐   ┏         ┓   ┌         ┐
                                           
       M × ◣    ←      M      →    M × ◥   
                                           
    └         ┘   ┗         ┛   └         ┘
                       ↓
                  ┌         ┐
                             
                     ◣ × M   
                             
                  └         ┘

    [←↑↓→] Move | [V] View | [S] Swap | [I] Inverse | [R] Reset | [Q] Quit
```
Left and right append the path, and up and down prepend the path. In plain mode, up uses ◥ and down uses ◣.

### Explain matrix

A static view that shows how each neighbor is computed from the current `a`, `b`, `c` and `d`. It respects the swap and inverse toggles.
```
    View: explain matrix | Swap: off | Inverse: off

    Path:

                  ┌         ┐
                  a + c b + d
                             
                    c     d  
                  └         ┘
                       ↑
    ┌         ┐   ┏         ┓   ┌         ┐
    a + b   b       a     b       a   a + b
                ←             →
    c + d   d       c     d       c   c + d
    └         ┘   ┗         ┛   └         ┘
                       ↓
                  ┌         ┐
                    a     b  
                             
                  a + c b + d
                  └         ┘

    [←↑↓→] Move | [V] View | [S] Swap | [I] Inverse | [R] Reset | [Q] Quit
```

## The standard tree

Stay in plain mode and use only ← and →. Every positive rational is reachable with finitely many moves. To reach a smaller number from your current one, move left. To reach a larger one, move right.

Irrationals can be approximated at increasing fidelity via their continued fractions.

- The golden ratio zigzags: `◥◣◥◣...`. The values are 2, 3/2, 5/3, 8/5, ...
- √2 zigzags in doubles: `◥ ◣◣ ◥◥ ◣◣ ◥◥...`. The values are 2, 3/2, 4/3, 7/5, 10/7, 17/12, ...
- e is more complicated: `◥◥ ◣ ◥◥ ◣ ◥ ◣◣◣◣ ◥ ◣ ◥◥◥◥◥◥ ◣ ◥...`, which follows its continued fraction `[2; 1, 2, 1, 1, 4, 1, 1, 6, ...]`.

Try a few different ways of approaching the golden ratio! 

## The extended graph

Inverse mode lets you walk past the root. From the root, press `I` and then:

- ← lands on `R⁻¹`, whose value is 0.
- → lands on `L⁻¹`, where we divide by zero, so the value is ∞.

```
    View: rational | Swap: off | Inverse: on

    Path:

                  ┌         ┐
                             
                       ∞     
                             
                  └         ┘
                       ↓
    ┌         ┐   ┏         ┓   ┌         ┐
                                           
         0      →      1      ←      ∞
                                           
    └         ┘   ┗         ┛   └         ┘
                       ↑
                  ┌         ┐
                             
                       0     
                             
                  └         ┘

    [←↑↓→] Move | [V] View | [S] Swap | [I] Inverse | [R] Reset | [Q] Quit
```

Negative numbers are reachable too. From the root, press ← twice in inverse mode to reach `[[1, -2], [0, 1]]`, which is -1.

Note that the rational representation is not unique. Here is the negative identity matrix (-I).
```
┏         ┓     ┏         ┓
 -1     0  
             ≈       1
  0    -1  
┗         ┛     ┗         ┛
```

To reach it, follow one of these paths:
- `◹◣◹◣◹◣`
- `◥◺◥◺◥◺`
- `◺◥◺◥◺◥`
- `◣◹◣◹◣◹`
- `◿◤◿◤◿◤`
- `◢◸◢◸◢◸`
- `◤◿◤◿◤◿`
- `◸◢◸◢◸◢`

Following the same path again gets you back to the identity. Try following the path to see how it rotates through zeros, infinities, and negatives.
Even more matrices share the same rational! See if you can reach this one using a combination of moves.

```
┏         ┓     ┏         ┓
 -2     1  
             ≈       1
 -1     0  
┗         ┛     ┗         ┛
```

## Limits

Entries are 64-bit integers. A move that would overflow is ignored, and that neighbor's box shows `overflow`.

## References

Many wonderful visualizations and explanations of the Stern-Brocot tree already exist. Here are a few.

- https://en.wikipedia.org/wiki/Stern%E2%80%93Brocot_tree
- https://en.wikipedia.org/wiki/Calkin%E2%80%93Wilf_tree
- https://filipjaniszewski.com/2016/12/11/stern-brocot-trees/
- https://youtu.be/qPeD87HJ0UA?si=Y5mMkz-K_eETFg_F
- Bates, Bunder & Tognetti, "Linking the Calkin-Wilf and Stern-Brocot trees", European Journal of Combinatorics 31(7), 2010.

I hope to fill a niche such that expanding the tree upwards is easier to explore!