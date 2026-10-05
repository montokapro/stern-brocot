# stern-brocot

Explore the Stern-Brocot tree, now extended upwards to form a graph.

## Execution

```
cargo build
target/debug/stern-brocot
```

## Navigation

Move upward left (↙) and right (↘) by pressing the arrow keys.
Move downward left (↖) and right (↗) by holding shift.
Swap between the matrix and simple presentation with space.

## The standard tree

We start with the identity matrix (I), which can be represented more simply as 1.
```
┏         ┓     ┏         ┓
  1     0  
             ≈       1
  0     1  
┗         ┛     ┗         ┛
```

By extending downwards in the tree you can enumerate any rational number with a finite number of moves.

To reach a smaller number from your current number, move left. To reach a larger number, move right.

Irrationals can be approximated at increasing fidelity via their continued fraction approximations.

To approximate the golden ratio, zigzag downwards: "↘↙↘↙..."
To approximate the golden ratio, zigzag downwards in doubles: "↘ ↙↙ ↘↘ ↙↙ ↘↘..."
Other approximations like e are more complicated: "↘↘  ↙ ↘↘ ↙ ↘ ↙↙↙↙ ↘ ↙ ↘↘↘↘↘↘ ↙ ↘..."

## The extended graph

By extending the tree upward, we can reach zero, negative numbers, and even terms where we divide by zero (labeled "∞" and "-∞").

Note that the simple representations are not unique. Here is the negative identity matrix (-I). 
```
┏         ┓     ┏         ┓
 -1     0  
             ≈       1
  0    -1  
┗         ┛     ┗         ┛
```
To get to the negative representations, follow the path "↗↘↗↘". To get back, you can follow "↗↘↗↘" again.

Even more representations exist for the same simple representations! See if you can reach one using a combination of upward and downward movements.
```
┏         ┓     ┏         ┓
 -2     1  
             ≈       1
 -1     0  
┗         ┛     ┗         ┛
```

# Modes

## Matrix

The matrix representation looks like this.
```
    ┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
      1    -2       2    -1                     1    -1       1     0  
                                                                       
      0     1      -1     1                    -1     2      -2     1  
    └         ┘   └         ┘                 └         ┘   └         ┘
                ↖      ↗                           ↖      ↗
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
                ↙      ↘                           ↙      ↘
    ┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
      1     0       1     1                     2     1       1     2  
                                                                       
      2     1       1     2                     1     1       0     1  
    └         ┘   └         ┘                 └         ┘   └         ┘

    Navigation: [Left/Right] Downward | [Shift+Left/Right] Upward | [Space] Toggle View | [Q/Esc] Quit
```

Moving down-left or up-right updates the left column.
Moving down-right or up-left updates the right column.

You can give the matrix a standard representation by adding the top row and bottom row
```
┏         ┓     ┏         ┓
  1     1            2
             ≈  -----------
  1     2            3
┗         ┛     ┗         ┛
```

## Simple

The simple view lets you view the standard way we represent numbers. However, some fidelity is lost in this view.
```
    ┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
                                                                       
        -1             ∞                           0            -1     
                                                                       
    └         ┘   └         ┘                 └         ┘   └         ┘
                ↖      ↗                           ↖      ↗
    ┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
        -1                                                             
    ----------- ↙      0                           ∞      ↘     -2     
         2                                                             
    └         ┘   └         ┘                 └         ┘   └         ┘
                              ↖             ↗
                                ┏         ┓
                                           
                                     1     
                                           
                                ┗         ┛
                              ↙             ↘
    ┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
                       1                                               
         0      ↖ -----------                      2      ↗      ∞     
                       2                                               
    └         ┘   └         ┘                 └         ┘   └         ┘
                ↙      ↘                           ↙      ↘
    ┌         ┐   ┌         ┐                 ┌         ┐   ┌         ┐
         1             2                           3                   
    -----------   -----------                 -----------        3     
         3             3                           2                   
    └         ┘   └         ┘                 └         ┘   └         ┘

    Navigation: [Left/Right] Downward | [Shift+Left/Right] Upward | [Space] Toggle View | [Q/Esc] Quit
```

## References

Many wonderful visualizations and explanations of the stern-brocot trees already exist. Here are a few.

https://en.wikipedia.org/wiki/Stern%E2%80%93Brocot_tree
https://filipjaniszewski.com/2016/12/11/stern-brocot-trees/
https://youtu.be/qPeD87HJ0UA?si=Y5mMkz-K_eETFg_F

I hope to fill a niche such that expanding the tree upwards is easier to explore!
