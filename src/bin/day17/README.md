# Day 17: Clumsy Crucible
## Input
You land near the gradually-filling pool of lava at the base of your new **lavafall**. Lavaducts will eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading it into large **crucibles** on wheels. The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult to steer at high speeds, and so it can be hard to go in a straight line for very long.

You'll need to find the best way to get the crucible **from the lava pool to the machine parts factory**. To do this, you need to **minimize heat loss** while choosing a route that doesn't require the crucible to go in a straight line for too long

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient temperature, and hundreds of other parameters to calculate exactly how much heat loss can be expected for a crucible entering any particular city block.

For example:
```
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
```
## Part 1
Each city block is marked by a single digit that represents the **amount of heat loss if the crucible enters that block**. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, **it can move at most three blocks** in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

One way to minimize heat loss is this path:
```
▶ 2/0  :0 |▶ 4/4  :1 |▶ 1/5  :2 |  3/0  :0 |  4/0  :0 |▲ 3/23 :1 |▶ 2/25 :1 |▶ 3/28 :2 |▶ 1/29 :3 |  1/0  :0 |  3/0  :0 |  2/0  :0 |  3/0  :0 |
  3/0  :0 |  2/0  :0 |▼ 1/6  :1 |▶ 5/11 :1 |▶ 4/15 :2 |▶ 5/20 :3 |  3/0  :0 |  5/0  :0 |▼ 3/32 :1 |▶ 5/37 :1 |  6/0  :0 |  2/0  :0 |  3/0  :0 |
  3/0  :0 |  2/0  :0 |  5/0  :0 |  5/0  :0 |  2/0  :0 |  4/0  :0 |  5/0  :0 |  6/0  :0 |  5/0  :0 |▼ 4/41 :1 |▶ 2/43 :1 |  5/0  :0 |  4/0  :0 |
  3/0  :0 |  4/0  :0 |  4/0  :0 |  6/0  :0 |  5/0  :0 |  8/0  :0 |  5/0  :0 |  8/0  :0 |  4/0  :0 |  5/0  :0 |▼ 4/47 :1 |  5/0  :0 |  2/0  :0 |
  4/0  :0 |  5/0  :0 |  4/0  :0 |  6/0  :0 |  6/0  :0 |  5/0  :0 |  7/0  :0 |  8/0  :0 |  6/0  :0 |  7/0  :0 |▼ 5/52 :2 |▶ 3/55 :1 |  6/0  :0 |
  1/0  :0 |  4/0  :0 |  3/0  :0 |  8/0  :0 |  5/0  :0 |  9/0  :0 |  8/0  :0 |  7/0  :0 |  9/0  :0 |  8/0  :0 |  4/0  :0 |▼ 5/60 :1 |  4/0  :0 |
  4/0  :0 |  4/0  :0 |  5/0  :0 |  7/0  :0 |  8/0  :0 |  7/0  :0 |  6/0  :0 |  9/0  :0 |  8/0  :0 |  7/0  :0 |  7/0  :0 |▼ 6/66 :2 |  6/0  :0 |
  3/0  :0 |  6/0  :0 |  3/0  :0 |  7/0  :0 |  8/0  :0 |  7/0  :0 |  7/0  :0 |  9/0  :0 |  7/0  :0 |  9/0  :0 |  6/0  :0 |▼ 5/71 :3 |▶ 3/74 :1 |
  4/0  :0 |  6/0  :0 |  5/0  :0 |  4/0  :0 |  9/0  :0 |  6/0  :0 |  7/0  :0 |  9/0  :0 |  8/0  :0 |  6/0  :0 |  8/0  :0 |  8/0  :0 |▼ 7/81 :1 |
  4/0  :0 |  5/0  :0 |  6/0  :0 |  4/0  :0 |  6/0  :0 |  7/0  :0 |  9/0  :0 |  9/0  :0 |  8/0  :0 |  6/0  :0 |  4/0  :0 |  5/0  :0 |▼ 3/84 :2 |
  1/0  :0 |  2/0  :0 |  2/0  :0 |  4/0  :0 |  6/0  :0 |  8/0  :0 |  6/0  :0 |  8/0  :0 |  6/0  :0 |  5/0  :0 |  5/0  :0 |◀ 6/93 :1 |▼ 3/87 :3 |
  2/0  :0 |  5/0  :0 |  4/0  :0 |  6/0  :0 |  5/0  :0 |  4/0  :0 |  8/0  :0 |  8/0  :0 |  8/0  :0 |  7/0  :0 |  7/0  :0 |▼ 3/96 :1 |  5/0  :0 |
  4/0  :0 |  3/0  :0 |  2/0  :0 |  2/0  :0 |  6/0  :0 |  7/0  :0 |  4/0  :0 |  6/0  :0 |  5/0  :0 |  5/0  :0 |  5/0  :0 |▼ 3/99 :2 |▶ 3/102:1 |
```
This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only `102`.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, **what is the least heat loss it can incur?**
## Part 2
Elves are going to upgrade to **ultra crucibles**. Once an ultra crucible starts moving in a direction, it needs to move **a minimum of four blocks** in that direction before it can turn (or even before it can stop at the end). However, it will eventually start to get wobbly: an ultra crucible can move **a maximum of ten consecutive blocks** without turning

