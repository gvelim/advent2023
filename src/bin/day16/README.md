# Day 16: The Floor Will Be Lava
## Input
You see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (`.`), mirrors (`/`and `\`), and splitters (`|` and `-`).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern. You note the layout of the contraption (your puzzle input). For example
```
. | . . . \ . . . . 
| . - . \ . . . . . 
. . . . . | - . . . 
. . . . . . . . | . 
. . . . . . . . . . 
. . . . . . . . . \ 
. . . . / . \ \ . . 
. - . - / . . | . . 
. | . . . . - | . \ 
. . / / . | . . . . 
```
## Part 1
The beam enters the top-left corner and heading to the right. Then, its behavior depends on what it encounters as it moves:

- If the beam encounters **empty space** (`.`), it continues in the same direction.
- If the beam encounters a **mirror** (`/` or `\`), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a `/` mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a `\` mirror would continue **downward** from the mirror's column.
- If the beam encounters the **pointy end of a splitter** (`|` or `-)`, the beam passes through the splitter as if the splitter were **empty space**. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
- If the beam encounters the **flat side of a splitter** (`|` or `-`), the beam is **split into two beams** going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues **upward** from the splitter's column and one that continues **downward** from the splitter's column. 

Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is **energized** if that tile has at least one beam pass through it, reflect in it, or split in it.
Here is the same diagram but instead only showing whether a tile is energized (`#`) or not (`.`).

Here is the same diagram but instead only showing whether a tile is energized (`#`) or not (`.`):
```
. | . . . \ . . . .        # # # # # # . . . .
| . - . \ . . . . .        . # . . . # . . . .
. . . . . | - . . .        . # . . . # # # # #
. . . . . . . . | .        . # . . . # # . . .
. . . . . . . . . .        . # . . . # # . . .
. . . . . . . . . \   =>   . # . . . # # . . .
. . . . / . \ \ . .        . # . . # # # # . .
. - . - / . . | . .        # # # # # # # # . .
. | . . . . - | . \        . # # # # # # # . .
. . / / . | . . . .        . # . . . # . # . .
 ```
Ultimately, in this example, `46` tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. **With the beam starting in the top-left heading right, how many tiles end up being energized**?
## Part 2
the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that **energizes as many tiles as possible**.

Find the initial beam configuration that energizes the largest number of tiles; **how many tiles are energized in that configuration**?
## Approach
### Data structure
We are using the following collections
1. `Contraption Array`, a single dimension vector holding contraption element. We traverse the data only with the `index` value; no need to `(x,y)` coordinates. 
2. `Energy Array`, a single dimension vector holding the energised tiles, accessed via the `index` value; no need for `(x,y)` coordinates. 
```
Contraption Collection Energy Collection
---------------------- --------------------
. | . . . \ . . . .    # # # # # # . . . .
| . - . \ . . . . .    . # . . . # . . . .
. . . . . | - . . .    . # . . . # # # # #
. . . . . . . . | .    . # . . . # # . . .
. . . . . . . . . .    . # . . . # # . . .
. . . . . . . . . \    . # . . . # # . . .
. . . . / . \ \ . .    . # . . # # # # . .
. - . - / . . | . .    # # # # # # # # . .
. | . . . . - | . \    . # # # # # # # . .
. . / / . | . . . .    . # . . . # . # . .
```
The above is reflected by the below `struct`
```rust
pub(crate) struct Cavern {
    pub(crate) width: usize,
    pub(crate) lines: usize,
    con: std::rc::Rc<[u8]>,
    nrg: Vec<(bool,Vec<Direction>)>,
}
```
### Navigating the cavern
Moving a single step requires knowledge of
* the current `Direction` vector
* the `tile` we are standing on

