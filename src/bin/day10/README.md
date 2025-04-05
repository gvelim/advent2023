# Day 10: Pipe Maze

## Problem Description
You make a quick sketch of all the surface pipes you can see (your puzzle input). The pipes are arranged in a two-dimensional grid of tiles:
```
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
```
- `|` is a vertical pipe connecting `north` and `south`.
- `-` is a horizontal pipe connecting `east` and `west`.
- `L` is a 90-degree bend connecting `north` and `east`.
- `J` is a 90-degree bend connecting `north` and `west`.
- `7` is a 90-degree bend connecting `south` and `west`.
- `F` is a 90-degree bend connecting `south` and `east`.
- `.` is ground; there is no pipe in this tile.
- `S` is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is **one large, continuous loop**.

## Part 1
Find the single giant loop starting at `S`. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?
```
'J', 'F', 'J', 'F', '7', '|', 'L', '7', 'J', '-', '-', 'F', 'J', 'L', '|'
                                    ^
                        Point further out - Step 8
```
## Part 2
You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?
```
...........  =>  ...........
.S-------7.  =>  .S-------7.
.|F-----7|.  =>  .|F-----7|.
.||.....||.  =>  .||OOOOO||.
.||.....||.  =>  .||OOOOO||.
.|L-7.F-J|.  =>  .|L-7OF-J|.
.|..|.|..|.  =>  .|II|O|II|.
.L--J.L--J.  =>  .L--JOL--J.
...........  =>  .....O.....
```
The above loop encloses merely four tiles - the two pairs of `.` in the southwest and southeast (marked `I` below). The middle `.` tiles (marked `O` below) are not in the loop

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?
```
.............  =>  ............. -> 0
.S---------7.  =>  .S---------7. -> 0
.|..F-7.F7.|.  =>  .|IIF-7IF7I|. -> 4
.|.FJ.|.|L7|.  =>  .|IFJ.|I|L7|. -> 2
.|FJ..L-J.||.  =>  .|FJ..L-J.||. -> 0
.|L-7...F-J|.  =>  .|L-7...F-J|. -> 0
.|..|...|..|.  =>  .|II|...|II|. -> 4
.L--J...L--J.  =>  .L--J...L--J. -> 0
.............  =>  ............. -> 0

Total enclosed tiles = 10
```

## Solution Approach

### Part 1: Pipe Navigation System

The first challenge requires finding the path through a continuous loop of connected pipes. To achieve this, we need to understand how to navigate through different pipe shapes, considering:

1. The current direction we're moving in
2. The pipe type we're entering
3. The resulting direction after navigating the pipe

Let's model this with a `Direction` enum that captures movement in four cardinal directions:

```rust
#[derive(Debug, Copy, Clone, PartialEq)]
pub(crate) enum Direction { Up, Right, Down, Left }
```

With this representation, we can define how direction changes when moving through different pipe types. For example, when moving down and encountering an `L` pipe, we'll turn right; when moving left and encountering an `F` pipe, we'll go down:

```rust
impl Direction {
    pub(crate) fn pipe_exit(&self, pipe: char) -> Option<Self> {
        use Direction::*;
        match (self, pipe) {
            (Left | Right, '-') => Some(*self),            // Horizontal pipe preserves direction
            (Up | Down, '|') => Some(*self),               // Vertical pipe preserves direction
            (Down, 'L') => Some(Right),                    // L bends down→right
            (Left, 'L') => Some(Up),                       // L bends left→up
            (Down, 'J') => Some(Left),                     // J bends down→left
            (Right, 'J') => Some(Up),                      // J bends right→up
            (Up, '7') => Some(Left),                       // 7 bends up→left
            (Right, '7') => Some(Down),                    // 7 bends right→down
            (Up, 'F') => Some(Right),                      // F bends up→right
            (Left, 'F') => Some(Down),                     // F bends left→down
            (_, 'S') => Some(*self),                       // Starting position keeps current direction
            _ => None                                      // Invalid movement
        }
    }
}
```

This function is the core of our navigation system - it either returns the new direction after moving through a pipe or `None` if the movement is invalid.

