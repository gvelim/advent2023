# Day 3: Gear Ratios - Learning About Grid Processing

## Problem Statement
The engine schematic consists of a visual representation of the engine. We need to identify "part numbers" - any numbers that are adjacent to a symbol (including diagonally). This is a classic grid processing problem that teaches us important concepts about data representation and adjacency detection.

Example input:
```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

## Part 1: Finding Part Numbers
Our first task is to identify all part numbers and calculate their sum. A part number is any number adjacent to a symbol (horizontally, vertically, or diagonally).

From the example above, the part numbers are:
```
467 + 35 + 633 + 617 + 592 + 755 + 664 + 598 = 4361
```

## Part 2: Understanding Gear Ratios
Building on Part 1, we now focus on a specific type of symbol: gears. A gear is any `*` symbol that is adjacent to exactly two part numbers. The "gear ratio" is the product of these two numbers. We need to sum all gear ratios in the schematic.

From our example:
```
467 * 35 + 755 * 598 = 467835
```

## Solution Strategy: From 2D to 1D

When working with grid problems, we have two main approaches: processing the grid as a 2D structure or flattening it to a 1D structure. Let's explore the 1D approach, which simplifies many operations:

### 1. **Grid Flattening: Converting Dimensions**
Instead of dealing with row and column indices, we convert our 2D grid into a single string. This simplifies our data structure while requiring us to understand position mapping:

```rust
// Store the line length to navigate between rows
let len = input.lines().next().unwrap().len();
// Convert the entire grid to a single string
let schematic = input.lines().flat_map(|d| d.chars()).collect::<String>();
```

### 2. **Data Modeling: Representing Elements**
We need to track two types of elements - numbers and symbols - along with their positions in our flattened grid:

```rust
// A part number with its value and position range
struct PartNumber {
    number: u32,
    pos: RangeInclusive<usize>
}
// A symbol with its position and character value
struct Symbol(usize, char);
```

This representation allows us to efficiently check for adjacency between elements.

### 3. **Adjacency Detection: Understanding Grid Navigation**
The key challenge is determining when elements are adjacent. In our flattened grid, we need to translate 2D adjacency to 1D position calculations:

```rust
fn is_touching(&self, pn: &PartNumber, len: usize) -> bool {
    // Check positions above, below, and adjacent using offset arithmetic
    (self.0 - len-1 ..= self.0 - len+1).contains(pn.pos.end()) ||
    // Additional adjacency checks...
}
```

### Understanding Grid Flattening: A Visual Guide

To truly understand how 2D to 1D conversion works, let's visualize it:

```
Original 2D Grid:       Flattened 1D String (offset: 9)
...123...               ...123.....%//*#..
..%//*#..               ^       ^
                        |       |
                        Position relationships:
                        ||+---10----+  (diagonal right/below)
                        ||+---9----+   (directly below)
                        |+---9----+    (diagonal left/below)
                        +---9----+     (directly above)
                        +---8---+      (diagonal above)

Position Analysis Example:
..abbbc..   ==>         ..abbbc....d123d....efffg..
..d123d..                     |    |    |
..efffg..                     |    |    |
                              |    |    |
Element Relationships:        |    |    |
- To move down one row:        +9   +9   +9
- To move down diagonally:    +8/+10 +8/+10 +8/+10
- To move horizontally:        ±1    ±1    ±1
```

This visualization helps us understand how to navigate between rows in our flattened representation. Using the line length (offset), we can move between rows by adding or subtracting this value, and check for adjacency in all eight directions around an element.

By understanding these position relationships, we can efficiently detect adjacency without needing to maintain a complex 2D data structure.
