# Day 18: Lavaduct Lagoon
## Input
To catch up with the large backlog of parts requests, the factory will also need a **large supply of lava** for a while; the Elves have already started creating a large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the **dig plan** (your puzzle input). For example:
```
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
```
The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters **up** (U), **down** (D), **left** (L), or **right** (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with **the color that the edge of the trench should be painted** as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the following loop of **trench** (#) having been dug out from otherwise **ground-level terrain** (.):
```
#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
```
## Part 1
At this point, the trench could contain `38` cubic meters of lava. However, this is just the edge of the lagoon; the next step is to **dig out the interior** so that it is one meter deep as well:
```
↑ → → → → → → 
↑ ◼ ◼ ◼ ◼ ◼ ↓ 
← ← ↑ ◼ ◼ ◼ ↓ 
. . ↑ ◼ ◼ ◼ ↓ 
. . ↑ ◼ ◼ ◼ ↓ 
↑ → → ◼ ← ← ↓ 
↑ ◼ ◼ ◼ ↓ . . 
← ↑ ◼ ◼ ↓ → → 
. ↑ ◼ ◼ ◼ ◼ ↓ 
. ← ← ← ← ← ↓ 
```
Now, the lagoon can contain a much more respectable `62` cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, **how many cubic meters of lava could it hold**?

## Part 2
The Elves were right to be concerned; the planned lagoon would be **much too small**.

After a few minutes, someone realizes what happened; someone **swapped the color and instruction parameters** when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

Each hexadecimal code is **six hexadecimal digits** long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the **direction to dig**: `0` means `R`, `1` means `D`, `2` means `L`, and `3` means `U`.

So, in the above example, the hexadecimal codes can be converted into the true instructions:

```
#70c710 = R 461937
#0dc571 = D 56407
#5713f0 = R 356671
#d2c081 = D 863240
#59c680 = R 367720
#411b91 = D 266681
#8ceee2 = L 577262
#caa173 = U 829975
#1b58a2 = L 112010
#caa171 = D 829975
#7807d2 = L 491645
#a77fa3 = U 686074
#015232 = L 5411
#7a21e3 = U 500254
```
Digging out this loop and its interior produces a lagoon that can hold an impressive `952408144115` cubic meters of lava. 

Convert the hexadecimal color codes into the correct instructions; if the Elves follow this new dig plan, **how many cubic meters of lava could the lagoon hold**?
# Approach
## Digging a trench
A trench is defined by its `RGB` color and `Direction` was dug,
therefore, we capture the data relationship with the below data structures. 
```rust
enum Direction { U, R, D, L }

struct Trench(Rgb, Direction);
```
We store each trench point in a `BTreeMap` ordered by `Y` coordinate first and followed by `X` coordinate. 
This will enable us later on, to retrieve all `X` coordinates for a specific line `Y`. 
Since a `Lagoon` contains one or more `Trench` we use the below data structure to hold the trench points, 
including the **top-left**, **bottom-right** positions of the 2D grid, hence tracking grid boundaries.
```rust
struct Lagoon {
    min: Position,
    max: Position,
    map: BTreeMap<Position, Trench>,
}
```
In terms of instruction plan, we use a simple collection of instructions;
hence we use the following data structure to represent a single instruction.
```rust
struct Instruction {
    pub dir: Direction,
    pub run: usize,
    pub rgb: Rgb
}
```
Therefore, digging the lagoon perimeter is as simple as capturing all the points for every trench dug. 
The `Digger` structure undertakes the role of digging a `lagoon` and is tracking its current position. 
The below function `dig()` takes both a `lagoon` and an `instruction`, mutates `lagoon` and returns the trench length dug.
```rust
pub(crate) struct Digger {
    // track latest position
    pos: Position
}

impl Digger {
...
    fn dig(&mut self, lagoon: &mut Lagoon, instr: &Instruction) -> usize {
        (0..instr.run)
            .take_while(|_| { 
                lagoon.dig_trench(                     
                    *self.pos.next_mut(instr.dir),                     
                    Trench(instr.rgb, instr.dir), 
                ).is_none()
            })
            .count()
}
```
Therefore, the sum of instruction lengths executed will give us the perimeter of the `lagoon`, as it is captured by the `total` variable below.
```rust
...
    let mut lagoon = Lagoon::default();
    let mut digger = Digger::new(Position(0, 0));
 
    let total = plan
        .iter()
        .map(|ins| digger.dig(&mut lagoon, ins))
        .sum::<usize>();
...
```
## Calculating lagoon's area
Now that we have dug the `lagoon` perimeter and also know its full length it is time to calculate the `laggon` area covered.

To calculate the area covered in a 2D grid, we use a form of **polygon fill algorithm** and particularly we use the **trench direction** in order to figure out which part of the space evaluated falls inside or outside the lagoon's enclosed area.

To understand the area enclosed by the lagoon trench, by observing  how trenches are lining up next to each other, we find out that enclosed area is denoted by the following direction pairs
* `↑ ↓` : always falls inside lagoon's perimeter
* `→ ←` : given direction before `←` was `↓`, otherwise area falls outside the lagoon's perimeter
* `↑ ←` : always falls inside lagoon's perimeter
* `→ ↓` : always falls inside lagoon's perimeter

Hence, by scanning each line for **"direction pairs"** that match the above combinations we can extract the enclosed areas
```
Lagoon Grid                             Scan line range evaluation
-----------------------------------     -------------------------------------------------
. . . . . . . ↑ → → → → → → . . . .   =  0 : ↑ →, → →, → →, → →, → →, → →,
                                              x    x    x    x    x    x
. . . . . . . ↑ ◼ ◼ ◼ ◼ ◼ ↓ . . . .   =  1 : ↑ ↓, 
                                              ✓
↑ → → → → . . ← ← ↑ ◼ ◼ ◼ ↓ . ↑ → →   =  1 : ↑ → ,→ →, → ←, ← ↑, ↑ ↓, ↓ ↑, ↑ →, → →
                                              x    x    x    x    ✓    x    ✓    x
↑ ◼ ◼ ◼ ↓ . . . . ↑ ◼ ◼ ◼ ↓ . ↑ ◼ ↓   =  3 : ↑ ↓, ↓ ↑, ↑ ↓, ↓ ↑, ↑ ↓
                                              ✓    x    ✓    x    ✓
↑ ◼ ◼ ◼ ↓ → → → . ↑ ◼ ◼ ◼ ↓ → → ◼ ↓   =  3 : ↑ ↓, ↓ →, → →, → →, → ↑, ↑ ↓, ↓ →, → →, → ↓
                                              ✓    x    x    x    x    ✓    x    x    ✓ 
↑ ◼ ◼ ◼ ◼ ◼ ◼ ↓ . ↑ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ↓   =  ...
← ← ← ↑ ◼ ◼ ◼ ↓ . ↑ ◼ ◼ ◼ ← ← ↑ ◼ ↓   =  ...
. . . ↑ ◼ ◼ ◼ ↓ . ↑ ◼ ◼ ◼ ↓ . ↑ ◼ ↓   =  ...
. . . ← ← ↑ ◼ ↓ → → ◼ ◼ ◼ ↓ . ← ← ↓   =  ...
. . . . . ↑ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ↓ → → . .   =  ...
. . . ↑ → → ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ↓ . .   =  ...
. . . ↑ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ◼ ↓ . .   =  ...
. . . ← ← ← ← ← ← ← ← ← ← ← ← ↓ . .   =  ...
```
With the above understanding we need to adjust `Trench` structure to hold the Direction of the previous trench so we can evaluate the condition for the `→ ←` pair.
```rust
struct Trench(Rgb, Direction, Option<Direction>);

pub(crate) struct Digger {
    pos: Position,
    // Track direction of previous instruction
    last: Option<Direction>,
}

impl Digger {
...
    pub fn dig(&mut self, lagoon: &mut Lagoon, instr: &Instruction) -> usize {
        let ret = (0..instr.run)
            .take_while(|_| {
                lagoon
                    .dig_trench(
                        *self.pos.next_mut(instr.dir),
                        // store in trench previous direction
                        Trench(instr.rgb, instr.dir, self.last),
                    )
                    .is_none()
            })
            .count();
        // store instruction direction
        self.last = Some(instr.dir);
        ret
    }
...
}
```
With the `Trench` adjustment completed we can now implement a function
that takes a line position `y` and returns the list of line ranges,
in the form of `(start..end)`, which **cut/intersect** the lagoon's enclosed areas.
It is here where the `BTreeMap` can efficiently give us the list of `x` points falling onto the line `y`.
```rust
impl Lagoon {
    ...
    fn floodfill_intersections(&self, line: Unit) -> impl Iterator<Item=Range<Unit>> + '_ {
        use Direction as D;
        let mut last: Option<(&Unit, &Direction)> = None;

        self.map
            // get a sorted list of all 'x' given a line at 'y'
            .range(Position(Unit::MIN, line)..=Position(Unit::MAX, line))
            // filter out the non-enclosed areas
            .filter_map(move |(Position(x, _), Trench(_, d, pd))| {
                let mut out = None;
                if let Some((lx, ld)) = last {
                    out = match (ld, d) {
                        (D::U, D::D) |
                        (D::U, D::L) |
                        (D::R, D::D) => Some(*lx..*x),
                        // gray case, needs condition check
                        (D::R, D::L)
                        // have we made a clockwise turn to reach D::L ? 
                        if pd.map(|pd| d.is_clockwise(pd)).unwrap_or(false)
                        // if yes, we found another enclosed area
                        => Some(*lx..*x),
                        _ => None,
                    }
                }
                last = Some((x, d));
                out
            })
    }
...
}
```
With the above function at hand, finding the lagoon's area becomes the sum of all line intersections between the minimum and maximum points in the `Y` axis; We implement this logic in the following function. 
```rust
impl Lagoon {
...
    fn calculate_area(&self) -> usize { 
        (self.min.1..=self.max.1)
            .flat_map(|y| {
                self.floodfill_intersections(y)
                    .map(|rng| (rng.len() - 1) as usize)
            })
            .sum::<usize>()
    }
...
}
```

