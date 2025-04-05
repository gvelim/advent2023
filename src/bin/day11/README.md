# Day 11: Cosmic Expansion

## Input
The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes **empty space** `.` and **galaxies** `#`. For example:
```
  Input           Gaps                Expanded Space
                 v  v  v               ....#........
...#......     ...#......              .........#...
.......#..     .......#..              #............
#.........     #.........              .............
..........    >..........<   Universe  .............
......#...     ......#...   Expansion  ........#....
.#........     .#........      =>      .#...........
.........#     .........#              ............#
..........    >..........<             .............
.......#..     .......#..              .............
#...#.....     #...#.....              .........#...
                                       #....#.......
```
## Part 1
The researcher is trying to figure out the **sum of the lengths of the shortest path between every pair of galaxies**. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, **only some space expands**. In fact, the result is that any rows or columns **that contain no galaxies** should all actually be twice as big.

Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
```
Galaxy { pos: (4, 0) } -> (9, 1):6,(0, 2):6,(8, 5):9,(1, 6):9,(12, 7):15,(9, 10):15,(0, 11):15,(5, 11):12, = Sum: 87,
Galaxy { pos: (9, 1) } -> (0, 2):10,(8, 5):5,(1, 6):13,(12, 7):9,(9, 10):9,(0, 11):19,(5, 11):14, = Sum: 79,
Galaxy { pos: (0, 2) } -> (8, 5):11,(1, 6):5,(12, 7):17,(9, 10):17,(0, 11):9,(5, 11):14, = Sum: 73,
Galaxy { pos: (8, 5) } -> (1, 6):8,(12, 7):6,(9, 10):6,(0, 11):14,(5, 11):9, = Sum: 43,
Galaxy { pos: (1, 6) } -> (12, 7):12,(9, 10):12,(0, 11):6,(5, 11):9, = Sum: 39,
Galaxy { pos: (12, 7) } -> (9, 10):6,(0, 11):16,(5, 11):11, = Sum: 33,
Galaxy { pos: (9, 10) } -> (0, 11):10,(5, 11):5, = Sum: 15,
Galaxy { pos: (0, 11) } -> (5, 11):5, = Sum: 5,
Galaxy { pos: (5, 11) } ->  = Sum: 0,

Sum of sortest paths = 374
```
## Part 2
Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?
```
Galaxy { pos: (1000002, 0) } -> (2000005, 1):1000004,(0, 2):1000004,(2000004, 1000003):2000005,(1, 1000004):2000005,(3000006, 1000005):3000009,(2000005, 2000006):3000009,(0, 2000007):3000009,(1000003, 2000007):2000008, = Sum: 17000053,
Galaxy { pos: (2000005, 1) } -> (0, 2):2000006,(2000004, 1000003):1000003,(1, 1000004):3000007,(3000006, 1000005):2000005,(2000005, 2000006):2000005,(0, 2000007):4000011,(1000003, 2000007):3000008, = Sum: 17000045,
Galaxy { pos: (0, 2) } -> (2000004, 1000003):3000005,(1, 1000004):1000003,(3000006, 1000005):4000009,(2000005, 2000006):4000009,(0, 2000007):2000005,(1000003, 2000007):3000008, = Sum: 17000039,
Galaxy { pos: (2000004, 1000003) } -> (1, 1000004):2000004,(3000006, 1000005):1000004,(2000005, 2000006):1000004,(0, 2000007):3000008,(1000003, 2000007):2000005, = Sum: 9000025,
Galaxy { pos: (1, 1000004) } -> (3000006, 1000005):3000006,(2000005, 2000006):3000006,(0, 2000007):1000004,(1000003, 2000007):2000005, = Sum: 9000021,
Galaxy { pos: (3000006, 1000005) } -> (2000005, 2000006):2000002,(0, 2000007):4000008,(1000003, 2000007):3000005, = Sum: 9000015,
Galaxy { pos: (2000005, 2000006) } -> (0, 2000007):2000006,(1000003, 2000007):1000003, = Sum: 3000009,
Galaxy { pos: (0, 2000007) } -> (1000003, 2000007):1000003, = Sum: 1000003,
Galaxy { pos: (1000003, 2000007) } ->  = Sum: 0,

Sum of sortest paths = 82000210
```
## Approach
Overall we avoid taking a matrix approach emulating the input as this will result in very large arrays with mostly zeros. Instead, we use a simple vector of galaxies to perform the (a) expansion and (b) distance calculations. This sparse representation is particularly critical for Part 2, where a 1,000,000x expansion would make a matrix approach impractical.

```rust
// Our core data structures
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Galaxy {
    pub(crate) pos: (usize, usize)
}

#[derive(Debug, PartialEq)]
pub(crate) struct Universe {
    pub(crate) clusters: Vec<Galaxy>
}
```

