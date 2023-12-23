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
Overall we avoid taking a matrix approach emulating the input as this will result in very large arrays with mostly zeros. Instead, we use a simple vector of galaxies to perform the (a) expansion and (b) distance calculations
### Gap Identification
We know gaps can grow very large hence use of `Range`, like `(2..=3)`, is an economical way to represent & process gaps.
Therefore, gaps can be extracted by 
1. Collecting all `X` values into a sorted array. Similarly, for `Y` coordinates
2. Check the **distance delta** of each `X` **pair** in the array and if greater than `1` then save it as range

The below function performs step 2 by providing an Iterator over the array of X or Y values.
```rust
pub(crate) fn extract_gaps(seq: &Vec<usize>) -> impl Iterator<Item=RangeInclusive<usize>> + '_ {
    seq.windows(2)
        .filter_map(|pair| {
            let [a,b] = pair else { unreachable!() };
            let gap = b - a;
            if gap > 1 {
                Some(b - gap + 1 ..= *b - 1)
            } else {
                None
            }
        })
}
```
### Universe Expansion
The important consideration here is that with each expanding gap, all subsequent gaps and galaxies are moved out by **expansion multiples**. As a result
1. 1st gap pushes all subsequent galaxies and gaps by `expand`
2. 2nd gap pushes all subsequent galaxies and gaps by `expand`*`2`
3. 3rd gap pushes all subsequent galaxies and gaps by `expand`*`3`
4. etc

With the above in mind, calculating the new position per galaxy we run the following logic
1. For each `gap range` identified on `X` dimension and with `gap order`
    1. get range's `gap length`
    2. For each galaxy with `X` > `gap range end` + `expand` * (`gap order` - 1)
         1. Increment Galaxy's X by (`expand` * `gap length`)
    3. increase `gap order` by `gap length`

With
* `gap range`, a region of X or Y values with no galaxies 
* `expand`, the amount we expand the gap i.e. double is `+1`, tenfold is `+9`
* `gap order`, the gap's sequence order i.e. `1` if first, `2` if second, etc

Expanding by Y dimension follows the same logic
### Distance
The Manhattan Distance formula `|x2-x1| + |y2-y1` calculates the steps required to connect two points in a matrix
