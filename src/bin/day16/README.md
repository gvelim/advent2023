# Day 16: The Floor Will Be Lava
## Input
you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example
```
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
```
## Part 1
The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

- If the beam encounters **empty space** (`.`), it continues in the same direction.
- If the beam encounters a **mirror** (`/` or `\`), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a `/` mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a `\` mirror would continue **downward** from the mirror's column.
- If the beam encounters the **pointy end of a splitter**** (`|` or `-)`, the beam passes through the splitter as if the splitter were **empty space**. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
- If the beam encounters the **flat side of a splitter** (`|` or `-`), the beam is **split into two beams** going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues **upward** from the splitter's column and one that continues **downward** from the splitter's column. 

Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is **energized** if that tile has at least one beam pass through it, reflect in it, or split in it.
Here is the same diagram but instead only showing whether a tile is energized (#) or not (.).

Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):
```
######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
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
### Navigating the cavern
Moving a single step requires knowledge of (a) current `Direction` vector & (b) `tile` we are standing on. However, we are told splitters forking the light-beam from one to two direction vectors hence we reflect this as a single unique state.

Therefore, the below enum provides the applicable direction vectors along with the `fn next()` which provides the resulting direction given current direction and tile content.
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

If the next position, given the new direction is valid, we will then receive `Some(New Position)` value otherwise we'll receive `None`
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
Dealing with those cyclical reflections is the core of this coding challenge. We make the following observation that
* **_Re-visiting a mirror or splitter from the same direction will result to a cyclical reflection_**

Hence, to break from the danger of reflection cycles, we will need to
1. For any mirror or splitters we have visited, remember
   * the location
   * entry direction vector 
2. If the `tile` we have stepped on is either a mirror or splitter and `current direction` == `saved direction` then we are entering a cyclical reflection, hence we need to stop the light-bean

```rust
struct Cavern {
    pub(crate) width: usize,
    pub(crate) lines: usize,
    con: std::rc::Rc<[u8]>,
    nrg: Vec<u8>,
    tail: HashMap<Position,Vec<Direction>>  // Detection of Cyclical reflections
}

fn has_entered_cycle(&mut self, tile: u8, idx: Position, dir: Direction) -> bool {
    use Direction as D;

    // we check for cycles only when we've fallen on a contraption
    if tile == b'.' { return false }

    // Cycle Detection: have we stepped onto the mirror/splitter from the same direction before ?
    if Some(true) == self.tail.get(&idx).map(|d| d.contains(&dir)) { return true }

    // Store light-beam direction at location for detection of cycles
    // Optimise around splitters by storing both opposite directions
    // this stops us from re-entering the cycle from the opposite direction
    let store = match (tile,dir) {
        (b'-'|b'|', D::Up| D::Down) => [D::Up, D::Down],
        (b'-'|b'|', D::Left| D::Right) => [D::Left, D::Right],
        _ => [dir,dir]
    };

    self.tail.entry(idx)
        .and_modify(|v| v.extend_from_slice(&store))
        .or_insert(Vec::default())
        .extend_from_slice(&store);

    return false;
}
```
### Traversing the light-beam through the contraption
With the knowledge on how to deal with cycles traversing the light-beam within the contraption we apply the following recursive logic
1. If we are not entering a cycle proceed to next step otherwise abort
2. Flag current tile location as energised
3. Get the new direction vector given current position and
   * if we have hit a splitter
     * Derive the new position **if available**, of the 1st direction vector and recurse 
     * Derive the new position **if available**, of the 2nd direction vector and recurse
   * if we have hit a mirror, derive the next position **if available**, and recurse

Hence the exit conditions of the recursion are
* we have hit a cyclical reflection
* we have hit a cavern wall e.g. we've fallen out of grid bounds

The below code reflect the logic discussed
```rust
fn move_beam(&mut self, idx: Position, dir:Direction) {
    use Direction as D;

    let tile = self.con[idx];

    // Has the light-beam fallen into a circle ?
    if self.has_entered_cycle(tile, idx, dir) { return }

    // Energise cell
    self.nrg[idx] = b'#';

    // Find new direction based on current tile
    match dir.next(tile) {
        D::LeftRight => {
            let _ = self.step(idx, D::Left).is_some_and(|pos| self.move_beam(pos, D::Left) == ());
            let _ = self.step(idx, D::Right).is_some_and(|pos| self.move_beam(pos, D::Right) == ());
        },
        D::UpDown => {
            let _ = self.step(idx, D::Down).is_some_and(|pos| self.move_beam(pos, D::Down) == ());
            let _ = self.step(idx, D::Up).is_some_and(|pos| self.move_beam(pos, D::Up) == ());
        },
        d => {
            let _ = self.step(idx, d).is_some_and(|pos| self.move_beam(pos, d) == ());
        }
    }
}
```
Calculating all the entry points of the light-beam to the cavern we can construct the following iterator, whcih produce all valid `(position, direction)` pairs
```rust
fn entry_points(w:usize, h:usize) -> impl Iterator<Item=(Position, Direction)> + 'static {
    use Direction as D;

    (0..w).zip(repeat(D::Down))
        .chain((w *(h-1)..w * h).zip(repeat(D::Up)))
        .chain((0..w * h).step_by(w).zip(repeat(D::Right)))
        .chain((0..w * h + 1).step_by(w).skip(1).map(|c| c-1).zip(repeat(D::Left)))
}
```

