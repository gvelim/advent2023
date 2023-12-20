# Day 10
## Input
You make a quick sketch of all the surface pipes you can see (your puzzle input).  The pipes are arranged in a two-dimensional grid of tiles:
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

Total enclosed tiled = 10
```
## Approach
### Part 1
The key logic is about navigating each step considering 
1. output direction of the pipe we are standing on; current tile, 
2. input direction of pipe we are moving into next

For example, we can move into pipe `L` only from directions
1. `down`/`south`
2. `left`/`west`

The below logic will return 
1. new direction vector given (a) `self` as current direction vector and (b) `pipe` we are about to step on
2. OR, `None` for all other combinations
```rust
enum Direction { Up, Right, Down, Left }
impl Direction {
    pub(crate) fn pipe_exit(&self, pipe: char) -> Option<Self> {
        use Direction::*;
        match (self, pipe) {
            (Left | Right, '-') => Some(*self),
            (Up | Down, '|') => Some(*self),
            (Down, 'L') => Some(Right),
            (Left, 'L') => Some(Up),
            (Down, 'J') => Some(Left),
            (Right, 'J') => Some(Up),
            (Up, '7') => Some(Left),
            (Right, '7') => Some(Down),
            (Up, 'F') => Some(Right),
            (Left, 'F') => Some(Down),
            _ => None
        }
    }
}
```
### Part 2
To address Part 2 we will make use of an algorithm similar to **polygon fill**, where we scan each line for a **pairs of vertical cuts**, where **odd pais fall outside** the loop and **even ones fall inside**. We then know that even pairs enclose the tiles we need to count.

To achieve this we need to perform the following steps
1. Sort and Group-by the loop coordinates by `y` and with each `y` group sorted by `x`.
2. For each line we scan
   1. Clean the line from redundant information 
      1. Remove `-` as we don't need horizontal pipes,
      2. Remove reduntant pipes, that is, pipes that when removed **do not alter** the number of valid pairs formed by the line
         1. Remove `J` from cases like `FJ` or `F--J` as `J` is a **reduntant** to the `F` 
         2. Remove `L` from cases like `L7` or `L--7` as `L` is a **reduntant** to the `7` 
   2. Pair up together those pipes survived the clean-up, hence we now have the **pairs of vertical cuts** for the line
   3. The number of tiles enclosed by a pair is equal to pair's `x` distance minus 1
3. Sum up all lines for the total number of tiles enclosed by the loop

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

