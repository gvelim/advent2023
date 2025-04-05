# Day 13: Point of Incidence

## Input
You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

For example:
```
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
```
To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns

For example, in the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:

```
123456789
    ><
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><
123456789
```
The second pattern reflects across a horizontal line instead:

```
1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7
```
## Part 1
To **summarize** your pattern notes, add up **the number of columns** to the left of each vertical line of reflection; to that, also **add 100 multiplied by the number of rows** above each horizontal line of reflection.

Find the line of reflection in each of the patterns in your notes. **What number do you get after summarizing all of your notes?**
```
  <---4--->
"#.##.|.##."
"..#.#|#.#."
"##...|...#"
"##...|...#"
"..#.#|#.#."
"..##.|.##."
"#.#.#|#.#."
Horizontal:[None], Vertical:[position:5, radius:4] -> Result: 5

"#...##..#"
"#....#..#" <
"..##..###" |
"#####.##." |
----------- 3
"#####.##." |
"..##..###" |
"#....#..#" <
Horizontal[position:4, radius:3], Vertical[None] -> Result 4 * 100

Sum: 405
```
## Part 2
You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing. Upon closer inspection, you discover that every mirror has exactly one smudge: **exactly one** `.` or `#` should be the opposite type

In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?
```
"..##..##." <
"..#.##.#." |
"##......#" |
----------- 3
"##......#" |
"..#.##.#." |
"..##..##." <
"#.#.##.#."
Horizontal[position:3, radius:3], Vertical[position:5, radius:4] -> Result 3 * 100 = 300

"#....#..#" <
----------- 1
"#....#..#" <
"..##..###"
"#####.##."
"#####.##."
"..##..###"
"#....#..#"
 Horizontal[position:1, radius:1], Vertical[None] -> Result 1 * 100 = 100

 Total = 400
```

## Solution Approach

### Understanding the Problem

Before diving into the solution, let's understand what constitutes a perfect reflection and a smudged reflection:

```
A perfect reflection      A smudged reflection is
MUST contains either      similar toa perfect one,
the first or last         but with a *ONE* flawed
column/row or both        reflection

   <---4--->                   <---4--->
"#[.##.|.##.]" = 4          "#[.##.|.##.]" 4
".[.#.#|#.#.]" = 4          ".[.#.#|#.#.]" 4
"#[#...|...#]" = 4          "#[#...|...#]" 4
"#[#...|...#]" = 4          "##.[..|..]##" 2 <-- smudged reflection
".[.#.#|#.#.]" = 4          ".[.#.#|#.#.]" 4
".[.##.|.##.]" = 4          ".[.##.|.##.]" 4
"#[.#.#|#.#.]" = 4          "#[.#.#|#.#.]" 4
```

### Part 1: Finding Perfect Reflections

Our solution leverages a systematic approach by checking each potential reflection line position and verifying if it creates a perfect reflection. For each line in the pattern, we calculate how many characters reflect perfectly at a given position.

#### Checking Reflections in a Single Line:

```
Starting form index position 1; 2nd position for zero based index arrays

[#|.]##..##. => Index 1, Reflected: 0 => Abandon, scan next Index

#[.|#]#..##. => Index 2, Reflected: 0 => Abandon, scan next Index

#.[#|#]..##. => Index 3, Reflected: 1 => found a reflection, not perfect, expand
#[.#|#.].##. => Index 3, Reflected: 2 => found a reflection, not perfect, expand
[#.#|#..]##. => Index 3, Reflected: 2 => abandon, scan next Index

#.#[#|.].##. => Index 4, Reflected: 0 => Abandon, scan next Index

#.##.[.|#]#. => Index 5, Reflected: 0 => Abandon, scan next Index

#.##..[#|#]. => Index 6, Reflected: 1 => found a reflection, not perfect, expand
#.##.[.#|#.] => Index 6, Reflected: 2 => Found a perfect reflecton !!
```

The core of this approach is implemented in our `reflections_at_index` function, which:
1. Splits the string at the potential reflection point
2. Reverses the left part and compares with the right part
3. Counts matching characters until a mismatch is found

This function is elegantly implemented using Rust's iterators:

```rust
fn reflections_at_index(s: &str, idx:usize) -> usize {
    let (l, r) = s.split_at(idx);
    let li = l.bytes().rev();
    let mut ri = r.bytes();
    li.take_while(|&lc| ri.next() == Some(lc)).count()
}
```

#### Applying to the Full Pattern:

We then apply this check to each line in the pattern, ensuring all lines reflect perfectly at the same position:

