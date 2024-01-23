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
We hold the input data into a single dimensional array therefore we'll do all the work using solely indexes
```
Input Form         Single Dimension Array Form

O....#....            Line 1     Line 2   Line 3           Line 10
O.OO#....#          <- W:10 -><- W:10 -><- W:10 ->  ...  <- W:10 ->
.....##...         [O....#....O.OO#....#.....##...  ...  #OO..#....]
   ...
#OO..#....
```
Therefore, moving vertically in a single dimension:
- A line step is equal to `+/- Width`
- Vertical movement logical bounds are defined as
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
Also, moving horizontally in a single dimension and while movement must be contained in the same logical line: 
- Step is equal to `+/- 1`
- horizontal movement logical bounds defined as 
```
        Line 1     Line 2      Line 3  
      <- W:10 -> <- W:10 -> <- W:10 ->  etc
array[O....#.... O.OO#....# .....##...  ...]
                |          | 
                |    ^     |
                |    |     |
 index % W > 0  |< Index  >| index % W < W - 1
    Lower Bound              Higher Bound
```
Therefore, we encapsulate the above logic in a function that takes (a) the current index position and (b) direction, and returns either the new `index` location or `nothing` if moving would push us **out of the logical bounds**   
```rust
fn next(&self, idx: usize, dir:Direction) -> Option<usize> {
    match dir {
        Direction::East if idx % self.width < self.width - 1 => Some(idx + 1),
        Direction::West if idx % self.width > 0 => Some(idx - 1),
        Direction::North if idx > self.width => Some(idx - self.width),
        Direction::South if idx < self.layout.len() - self.width => Some(idx + self.width),
        _ => None
    }
}
```
Therefore, given a round rock's position we can easily move it to any direction by recursing into the next feasible location. The below function will move the rock on the dish and return its **new logical line position** which we can use to calculate rock's cost. 
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
Performing `1,000,000,000` spin cycles will take a very long time, however after a certain spin cycle, we observe that rock arrangements are identical to previously seen ones, therefore rock arrangements have a fixed period of reoccurrence.
```
Spin Cycle   Cost    Rock Arrangement 
                     last seen in cycle
==========   ====    ==================
    1,        87,    None
    2,        69,    None
    3,        69,    None     <------------------+
    4,        69,    None                        |
    5,        65,    None                    Reoccurence
    6,        64,    None                      Period
    7,        65,    None                        |
    8,        63,    None                        |
    9,        68,    None     <------------------+
    10,       69,    Some(3) <- Same as cycle 3  
    11,       69,    Some(4)                     
    12,       65,    Some(5)
    13,       64,    Some(6)  
```
In the above example, we see **Cycle 3** reoccurs in **Cycle 10**, giving us a `period: 10 - 3 =  7`. Therefore, we extrapolate the cost at nth cycle by finding the **fist cost occurrence** where the below condition holds `true`.
```
(nth cycle - first seen) % period == 0
```
A `HashMap` is used to store the rock arrangement at the end of each cycle and is queried immediately after for the confirmation of a reoccurrence. The below logic captures the approach discussed and will run for as long as it takes for the key condition to turn `true`
```rust
 fn spin_cycle_nth(&mut self, nth: usize) -> Option<Cost> {
     let mut map = std::collections::HashMap::<Vec<u8>,usize>::new();

     (1..nth)
         .map(|cycle| (
             cycle,
             self.spin_cycle(),
             map.insert(self.layout.clone(),cycle)
         ))
         .skip_while(|(cycle, _, seen)|
             seen.map(|last| {
                 (nth - last) % (cycle - last) != 0
             }).unwrap_or(true)
         )
         .map(|(_,cost,_)| cost)
         .next()
 }
```