In the above example, an ultra crucible could follow this path to minimize heat loss:
```
▶ 2/0  : 0 |▶ 4/4  : 1 |▶ 1/5  : 2 |▶ 3/8  : 3 |▶ 4/12 : 4 |▶ 3/15 : 5 |▶ 2/17 : 6 |▶ 3/20 : 7 |▶ 1/21 : 8 |  1/0  : 0 |  3/0  : 0 |  2/0  : 0 |  3/0  : 0 |
  3/0  : 0 |  2/0  : 0 |  1/0  : 0 |  5/0  : 0 |  4/0  : 0 |  5/0  : 0 |  3/0  : 0 |  5/0  : 0 |▼ 3/24 : 1 |  5/0  : 0 |  6/0  : 0 |  2/0  : 0 |  3/0  : 0 |
  3/0  : 0 |  2/0  : 0 |  5/0  : 0 |  5/0  : 0 |  2/0  : 0 |  4/0  : 0 |  5/0  : 0 |  6/0  : 0 |▼ 5/29 : 2 |  4/0  : 0 |  2/0  : 0 |  5/0  : 0 |  4/0  : 0 |
  3/0  : 0 |  4/0  : 0 |  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  8/0  : 0 |  5/0  : 0 |  8/0  : 0 |▼ 4/33 : 3 |  5/0  : 0 |  4/0  : 0 |  5/0  : 0 |  2/0  : 0 |
  4/0  : 0 |  5/0  : 0 |  4/0  : 0 |  6/0  : 0 |  6/0  : 0 |  5/0  : 0 |  7/0  : 0 |  8/0  : 0 |▼ 6/39 : 4 |▶ 7/46 : 1 |▶ 5/51 : 2 |▶ 3/54 : 3 |▶ 6/60 : 4 |
  1/0  : 0 |  4/0  : 0 |  3/0  : 0 |  8/0  : 0 |  5/0  : 0 |  9/0  : 0 |  8/0  : 0 |  7/0  : 0 |  9/0  : 0 |  8/0  : 0 |  4/0  : 0 |  5/0  : 0 |▼ 4/64 : 1 |
  4/0  : 0 |  4/0  : 0 |  5/0  : 0 |  7/0  : 0 |  8/0  : 0 |  7/0  : 0 |  6/0  : 0 |  9/0  : 0 |  8/0  : 0 |  7/0  : 0 |  7/0  : 0 |  6/0  : 0 |▼ 6/70 : 2 |
  3/0  : 0 |  6/0  : 0 |  3/0  : 0 |  7/0  : 0 |  8/0  : 0 |  7/0  : 0 |  7/0  : 0 |  9/0  : 0 |  7/0  : 0 |  9/0  : 0 |  6/0  : 0 |  5/0  : 0 |▼ 3/73 : 3 |
  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  4/0  : 0 |  9/0  : 0 |  6/0  : 0 |  7/0  : 0 |  9/0  : 0 |  8/0  : 0 |  6/0  : 0 |  8/0  : 0 |  8/0  : 0 |▼ 7/80 : 4 |
  4/0  : 0 |  5/0  : 0 |  6/0  : 0 |  4/0  : 0 |  6/0  : 0 |  7/0  : 0 |  9/0  : 0 |  9/0  : 0 |  8/0  : 0 |  6/0  : 0 |  4/0  : 0 |  5/0  : 0 |▼ 3/83 : 5 |
  1/0  : 0 |  2/0  : 0 |  2/0  : 0 |  4/0  : 0 |  6/0  : 0 |  8/0  : 0 |  6/0  : 0 |  8/0  : 0 |  6/0  : 0 |  5/0  : 0 |  5/0  : 0 |  6/0  : 0 |▼ 3/86 : 6 |
  2/0  : 0 |  5/0  : 0 |  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  4/0  : 0 |  8/0  : 0 |  8/0  : 0 |  8/0  : 0 |  7/0  : 0 |  7/0  : 0 |  3/0  : 0 |▼ 5/91 : 7 |
  4/0  : 0 |  3/0  : 0 |  2/0  : 0 |  2/0  : 0 |  6/0  : 0 |  7/0  : 0 |  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  5/0  : 0 |  5/0  : 0 |  3/0  : 0 |▼ 3/94 : 
  ```
