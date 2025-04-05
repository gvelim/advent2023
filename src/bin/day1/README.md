# Day 1: Digit Extraction Challenge

## Problem Description

We're given lines of text where each line contains digits, either as literal numeric characters ("1", "2", etc.) or as spelled-out words ("one", "two", etc.). We need to extract the first and last digits from each line to form a two-digit number. If there's only one digit in a line, it's used as both the first and last digit.

### Sample Input:
```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixtee
```

### Expected Output:
```
29
83
13
24
42
14
76
```

Note the special case: Words can overlap. For example, in `threeightwo`, we identify "three", "eight", and "two", resulting in 32.

## Solution Design

### Intuition

To solve this problem, we need to:
1. Parse each line of input and identify all occurrences of digits.
2. Extract the first and last digit to form a two-digit number.
3. Sum all these two-digit numbers to get the final result.

For part two, we encounter an additional challenge: recognizing spelled-out digits like "one", "two", etc.

### Step 1: Define an Interface for Parsing

Let's start by defining a trait that provides a consistent interface for our parsers:

```rust
trait Parse {
    fn parser<'a>(&self, inp: &'a str) -> impl Iterator<Item = u32> + 'a;
}
```

This abstraction allows us to implement different parsing strategies while maintaining a consistent interface - a classic application of the Strategy pattern.

### Step 2: Implement Part 1 (Numeric Digits Only)

For Part 1, we only need to consider numeric digits:

```rust
struct ParserDigits;
impl Parse for ParserDigits {
    fn parser<'a>(&self, inp: &'a str) -> impl Iterator<Item = u32> + 'a {
        inp.chars()
            .filter(|c| c.is_ascii_digit())
            .map(|c| (c as u8 - b'0') as u32)
    }
}
```

This implementation:
1. Iterates through each character
2. Filters out non-digit characters
3. Converts ASCII digits to numerical values

### Step 3: Sum Up the Calibration Values

Now we can create a function that takes a parsing strategy and computes the sum:

```rust
fn sum_up(inp: &str, p: impl Parse) -> u32 {
    inp.lines()
        .filter_map(|line| {
            let mut iter = p.parser(line);
            iter.next().map(|f| 10*f + iter.last().unwrap_or(f))
        })
        .sum::<u32>()
}
```

This function:
1. Processes each line of input
2. Uses our parser to extract digits
3. Takes the first digit and multiplies it by 10 (to make it the tens place)
4. Adds the last digit (or the first digit again if there's only one)
5. Sums all the resulting values

### Step 4: Implement Part 2 (Words and Digits)

For Part 2, we need to recognize both numeric digits and spelled-out digits:

```rust
struct ParserNumerics;
impl Parse for ParserNumerics {
    fn parser<'a>(&self, input: &'a str) -> impl Iterator<Item = u32> + 'a {
        static DIGITS: [(&str, u32); 9] = [
            ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5),
            ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)
        ];

        let mut buf = String::with_capacity(60);
        input.chars()
            .filter_map(move |c| {
                match c {
                    '0'..='9' => Some((c as u8 - b'0') as u32),
                    'a'..='z' => {
                        buf.push(c);
                        DIGITS.iter()
                            .filter_map(|(d, numeric)|
                                if !buf.ends_with(d) { None } else { Some(*numeric) }
                            )
                            .next()
                    },
                    _ => None
                }
            })
    }
}
```

This implementation:
1. Creates a lookup table for spelled-out digits
2. Maintains a buffer that accumulates characters
3. For each character:
   - If it's a numeric digit, converts it directly
   - If it's a letter, adds it to the buffer and checks if any spelled-out digit ends at this point
4. Returns the digit if found, otherwise None

The key insight here is how we handle overlapping words by checking if any digit word ends with the current buffer, rather than checking if the buffer equals a digit word. This allows us to detect things like "oneight" as both "one" and "eight".

### Step 5: Running It All Together

Finally, we can run both parts:

```rust
fn main() {
    let inp = std::fs::read_to_string("src/bin/day1/input.txt")
        .unwrap_or_else(|e| panic!("{e}"));

    let t = Instant::now();
    println!("Part 1 -> Sum = {:?} - {:?}",
             sum_up(&inp, ParserDigits), t.elapsed());

    let t = Instant::now();
    println!("Part 2 -> Sum = {:?} - {:?}",
             sum_up(&inp, ParserNumerics), t.elapsed());
}
```

This measures the execution time of each part, showing not only the correctness but also the efficiency of our solutions.

## Key Takeaways

1. **Abstraction through Traits**: By defining the `Parse` trait, we created a flexible interface for different parsing strategies.
2. **Functional Programming**: Using iterators and closures makes our code concise and expressive.
3. **Buffer Management**: For Part 2, we maintain a buffer to detect spelled-out digits, handling overlapping cases elegantly.
4. **Performance Consideration**: The code includes timing measurements, showing a focus on efficiency.

This problem demonstrates how appropriate abstractions and data structures can make complex parsing tasks straightforward and maintainable.
