# Day 5

## Input
The almanac (your puzzle input) lists all of the seeds that need to be planted. It also contains a list of maps which describe how to convert numbers from a `src` category into numbers in a `dest` category

```
seeds: 79 14 55 13      <--- seeds that need to be planted

seed-to-soil map:       <--- maps how to convert a seed number to a soil number
50 98 2                 <--- `dest` range start, the `src` range start, and the range length.
52 50 48                <--- Input Range (50..=97), Output Range(52..=99)

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

```
## Part 1: Output
Using these maps, find the lowest location number that corresponds to any of the initial seeds
```
(79, Seed)->(81, Soil)->(81, Fertilizer)->(81, Water)->(74, Light)->(78, Temperature)->(78, Humidity)->(82, Location)->Finished = (82, Location)
(14, Seed)->(14, Soil)->(53, Fertilizer)->(49, Water)->(42, Light)->(42, Temperature)->(43, Humidity)->(43, Location)->Finished = (43, Location)
(55, Seed)->(57, Soil)->(57, Fertilizer)->(53, Water)->(46, Light)->(82, Temperature)->(82, Humidity)->(86, Location)->Finished = (86, Location)
(13, Seed)->(13, Soil)->(52, Fertilizer)->(41, Water)->(34, Light)->(34, Temperature)->(35, Humidity)->(35, Location)->Finished = (35, Location)
Min = 35
```
## Part 2: Output
Repeat Part 1 however the seeds line now actually describes ranges of seed numbers, e.g. `seeds: 79 14 55 13` has two ranges, `(79..=92)` and `(55..=67)`

## Approach
We know that
1. we have a number of map types
2. each map
   1. has a name
   2. links to the next map in the processing sequence
   3. has 1 or more mappings
      1. each mapping translates a number within the base range to a target range

The following data structures capture the behaviour.
``` rust
enum MapType {
    Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location
}

struct Mapping {
    src_base: Range<u64>, // 98..100 from base: 98, len: 2
    dst_base: u64,        // 52: dst base: 52
}

struct Map {
    map: MapType,
    dest: MapType,
    mappings: Rc<[Mapping]>
}

struct Pipeline {
    maps: HashMap<MapType,Map>
}
```
## Part 1
When a seed is fed onto a map the following steps app
1. for each mapping
    1. does the `seed` falls into the `src` range ?
        1. yes, then convert it to dst value and return it along with the `name` of next map
        2. no, check seed against next mapping until no mappings remain
2. If no mapping matched then pass seed to the next map