However, we are told splitters **cause the light-beam to fork into two direction vectors** hence we reflect this as a single unique state.  Therefore, the below `enum` provides the applicable direction states. The `next()` function encodes the direction logic for mirrors and splitters.
```rust
enum Direction { Up, Left, Down, Right, UpDown, LeftRight }
use Direction as D;

impl Direction {
    pub(crate) fn next(&self, tile: u8) -> Direction {
        match (tile, self) {
            (b'/', D::Right) => D::Up,
            (b'/', D::Left) => D::Down,
            (b'/', D::Up) => D::Right,
            (b'/', D::Down) => D::Left,
            (b'\\', D::Right) => D::Down,
            (b'\\', D::Left) => D::Up,
            (b'\\', D::Up) => D::Left,
            (b'\\', D::Down) => D::Right,
            (b'-', D::Up | D::Down) => D::LeftRight,
            (b'|' , D::Right | D::Left) => D::UpDown,
            (_,d) => *d
        }
    }
}
```
Given (a) a direction and (b) current position we can calculate the next location in the single dimensional array using the below formulas. The logic and formulas are explained in more detail at the solution in [Day 14](../day14/README.md)
```rust
fn step(&self, idx: Position, dir:Direction) -> Option<Position> {
    use Direction as D;
    match dir {
        D::Right if idx % self.width < self.width-1 => Some(idx + 1),
        D::Left if idx % self.width > 0 => Some(idx - 1),
        D::Up if (self.width..self.con.len()).contains(&idx) => Some(idx - self.width),
        D::Down if idx < self.con.len() - self.width => Some(idx + self.width),
        _ => None
    }
}
```
### Avoiding Cyclical reflections
Using the above navigation mechanism we quickly realise the contraption causes cyclical reflections of the light-beam.
```
     Start -> . > | V .   .   .   \   .   .   .   .   
              |   . V -   .   \   .   .   .   .   .   
              .   . V .   .   .   |   -   .   .   .   
              .   . V .   .   .   .   .   .   |   .   
              .   . V .   .   .   .   .   .   .   .   
              .   . V .   .   .   .   .   .   .   \   
              .   . V .   .   / > . > \ V \   .   .   
Getting in -----> - > . > - > / ^ .   . V |   .   .   
Circle here   .   | < . < . < . < . < - < |   .   \    
              .   .   /   /   .   |   .   .   .   .   
``` 
Dealing with those cyclical reflections is at the **core of the coding challenge**. Therefore, a critical observation is:
* **_Re-visiting a mirror or splitter from the same direction will result in a cyclical reflection_**

Hence, to prevent from getting into cyclical reflections, 
1. We will need for any mirror or splitters we have visited, remember (a) the location & (b) entry direction vector 
2. If the `tile` we have stepped onto, is either a mirror or splitter and `current direction` == `saved direction`, then we have a **cycle forming condition** hence we have to stop the light-beam

```rust
fn has_entered_cycle(&mut self, idx: Position, dir: Direction) -> bool {
    use Direction as D;

    // Cycle Detection: have we enter the contraption from the same direction before ?
    if self.nrg[idx].1.contains(&dir) { return true }

    // Store light-beam direction at contraption point, for cycles detection
    // Optimise around splitters by storing both opposite directions
    // this stops us from re-entering the cycle from the opposite direction
    match (self.con[idx],dir) {
        (b'-', D::Up|D::Down) => self.nrg[idx].1.extend_from_slice(&[D::Up, D::Down,]),
        (b'|', D::Left|D::Right) => self.nrg[idx].1.extend_from_slice(&[D::Left, D::Right]),
        _ => self.nrg[idx].1.push(dir)
    };
    
    false
}
```
### Traversing the light-beam through the contraption
With the knowledge on how to deal with cycles traversing the light-beam within the contraption, we apply the following recursive logic
1. Proceed with the next step given we aren't in a **cycle forming condition**
2. Flag current tile location as energised
3. Given the current position, get the new direction vector and
   * a splitter is in current position 
     * Derive the new position, and **if valid**, recurse into the 1st direction vector
     * Derive the new position, and **if valid**, recurse into the 2nd direction vector
   * a mirror is in current position, derive the next position, and **if valid**, recurse into 

Hence the exit conditions of the recursion are
* we have a valid **cycle forming condition**
* we have fallen off the grid's bounds

The below code reflects the logic discussed
```rust
fn move_beam(&mut self, idx: Position, dir:Direction) {
    use Direction as D;

    // Has the light-beam fallen into a circle ?
    if self.con[idx] != b'.' && self.has_entered_cycle(idx, dir) { return }

    // Energise cell
    self.nrg[idx].0 = true;

    // Find new direction based on current tile
    match dir.next( self.con[idx] ) {
        D::LeftRight => {
            if let Some(pos) = self.step(idx, D::Left) { self.move_beam(pos, D::Left) };
            if let Some(pos) = self.step(idx, D::Right) { self.move_beam(pos, D::Right) };
        },
        D::UpDown => {
            if let Some(pos) = self.step(idx, D::Down) { self.move_beam(pos, D::Down) };
            if let Some(pos) = self.step(idx, D::Up) { self.move_beam(pos, D::Up) };
        },
        any =>
            if let Some(pos) = self.step(idx, any) { self.move_beam(pos, any) }
    }
}

```
In part 2, to calculate all the entry points where the light-beam, enters the contraption, we resort to the below iterator `chain()` that produces all valid `(position, direction)` pairs
```rust
fn entry_points(w:usize, h:usize) -> impl Iterator<Item=(Position, Direction)> + 'static {
    use Direction as D;

    (0..w).zip(repeat(D::Down))
        .chain((w *(h-1)..w * h).zip(repeat(D::Up)))
        .chain((0..w * h).step_by(w).zip(repeat(D::Right)))
        .chain((0..w * h + 1).step_by(w).skip(1).map(|c| c-1).zip(repeat(D::Left)))
}
```
Therefore, to find the entry point that gives the configuration that **energizes as many tiles as possible**, can be written as 
```rust
let m = entry_points(cavern.width, cavern.lines)
        .map(|(idx,dir)| {
            cavern.energise(idx,dir);
            cavern.measure_energy()
        })
        .max();
```

