# Day 9
## Input
You pull out your handy Oasis And Sand Instability Sensor and analyze your surroundings. The OASIS produces a report of many values and how they are changing over time (your puzzle input). Each line in the report contains the history of a single value
```
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
```
In the above dataset, the first history is `0 3 6 9 12 15`. 

Because the values increase by `3` each step, the first sequence of differences that you generate will be `3 3 3 3 3`. Note that this sequence has one fewer value than the input sequence because at each step it considers two numbers from the input.

Since these values aren't all zero, repeat the process: the values differ by `0` at each step, so the next sequence is `0 0 0 0`. This means you have enough information to extrapolate the history!

Visually, these sequences can be arranged like this:
```
0   3   6   9  12  15
  3   3   3   3   3
    0   0   0   0
```
To extrapolate, start by adding a new zero to the end of your list of zeroes; because the zeroes represent differences between the two values above them, this also means there is now a placeholder in every sequence above it:
```
0   3   6   9  12  15   B
  3   3   3   3   3   A
    0   0   0   0   0
```
You can then start filling in placeholders from the bottom up. A needs to be the result of increasing `3` (the value to its left) by `0` (the value below it); this means `A` must be `3`:
```
0   3   6   9  12  15   B
  3   3   3   3   3   3
    0   0   0   0   0
```
Finally, you can fill in `B`, which needs to be the result of increasing `15` (the value to its left) by `3` (the value below it), or `18`:
```
0   3   6   9  12  15  18
  3   3   3   3   3   3
    0   0   0   0   0
```
So, the next value of the first history is `18`

## Part 1
Analyze your OASIS report and extrapolate the next value for each history. What is the sum of these extrapolated values?
```
History: [0, 3, 6, 9, 12, 15] } -> Some(18)
History: [1, 3, 6, 10, 15, 21] } -> Some(28)
History: [10, 13, 16, 21, 30, 45] } -> Some(68)

Sum of forward predictions = 114
```
## Part 2
Surely it's safe to just extrapolate backwards as well, right?

OASIS report again, this time extrapolating the previous value for each history. What is the sum of these extrapolated values?
```
History: [15, 12, 9, 6, 3, 0] } -> -3
History: [21, 15, 10, 6, 3, 1] } -> 0
History: [45, 30, 21, 16, 13, 10] } -> 5

Sum of backward predictions = 2
```
## Approach
We reduce the input recursively until the resulting array is full of zeros. Once the condition is reached the recursion unwinds by returning the SUM of last numbers in last + current sequence, i.e. `SUM(.. SUM( SUM( SUM(last + current) + current ) + current ) .. + current)`
```
10  13  16  21  30  45 --reduce(hist)           -> 23+45=68
   3   3   5   9  15    --reduce(hist)        -> 8+15=23
     0   2   4   6       --reduce(hist)     -> 2+6=8
       2   2   2          --reduce(hist)  -> 2
         0   0            * recursion unwind *   
```

```rust
fn predict_next(history: &[i32]) -> i32 {
    
    let reduced = history.windows(2).map(|a| a[1]-a[0]).collect::<Vec<_>>();
    
    if reduced.iter().all(|d| d.eq(&0)) {
        return history[0];
    }
    
    Self::predict_next(&reduced) + history[reduced.len()]
}
```