The below `Map::transform()` performs the above logic
```rust
impl Mapping {
    fn shift(&self, n:u64) ->u64 {
        self.dst_base + n - self.src_base.start
    }
    fn transform(&self, seed: u64) -> Option<u64> {
        if self.src_base.contains(&seed) {
            Some(self.shift(seed))
        } else {
            None
        }
    }
...
}

pub trait MapTransform<T> {
    fn transform(&self, seed: T) -> (T,MapType) where T: Clone;
}

impl MapTransform<u64> for Map {
    fn transform(&self, seed: u64) -> (u64,MapType) where u64: Clone {
        self.mappings.iter()
            .filter_map(|mapping| mapping.transform(seed))
            .map(|seed| (seed, self.dest))
            .next()
            .unwrap_or( (seed, self.dest))
    }
}
```
With the map logic in place, the `Pipeline::run()` will 
1. Feed the seed against the starting map type
2. receive the value and next map name
3. repeat (1) until the next map name is None (doesn't exist) and the final value is received
    1. `Location` isn't a defined map
    2. Hence when next map type becomes `Location` this will terminate the loop hence we have the final value

```rust
trait PipelineRun<T> {
    fn run(&self, seed: T, map_type: MapType) -> T;
}

impl PipelineRun<u64> for Pipeline {
    fn run(&self, seed: u64, mut map_type: MapType) -> u64 {
        let mut out = seed;

        while let Some(map) = self.maps.get(&map_type) {
             (out, map_type) = map.transform(out);
        }
        out
    }
}
```
Answering Part 1 is given by the below logic, given seeds is a vector of `int` values.
```rust
let min = seeds
    .iter()
    .map(|&seed| pipeline.run(seed, MapType::Seed))
    .min();

```
## Part 2
Part 2 becomes trickier as the above implementation has `O(seeds * MapTypes * Mappings)` therefore it will take a lot computing time to complete.

An alternative approach here is to enhance the `Mapping` logic to transform `src` ranges to `dst` ranges. This will require us to understand the **`range` vs `mapping` transformation**. This is explained in the below picture
```
Mapping (M)   ----------XXXXXXXXXXXX---------
Range 1 (R1)     xxxxx
    M*R1         xxxxx  <- unmapped range 
Range 2 (R2)        xxxxxxx
    M*R2            xxxx    <- src unmapped part (residual)
                        TTT <- dst mapped part
Range 3 (R3)        xxxxxxxxxxxxxxxxxxxx
    M*R3            xxxxx                   <- src unmapped part (residual)
                         TTTTTTTTTTTT       <- dst mapped part
                                     xxx    <- src unmapped part (residual)
```
With the above intuition we implement the `Mapping::transform_range()` that given an input range it returns 
1. the transformed part
2. the residual, non-transformed part

```rust
enum RangeResidue {
    None,
    Single(Range<u64>),
    Double(Range<u64>,Range<u64>)
}
impl Mapping {
...
    pub(crate) fn transform_range(&self, rng: &Range<u64>) -> (Option<Range<u64>>,RangeResidue) {
        let src = &self.src_base;
        match (src.contains(&rng.start), src.contains(&(rng.end-1))) {
            (true, true) =>
                (Some(self.shift(rng.start)..self.shift(rng.end)), RangeResidue::None),
            (true, false) =>
                (Some(self.shift(rng.start)..self.shift(src.end)), RangeResidue::Single(src.end..rng.end)),
            (false, true) =>
                (Some(self.shift(src.start)..self.shift(rng.end)), RangeResidue::Single(rng.start..src.start)),
            (false, false) =>{
                if rng.end <= src.start || rng.start >= src.end {
                    (None, RangeResidue::Single(rng.clone()))
                } else {
                    (Some(self.shift(src.start)..self.shift(src.end)),
                        RangeResidue::Double(rng.start..src.start,src.end..rng.end))
                }
            }
        }
    }
}
```
Now, a `Map` applies multiple `Mappings` to an input `range` and here we need to understand how the **output** from one `Mapping` affects the **input** of the subsequent `Mapping`

```
            Input Range processing Queue       Map Output Queue
            ===============================  =======================
Inp: Range        xxxxxxxxxxxxxxxxxxxx

Mapping 1   ----------XXXXXXXXXXXX---------
Out: Range        xxxx            xxxx       TTTTTTTTTTTT
Mapping 2   -------XXXXX-----XXXXXXX-------
Out: Range        x                 xx       TTT, TT
--------------------------------------------------------------------
Result/Output : [ x, xx, TTTTTTTTTTTT, TTT, TT ]
```
Hence here we see that a transformed part **should never** be fed a subsequent mapping as this invalidates the processing rules.

Hence the `Map::transform()` takes an vector of ranges and returns the mapping results along with the name of the next map
```rust

impl MapTransform<Rc<[Range<u64>]>> for Map {
    fn transform(&self, seeds: Rc<[Range<u64>]>) -> (Rc<[Range<u64>]>,MapType) {
        let mut queue1: Vec<Range<u64>> = seeds.as_ref().into();
        let mut queue2 = Vec::with_capacity(seeds.len()*2);
        let mut out = Vec::with_capacity(seeds.len());

        for mapping in self.mappings.iter() {
            while let Some(rng) = queue1.pop() {
                // map input range into mapped and residual range(s)
                let (mapped, residual) = mapping.transform_range(&rng);
                // push mapped range to the output
                mapped.map(|r| out.push(r));
                // push residual to the queue for processing by subsequent mappings
                match residual {
                    RangeResidue::Single(a) => queue2.push(a),
                    RangeResidue::Double(a, b) => queue2.extend([a,b]),
                    _ => (),
                }
            }
            // flip/flop the pointers to the queues' memory allocation:
            // one is now empty and the other has the ranges for processing by the next mapping
            // so we avoid temporary vector and subsequenly heap allocation
            std::mem::swap::<Vec<Range<u64>>>(&mut queue1, &mut queue2);
            // println!("{:?}",(self.map, mapping,&queue1));
        }
        // add remaining residual ranges following the processing of all mappings
        queue1.extend(out);

        (queue1.into(), self.dest)
    }
}
```
The Pipeline logic remains nearly identical and returns the final vector of processed ranges
```rust
impl PipelineRun<Rc<[Range<u64>]>> for Pipeline {
    fn run(&self, seeds: Rc<[Range<u64>]>, mut map_type: MapType) -> Rc<[Range<u64>]> {
        let mut out: Rc<[Range<u64>]> = seeds.as_ref().into();

        while let Some(map) = self.maps.get(&map_type) {
             (out, map_type) = map.transform(out);
        }
        out
    }
}
```
Finding the minimum value becomes an exercise to find the range with the smallest starting value
```rust
let ranges = pipeline.run(seeds.get_ranges(), MapType::Seed);
let min = ranges
    .into_iter()
    .min_by_key(|r| r.start)
    .unwrap();

```