```
For
Index 1            Index 2            Index 3            Index 4             Index 5            Index 6

[#|.]##..##. = 0   #[.|#]#..##. = 0   #.[#|#]..##. = 1   #.#[#|.].##. = 0    #[.##.|.##.] = 4   #.##.[.|#]#. = 0
[. .]#.##.#. Stop  .[. #].##.#. Stop  ..[#|.]##.#. = 0   ..#[. #]#.#. Stop   .[.#.#|#.#.] = 4   ..#.#[# .]#. Stop
[# #]......#       #[# .].....#       ##[. .]....# Stop  ##.[. .]...#        #[#...|...#] = 4   ##...[. .].#
[# #]......#       #[# .].....#       ##[. .]....#       ##.[. .]...#        #[#...|...#] = 4   ##...[. .].#
[. .]#.##.#.       .[. #].##.#.       ..[# .]##.#.       ..#[. #]#.#.        .[.#.#|#.#.] = 4   ..#.#[# .]#.
[. .]##..##.       .[. #]#..##.       ..[# #]..##.       ..#[# .].##.        .[.##.|.##.] = 4   ..##.[. #]#.
[# .]#.##.#.       #[. #].##.#.       #.[# .]##.#.       #.#[. #]#.#.        #[.#.#|#.#.] = 4   #.#.#[# .]#.
                                                                                       Finished
Max Height 1       Max Height 1       Max Height 2       Max Height 1        Max Height 7       Max Height 1
                                                                              ** MATCH **
Perfect Line Mirror at Index 5 with radius 4
```

This check is encapsulated in the `find_perfect_reflection` function:

```rust
pub(crate) fn find_perfect_reflection(pat: &[String]) -> impl Iterator<Item=Reflection> + '_ {
    let width = pat[0].len();

    (1..width)
        .filter(move |&idx|
            pat.iter()
                .map(|line| Pattern::reflections_at_index(line, idx))
                .all(|r| idx == r || idx + r == width)
        )
}
```

A key optimization is that we handle both vertical and horizontal reflections with the same algorithm by transposing the pattern once during initialization:

```rust
fn transpose(p: &[String]) -> impl Iterator<Item=String> + '_ {
    (0..p[0].len())
        .map(move |col| {
            p.iter().map(|line| line.as_bytes()[col] as char).collect::<String>()
        })
}
```

### Part 2: Finding Smudged Reflections

For smudged reflections, we adapt our approach to count and track imperfections:

```
Index 5

#[.##.|.##.] = 4 <-- Perfect reflection
.[.#.#|#.#.] = 4
##..[.|.]#.# = 1 <-- Smudged reflection at radius 2 (1 + 1)
#[#...|...#] = 4
.[.#.#|#.#.] = 4
.[.##.|.##.] = 4
#[.#.#|#.#.] = 4

Max Height  7
```

This requires tracking the quality of reflections at each position using a frequency counter:

```
Array Length =  Zero ---- to ----> Pattern Width
            freq[0, 1, 0, 0, 6, 0, 0]
   radius = pos :  '1'      '4'

For pattern height = 7, the array reads as:
radius '4' - appeared 6/7 times
radius '1' - appeared 1/7 time
```

The implementation builds on our perfect reflection algorithm but adds counting logic:

```rust
pub(crate) fn find_smudged_reflection(pat: &[String]) -> impl Iterator<Item=Reflection> + '_ {
    let (width, height) = (pat[0].len(), pat.len());
    let mut smudge_counter = vec![0; width];

    (1..width)
        .filter(move |&idx| {
            let mut radius = usize::MIN;
            smudge_counter.fill(0);

            let line_found = pat.iter()
                .map(|line| Pattern::reflections_at_index(line, idx))
                .all(|r| {
                    radius = std::cmp::max(r,radius);
                    smudge_counter[r] += 1;
                    smudge_counter[0] < 2 && smudge_counter[..radius].iter().sum::<usize>() < 2
                });

            line_found && smudge_counter[radius] == height-1
        })
}
```

A critical optimization is early termination - we stop checking a potential position as soon as we detect more than one flaw.

To calculate the final summary, we check for reflections in both orientations and apply the appropriate multiplier:

```rust
pub(crate) fn summarise_notes<'a, F, I>(&'a self, find: F) -> usize
    where
        F: Fn(&'a [String]) -> I,
        I: Iterator<Item = Reflection> + 'a
{
    self.patterns.iter()
        .map(|pat|
            find(&pat.t).next()
                .map(|v| (Some(v), None))
                .or_else(||{ Some((None, find(&pat.p).next())) })
                .unwrap()
        )
        .map(|(v,h)| {
            v.map(|v| v * 100)
                .or_else(|| Some(h.unwrap_or(0)) )
                .unwrap()
        })
        .sum::<usize>()
}
```

This modular design lets us reuse the same summarization logic for both parts of the puzzle by simply swapping the reflection-finding algorithm.