To traverse the pipe loop, we create an `Elf` struct that implements the `Iterator` trait:

```rust
impl Iterator for Elf<'_> {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.dir {
            Up => (self.pos.0, self.pos.1-1),
            Right => (self.pos.0+1, self.pos.1),
            Down => (self.pos.0, self.pos.1+1),
            Left => (self.pos.0-1, self.pos.1),
        };

        // Try to move to the next position and update direction
        self.field.get_pipe(pos)
            .and_then(|p|
                self.dir.pipe_exit(p)
                    .map(|dir| {
                        self.pos = pos;
                        self.dir = dir;
                        (p,pos)
                    })
            )
    }
}
```

Using this system, we can simply traverse the loop until we return to the starting position, then divide the total steps by 2 to find the farthest point.

### Part 2: Finding Enclosed Tiles

For Part 2, we need to determine which tiles are enclosed by the loop. This is a classic point-in-polygon problem, which we'll solve using a scan-line algorithm:

1. First, we order the pipe coordinates by scanlines (rows with the same y-coordinate):

```rust
pub(crate) fn order_by_scan_lines(&mut self) -> impl Iterator<Item=&mut [Step]> + '_ {
    self.path.sort_by(|(_, a), (_, b)|
        match a.1.cmp(&b.1) {
            Ordering::Equal => a.0.cmp(&b.0),
            cmp => cmp
        });

    self.path.chunk_by_mut(|(_, a), (_, b)| a.1 == b.1)
}
```

2. Then we need to clean up each line by removing pipes that don't affect the boundary crossing count:

```rust
fn get_valid_pairs(&mut self, f: &Field) -> impl Iterator<Item=&Self::Output> {
    let mut pipes_removed = 0;

    self.iter_mut()
        .filter_map(move |p| {
            match p.0 {
                // Remove horizontal pipes - they don't affect vertical crossings
                '-' => { pipes_removed += 1; None },
                // Remove redundant J pipes in patterns like F--J
                'J' if f.connects_left_with(p.1)
                    .is_some_and(|c| 'F'.eq(c)) => {
                        pipes_removed += 1; None
                    },
                // Remove redundant L pipes in patterns like L--7
                'L' if f.connects_right_with(p.1)
                    .is_some_and(|c| '7'.eq(c)) => {
                        pipes_removed += 1; None
                    },
                // Keep other pipes and adjust x-coordinate
                _ => {
                    p.1.0 -= pipes_removed;
                    Some(&*p)
                }
            }
        })
}
```

3. Finally, we pair up the remaining pipes and count the enclosed tiles between pairs:

```rust
let tiles = path
    .order_by_scan_lines()
    .map(|line| {
        // Clean and collect valid vertical pipes pairs
        pairs.clear();
        line.get_valid_pairs(&f)
            .collect_into(&mut pairs);

        // Count tiles between pairs
        pairs.chunks(2)
            .map(|pair| {
                let [(_,a),(_,b)] = pair else { unreachable!() };
                b.0 - a.0 - 1  // Distance between pipes minus 1
            })
            .sum::<usize>()
    })
    .sum::<usize>();
```

The key insight is that between each valid pair of pipes, all tiles are either all inside or all outside the loop. By removing redundant pipes (like paired F--J combinations), we can accurately count tiles that are truly enclosed.

As shown in our visual example:
```
Sort & Group-By ->  Clean-up ->  Pair up          ->   Count
.............
.S---------7.       S7           `S7`                  -> 0
.|..F-7.F7.|.       |..F7.F7.|   `|..F`, `7.F`, `7.|`  -> 4
.|.FJ.|.|L7|.       |.F.|.|7|    `|.F`, `|.|`, `7|`    -> 2
.|FJ..L-J.||.  =>   |F..LJ.||    `|F`, `LJ`, `||`      -> 0
.|L-7...F-J|.       |7...F|      `|7`, `F|`            -> 0
.|..|...|..|.       |..|...|..|  `|..|`, `|..|`        -> 4
.L--J...L--J.       LJ...LJ      `LJ`, `LJ`            -> 0
.............

Total enclosed tiles = 10
```