In the above example, an ultra crucible would incur the minimum possible heat loss of `94`.

Directing the ultra crucible from the lava pool to the machine parts factory, **what is the least heat loss it can incur**?

## Approach
### Navigating the CityMap
For each `CityBlock` that we are stepping onto, the decision on **"where can we move next?"** is dictated by,
1. **Current position** and available moves, especially around edges & corners 
2. **Vector of direction**: either turn left, turn right or move in same direction
3. **Steps in same direction**: no more than 3 consecutive steps

The above information is encapsulated by the below tuple structure
```rust
struct CityBlock( Position, Direction, Step);
```

The **position constraint** is implemented by the `move_from()` function, that based on a `Vector<CityBlocks>`, it  maps a `(Position,Direction)` tuple into the next valid CityBlock position (vector index) or `None` if there is no valid position.
```rust
fn move_from(&self, from: Position, dir: Direction) -> Option<Position> {
    if from >= self.map.len() { return None }
    match dir {
        D::Right if from % self.width < self.width-1 => Some(from + 1),
        D::Left if from % self.width > 0 => Some(from - 1),
        D::Up if from > self.width - 1 => Some(from - self.width),
        D::Down if from < self.map.len() - self.width => Some(from + self.width),
        _ => None
    }
}
```
The **direction constrain** is implemented by the `directions()` function which provides the applicable directions given the current direction vector 
```rust
pub(crate) enum Direction { Up,  Right, Down, Left }

impl Direction {
    fn directions(&self) -> impl Iterator<Item=Direction> + 'static {
        use Direction as D;
        match self{
            D::Up => [D::Up, D::Left, D::Right],
            D::Right => [D::Right, D::Up, D::Down],
            D::Down => [D::Down, D::Left, D::Right],
            D::Left => [D::Left, D::Up, D::Down],
        }.into_iter()
    }
}
```
Therefore, at a given CityBlock, finding all **valid** neighbour CityBlocks that we can **step onto**, is expressed by the below function. The function returns an `Iterator` that **iterates along the remaining valid directions** and outputs the valid CityBlocks that we can step onto, therefore complying to all three movement constraints.
```rust
fn neighbour_blocks(&self, current: CityBlock, rng: &Range<usize>) -> impl Iterator<Item=CityBlock> + '_ {
    let CityBlock(pos, dir, step) = current;
    dir.directions()
        // drop same direction vector if we have already moved max number of steps e.g. 3
        .filter(move |d| step < rng.end || dir.ne(d) )
        // extract CityBlocks that are valid moves
        .filter_map(move |d|
            self.citymap.move_from(pos, d)
                .map(|p|
                    CityBlock(p, d, if d == dir {step + 1} else { 1 })
                )
        )
}
```
## Finding the path with the minimum heat loss
To find out the path with the **minimum heat loss** across any two `CityBlocks`, will require to explore all possible paths combinations connecting the two `CityBlocks`. Hence, it is **critical** to understand how we calculate the path cost `C` at a path position `P`.

We already know that a `Crucible's` move is dependent on 3 key constraints
1. Current Position
2. Current Direction vector
3. Number of Steps at current position

