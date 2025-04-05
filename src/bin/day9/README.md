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

## Solution Approach

### Intuition
This problem involves pattern recognition in sequences. The key insight is that by repeatedly taking differences between consecutive values, we eventually reach a sequence of zeros, indicating a stable pattern. Once we have this "base case", we can extrapolate new values by working backward through our recursion.

### Step 1: Basic Data Structure
First, we need to represent our sequences. We use a `Sequence` struct that wraps a history of numbers:

```rust
pub(crate) type Number = i32;

#[derive(Debug, PartialEq)]
pub(crate) struct Sequence {
    pub(crate) history: Rc<[Number]>
}
```

Using `Rc<[Number]>` (a reference-counted array) allows us to efficiently share the sequence data without unnecessary copying.

### Step 2: Parsing Input
We implement `FromStr` for `Sequence` to parse each line of the input:

```rust
impl FromStr for Sequence {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split_ascii_whitespace()
            .map(|s| s.parse::<Number>())
            .collect::<Result<Rc<[_]>,_>>()
        {
            Ok(history) => Ok(Sequence { history }),
            Err(e) => Err(e),
        }
    }
}
```

This approach handles parsing errors gracefully, allowing us to identify malformed input.

### Step 3: Sequence Reduction
The core algorithm involves reducing a sequence to its differences, which we implement with a helper function:

```rust
fn reduce_level(
    vec: &[Number],
    pair_calc: fn(&[Number]) -> Number
) -> Rc<[Number]>
{
    vec
        .windows(2)
        .map(pair_calc)
        .collect::<Rc<[Number]>>()
}
```

Here, `windows(2)` gives us adjacent pairs of elements, and we apply a calculation function to each pair. This abstraction handles both forward and backward predictions with the same code structure.

### Step 4: Recursive Forward Prediction
For part 1, we implement a recursive algorithm to predict the next value:

```rust
fn predict_next(history: &[Number]) -> Number {
    let reduced = reduce_level(history, |a| a[1]-a[0]);
    if reduced.iter().all(|d| 0.eq(d)) {
        history[0]
    } else {
        Self::predict_next(&reduced) + history[reduced.len()]
    }
}
```

The algorithm works by:
1. Computing the differences between consecutive elements
2. If all differences are zero, we've found our base case
3. Otherwise, recursively compute the next value of the reduced sequence
4. Add it to the last value of the current sequence to get the prediction

### Step 5: Backward Prediction
For part 2, we need to predict values before the start of the sequence. The approach is similar but with adjusted logic:

```rust
fn predict_bwd(history: &[Number]) -> Number {
    let reduced = reduce_level(history, |a| a[0]-a[1]);
    if reduced.iter().all(|d| 0.eq(d)) {
        history[0]
    } else {
        history[reduced.len()] - Self::predict_bwd(&reduced)
    }
}
```

Key differences:
1. We reverse the order of subtraction (`a[0]-a[1]` instead of `a[1]-a[0]`)
2. We subtract the recursive result from the last value instead of adding

### Step 6: Iterator Implementation
We wrap our prediction algorithms in iterators for a clean API:

```rust
pub(crate) struct FwdIterator {
    seq: Vec<Number>
}

impl Iterator for FwdIterator {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let p = FwdIterator::predict_next(&self.seq);
        self.seq.push(p);
        Some(p)
    }
}
```

The backward iterator works similarly, but first reverses the sequence and uses the backward prediction algorithm.

### Step 7: Putting It All Together
Our main function reads the input, parses sequences, and computes the sum of predictions:

```rust
let mut seqs = input
    .lines()
    .map(|line| line
        .parse::<Sequence>()
        .unwrap_or_else(|e| panic!("Ops! {} -> {:?}",e, line))
    )
    .collect::<Vec<_>>();

let sum = seqs
    .iter_mut()
    .map(|seq| seq.iter_forward().next().unwrap())
    .sum::<Number>();
```

## Educational Insights

1. **Separation of Concerns**: Our solution separates data structure (`Sequence`), algorithm (`predict_next`/`predict_bwd`), and interface (`Iterator`) into distinct components.

2. **Generic Algorithms**: By parameterizing `reduce_level` with a calculation function, we reuse code between forward and backward prediction.

3. **Error Handling**: Our parsing code handles various error cases, making the solution robust against malformed input.

4. **Recursive Problem Solving**: This problem demonstrates the power of recursive thinking - by breaking down the problem into simpler versions of itself until we reach a base case.

5. **Iterator Pattern**: By implementing iterators, we create a composable, expressive API that follows Rust idioms and allows for future extensibility.
