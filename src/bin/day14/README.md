# Day 14: Parabolic Reflector Dish
## Input
A massive parabolic reflector dish attached to the side of another large mountain. The dish is made up of many small mirrors, each individual mirror seems to be pointing in slightly the wrong direction.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, **the weight of the rocks deforms the platform**, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. 

You note the positions of all the empty spaces (.) and rocks (your puzzle input). For example:
```
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
```
Start by tilting the lever so all of the rocks will slide north as far as they will go:
```
OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
```
## Part 1
You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:
```
OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1
```
The total load is the sum of the load caused by all the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, **what is the total load on the north support beams?**
## Part 2
The parabolic reflector dish deforms, but not in a way that focuses the beam. To do that, you'll need to move the rocks to the edges of the platform. Fortunately, a button on the side of the control panel labeled **"spin cycle"** attempts to do just that!

Each **cycle** tilts the platform four times so that the rounded rocks roll **north**, then **west**, then **south**, then **east**. After each tilt, the rounded rocks roll as far as they can before the platform tilts in the next direction. After one cycle, the platform will have finished rolling the rounded rocks in those four directions in that order
```
After 1 cycle:
.....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
```
This process should work if you leave it running long enough, but you're still worried about the north support beams. To make sure they'll survive for a while, you need to calculate the total load on the north support beams after `1000000000` cycles.

In the above example, after `1000000000` cycles, the total load on the north support beams is `64`.

Run the spin cycle for `1000000000` cycles. Afterward, **what is the total load on the north support beams?**

## Approach
### Part 1
We hold the info in a single dimensional array therefore we can do all work with a single index value
```
Input Form         Single Dimension Array Form

O....#....            Line 1     Line 2   Line 3           Line 10
O.OO#....#          <- W:10 -><- W:10 -><- W:10 ->  ...  <- W:10 ->
.....##...         [O....#....O.OO#....#.....##...  ...  #OO..#....]
   ...
#OO..#....
```
Moving vertically in a single dimension:
- A line step is equal to `+/- Width`
- Vertical movement logical bounds defined as
```
        Line 1     Line 2            Line 10  
      <- W:10 -> <- W:10 ->  ...   <- W:10 ->
array[O....#.... O.OO#....#  ...   #OO..#....]
                |                 | 
                |        ^        |
                |        |        |
              W >      Index      < array.len()-W
    Lower Bound                     Higher Bound
```
Moving horizontally in a single dimension while avoiding crossing lines: 
- Step is equal to `+/- 1`
- horizontal movement logical bounds defined as 
```
        Line 1     Line 2      Line 3  
      <- W:10 -> <- W:10 -> <- W:10 ->  etc
array[O....#.... O.OO#....# .....##...  ...]
                |          | 
                |    ^     |
                |    |     |
      W*Index/H >  Index   < (Index/H+1)*W-1
    Lower Bound              Higher Bound
```
Therefore, we encapsulate the above logic in a function that given (a) the current index position and (b) direction, will return either the new `index` location or `nothing` if we are about to move **out of the logical bounds**   
```rust
    fn next(&self, idx: usize, dir:Direction) -> Option<usize> {
        match dir {
            Direction::East if idx < (idx/self.lines)*self.width + self.width - 1 => Some(idx+1),
            Direction::West if idx > (idx/self.lines)*self.width => Some(idx - 1),
            Direction::North if idx > self.width => Some(idx - self.width),
            Direction::South if idx < self.layout.len() - self.width => Some(idx + self.width),
            _ => None
        }
    }
```
Therefore, once we know a round rock's position we can easily move it to any direction by recursing into all valid positions. The below function will move the rock on the dish and return its **new logical line position** so we can calculate the cost   
```rust
    fn move_rock(&mut self, idx: usize, dir:Direction) -> Option<usize> {
        if idx >= self.layout.len() { return None }
        self.next(idx,dir)
            .and_then(|next|{
                if self.layout[next] == b'.' {
                    self.layout.swap(idx,next);
                    self.move_rock(next,dir)
                } else {
                    Some(idx / self.lines)
                }
            })
            .or( Some(idx / self.lines) )
    }
```
### Part 2