Therefore, at a certain position i.e. (10,10), we may have incurred different costs depending on constraints 2 & 3, for example
1. (10,10), arrived from Left, and it is the 3rd step left, hence we have to take a turn
2. (10,10), arrived from Up, and it is the 1st step downwards, hence we can move further down or turn
3. (10,10), arrived from Down, and it is the 1st step upwards, hence we can move further up or turn
4. etc

Therefore, the cost at Position `P` is defined in terms of all the previous/past Blocks with Position `P`, Direction `D` and Step `S` leading to the position `P`. Hence, the path cost at any specific position P is
```
Cost at path position P =  Cost at P + Parent_Cost( Position, Direction, Step )
with Parent_Cost expressed by the same formula recursively.
```
With the above definition for every step in the path, we can use the [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm) to calculate the series of `( Position, Direction, Step )` steps the form the path with the least heat loss.

Hence in our case the below algorithm will
1. Push in the priority `queue` the starting CityBlock with Heat loss equal to zero
2. Pull from the priority `queue` the CityBlock with the lowest accumulated heat loss
3. If the CityBlock pulled is at the target position then return the cumulative heat loss and exit  
3. Otherwise, for each `neighbour()` CityBlock to current; pulled from the queue
   1. Sum up current and neighnour's heat losses 
   2. if the heat loss `sum()` is lower than anything previously calculated against the neighbour, then 
      1. Store heat loss `sum()` in the `heat_map` hashmap against the neighbour
      2. Push neighbour CityBlock and heat loss `sum()` to the priority queue for further exploration
3. Repeat step 2 until no more CityBlocks left in the queue for processing

```rust
fn find_path_to(&mut self, target: Position, rng: Range<usize>) -> Option<CityMapPath> {
   let mut cost_map = HashMap::<CityBlock,(Heat, Option<CityBlock>)>::new();
   let mut queue = BinaryHeap::<QueuedCityBlock>::new();
   // push starting conditions of start position, direction, zero heat, zero steps
   queue.push( QueuedCityBlock(0, CityBlock(self.pos, self.dir, 0)) );
   cost_map.insert(CityBlock(self.pos, self.dir, 0), (0, None));
   // pull the block with the least heat cost from the queue
   while let Some(QueuedCityBlock(heat, block)) = queue.pop() {
      // is this block our target ?
      if block.0 == target {
         // yes, return the path cost map with the starting block for traversing it
         return Some(CityMapPath::new(cost_map, block))
      }
      // get all feasible neighbouring blocks given the movement constraints
      self.neighbour_blocks(block, &rng)
              .for_each(|neighbour| {
                 // calculate cost if we are to move to this neighbour
                 let heat_sum = heat + self.cmap[neighbour.0];
                 // is the cost higher than previously found ? if not, store it
                 if heat_sum < cost_map.get(&neighbour).unwrap_or(&(Heat::MAX, None)).0 {
                    // remember the heat cost at this block along the block we stepped from
                    cost_map.insert(neighbour, (heat_sum, Some(block)));
                    // push neighbouring block to priority queue for processing
                    queue.push(QueuedCityBlock(heat_sum, neighbour));
                 }
              });
   }
   None
}
```
## Part 2
Part 2 alters the step constrains by enforcing
1. to move **a minimum of four blocks** when start moving in a new direction, and before it can turn (or even before it can stop at the end). 
2. to move **a maximum of ten consecutive blocks** in one direction

Therefore the only adjustment that we need to do here is to the `neighbour_blocks()` where we 
* only enforce same direction for CityBlocks with `step < 4`
* stop same direction move for CityBlocks with `step > 10`
```rust
fn neighbour_blocks(&self, node: CityBlock, rng: &Range<usize>) -> impl Iterator<Item=CityBlock> + '_ {
    let CityBlock(pos, dir, step) = node;
    dir.directions()
        // if step < min then move same direction otherwise move all directions
        .filter(move |d|  dir.eq(d) || step >= rng.start)
        // if step == max direction then drop same direction
        .filter(move |d| step < rng.end || dir.ne(d) )
        // extract CityBlocks that we can move onto
        .filter_map(move |d|
            self.cmap.move_from(pos, d)
                .map(|p|
                    CityBlock(p, d, if d == dir {step + 1} else { 1 })
                )
        ) 
}

```
