Write me a terminal application in rust that renders 2 by 2 matricies.

Let the user navigate the stern-brocot tree downward using the left and right arrows.
The user should be able to move left and right upward on the tree by holding shift.
The user should be able to swap between the matrix and simple presentation with space.

# Presentation

## Numbers

Numbers should be centered if possible.

Here are examples of centering in 5 width slot (aaaaa):
"  1  "
" -2  "
" 345 "
"6789 "

Here are examples of centering in 11 width slot (aaaaaaaaaa):
"     1     "
"    -2     "
"    345    "
"   6789    "

Infinity should be represented by "∞" (a/0) or "-∞" (-a/0).

## Boxes

Boxes should display by default in matrix mode. Hit space to switch between matrix mode and simple mode.

## Matrix

Matricies should be displayed with its four values written in four quadrants.

```
┌         ┐
aaaaa bbbbb

ccccc ddddd
└         ┘
```

## Simple

Outside of matrix mode, render as fraction, integer, or infinity sign as appropriate. The value to display is (a + b) / (c + d) where those values are extracted from the matrix representation..
```
┌         ┐
aaaaaaaaaaa
-----------
bbbbbbbbbbb
└         ┘
```

Integer
```
┌         ┐

aaaaaaaaaaa

└         ┘
```

## Screen

Use the following math to update the screen when an arrow (or shift arrow) is pressed.
```
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
  a   b-2a    2a-b   b-a                   a-b  2b-a    a-2b    b

  c   d-2c    2c-d   d-c                   c-d  2d-c    b-2d    d
└         ┘   └         ┘                 └         ┘   └         ┘
            ↖     ↗                            ↖      ↗
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
  b    b-a      a    b-a                   a-b    b      a-b    a
            ↙                                         ↘
  c    d-c      c    d-c                   c-d    d      c-d    c
└         ┘   └         ┘                 └         ┘   └         ┘
                          ↖             ↗
                            ┏         ┓
                              a     b

                              c     d
                            ┗         ┛
                          ↙             ↘
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
 a+b   -a      a+b    b                     a    a+b     -b    a+b
            ↖                                         ↗
 c+d   -c      c+d    d                     c    c+d     -d    c+d
└         ┘   └         ┘                 └         ┘   └         ┘
            ↙      ↘                          ↙       ↘
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
 a+2b   b      a+b  a+2b                   2a+b  a+b     a+b  2a+d

 c+2d   d      c+d  c+2d                   2c+d  c+d     c+d  2c+d
└         ┘   └         ┘                 └         ┘   └         ┘
```

We start with the identity matrix in the middle, so it should look like this.
```
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
  1    -2       2    -1                     1     1       1     0

  0     1       1     1                    -1     2      -2     1
└         ┘   └         ┘                 └         ┘   └         ┘
            ↖     ↗                            ↖      ↗
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
  0    -1       1    -1                     1     0       1     1
            ↙                                         ↘
  1     1       0     1                    -1     1      -1     0
└         ┘   └         ┘                 └         ┘   └         ┘
                          ↖             ↗
                            ┏         ┓
                              1     0

                              0     1
                            ┗         ┛
                          ↙             ↘
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
  1    -1       1     0                     1     1       0     1
            ↖                                         ↗
  1     0       1     1                     0     1      -1     1
└         ┘   └         ┘                 └         ┘   └         ┘
            ↙      ↘                          ↙       ↘
┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
  1     0       1     1                     2     1       1     2

  2     1       1     2                     1     1       0     1
└         ┘   └         ┘                 └         ┘   └         ┘
```

When an arrow key is pressed, the screen should update such that the center matrix is now what the central arrow pointed to.
All other matricies should update in accordance.