### Gap Identification
We know gaps can grow very large hence use of `Range`, like `(2..=3)`, is an economical way to represent & process gaps.
Therefore, gaps can be extracted by:
1. Collecting all `X` values into a sorted array. Similarly, for `Y` coordinates
2. Check the **distance delta** of each `X` **pair** in the array and if greater than `1` then save it as range

This approach allows us to work with potentially enormous gaps without explicitly storing every coordinate within them - essential for million-fold expansions.

The below function performs step 2 by providing an Iterator over the array of X or Y values:
```rust
pub(crate) fn extract_gaps(seq: &[usize]) -> impl Iterator<Item=RangeInclusive<usize>> + '_ {
    seq.windows(2)
        .filter_map(|pair| {
            if pair[1] - pair[0] > 1 {
                Some(pair[0] + 1 ..= pair[1] - 1)
            } else {
                None
            }
        })
}
```

The function uses the `.windows(2)` method which creates a sliding window of size 2 over the array, allowing us to examine pairs of adjacent values. When a gap is detected, we return it as an inclusive range.

### Universe Expansion
The important consideration here is that with each expanding gap, all subsequent gaps and galaxies are moved out by **expansion multiples**. As a result:
1. 1st gap pushes all subsequent galaxies and gaps by `expand`*`1`
2. 2nd gap pushes all subsequent galaxies and gaps by `expand`*`2`
3. 3rd gap pushes all subsequent galaxies and gaps by `expand`*`3`
4. etc

This cumulative effect is crucial - each gap not only shifts galaxies by its own expansion amount but also needs to account for all previous expansions.

First, we define methods to modify galaxy positions:

```rust
impl Galaxy {
    pub(crate) fn shift_by(&mut self, delta: (usize, usize)) {
        self.pos.0 += delta.0;
        self.pos.1 += delta.1;
    }
}
```

With the above in mind, calculating the new position per galaxy we run the following logic:
1. For each `gap range` identified on `X` dimension and with `gap order`
   1. get range's `gap length`
   2. For each galaxy with `X` > `gap range end` + `expand` * (`gap order` - 1)
      1. Increment Galaxy's X by (`expand` * `gap length`)
   3. increase `gap order` by `gap length`

With:
* `gap range`, a region of X or Y values with no galaxies
* `expand`, the amount we expand the gap i.e. double is `+1`, tenfold is `+9`
* `gap order`, the gap's sequence order i.e. `1` if first, `2` if second, etc

The complete expand method:
```rust
pub(crate) fn expand(&mut self, multiplier: usize) -> &Self {
    let expand = if multiplier > 1 { multiplier - 1 } else { 1 };

    let (mut x_gap, mut y_gap) = (vec![], vec![]);

    self.clusters.iter().for_each(|g| {
        x_gap.push(g.pos.0);
        y_gap.push(g.pos.1);
    });

    x_gap.sort();

    // Expand along x-axis
    let mut i = 0;
    Universe::extract_gaps(&x_gap)
        .for_each(|x| {
            let len = x.end() - x.start() + 1;
            self.clusters.iter_mut()
                .filter(|g| g.pos.0.gt(&(x.end() + i * expand)))
                .for_each(|g| {
                    g.shift_by((expand * len, 0));
                });
            i += len;
        });

    // Expand along y-axis
    i = 0;
    Universe::extract_gaps(&y_gap)
        .for_each(|y| {
            let len = y.end() - y.start() + 1;
            self.clusters.iter_mut()
                .filter(|g| g.pos.1.gt(&(y.end() + i * expand)))
                .for_each(|g|
                    g.shift_by((0, expand * len))
                );
            i += len;
        });

    self
}
```

The filter condition `g.pos.0 > gap_range.end() + gap_order * expand` ensures we only shift galaxies that are beyond the current gap, taking into account all previous expansions.

Expanding by Y dimension follows the same logic, operating on the Y coordinates instead.

### Distance between galaxies
The Manhattan Distance formula `|x2-x1| + |y2-y1|` calculates the steps required to connect two points in a matrix. This is appropriate for our problem because we can only move in four directions (up, down, left, right) through the grid. After expansion, we simply apply this formula to each pair of galaxies to find the shortest path between them.

```rust
impl Galaxy {
    pub(crate) fn distance_to(&self, dst: &Galaxy) -> usize {
        // Using the Manhattan distance formula
        dst.pos.0.abs_diff(self.pos.0) + dst.pos.1.abs_diff(self.pos.1)
    }
}
```

Finally, to solve both parts, we compute the sum of all galaxy pair distances:

```rust
fn run_part(universe: &mut Universe, multiplier: usize) -> usize {
    universe.expand(multiplier);
    universe.clusters
        .iter()
        .enumerate()
        .map(|(i, from)| {
            universe.clusters
                .iter()
                .skip(i + 1)  // Only count each pair once
                .map(|to| from.distance_to(to))
                .sum::<usize>()
        })
        .sum::<usize>()
}
```
