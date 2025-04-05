# Day 5: If You Give A Seed A Fertilizer

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

## Understanding the Problem

The challenge asks us to create a pipeline of transformations where seeds go through multiple mapping stages to determine their final locations.

### Core Concepts

1. **Seeds**: Starting values that we need to trace through the transformation pipeline
2. **Maps**: Each map represents a transformation stage (e.g., seed-to-soil, soil-to-fertilizer)
3. **Mappings**: Within each map, there are specific range mappings that tell us how to transform values

## Solution Design

### Step 1: Modeling the Problem

First, let's define the fundamental data structures to represent the problem:

```rust
// Represents the different types of categories in our transformation pipeline
enum MapType {
    Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location
}

// Represents a single mapping rule within a map
struct Mapping {
    src_base: Range<u64>, // Source range (e.g., 98..100)
    dst_base: u64,        // Destination base value (e.g., 52)
}

// Represents a complete map with all its mapping rules
struct Map {
    map: MapType,         // Source map type
    dest: MapType,        // Destination map type
    mappings: Rc<[Mapping]> // Collection of mapping rules
}

// Represents the entire pipeline of transformations
struct Pipeline {
    maps: HashMap<MapType,Map>
}
```

### Step 2: Single Value Transformation

For Part 1, we need to trace individual seed values through the pipeline. Let's implement the transformation logic:

```rust
impl Mapping {
    // Calculate the destination value from a source value
    fn shift(&self, n:u64) ->u64 {
        self.dst_base + n - self.src_base.start
    }

    // Transform a single value if it falls within this mapping's range
    fn transform(&self, seed: u64) -> Option<u64> {
        if self.src_base.contains(&seed) {
            Some(self.shift(seed))
        } else {
            None
        }
    }
}

// Define a trait for transformations
pub trait Transform<T> {
    fn transform(&self, seed: T) -> (T,MapType) where T: Clone;
}

// Implement the transformation for single values
impl Transform<u64> for Map {
    fn transform(&self, seed: u64) -> (u64,MapType) {
        self.mappings
            .iter()
            .filter_map(|mapping| mapping.transform(seed))
            .map(|seed| (seed, self.dest))
            .next()
            .unwrap_or( (seed, self.dest))
    }
}
```

This implementation:
1. Checks if a seed falls within any mapping's source range
2. If it does, transforms the seed using the mapping rule
3. If not, the seed value remains unchanged
4. Returns both the transformed value and the next map type

### Step 3: Creating the Pipeline

Now we implement the pipeline that processes a seed through all maps:

```rust
trait Run<T> {
    fn run(&self, seed: T, map_type: MapType) -> T;
}

impl Run<u64> for Pipeline {
    fn run(&self, seed: u64, mut map_type: MapType) -> u64 {
        let mut out = seed;

        while let Some(map) = self.maps.get(&map_type) {
             (out, map_type) = map.transform(out);
        }
        out
    }
}
```

This continues transforming the value until we reach a map type that doesn't exist in our pipeline (which will be `Location`).

### Step 4: Solving Part 1

With our pipeline implemented, we can solve Part 1:

```rust
let min = seeds.iter()
    .map(|&seed| pipeline.run(seed, MapType::Seed))
    .min();
```

This code:
1. Takes each seed value
2. Runs it through the entire pipeline
3. Finds the minimum resulting location value

## Optimizing for Part 2

Part 2 introduces a significant challenge: instead of individual seed values, we now have ranges of seeds. Processing each value individually would be inefficient.

### Step 5: Range-Based Transformation

We need to transform entire ranges at once. Let's understand how ranges interact with mappings:

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

To handle this efficiently, we'll implement range transformation:

```rust
// Represents residual (unmapped) parts of a range
enum RangeResidue {
    None,
    Single(Range<u64>),
    Double(Range<u64>,Range<u64>)
}

impl Mapping {
    // Transform an entire range, returning both the mapped portion and residual portions
    pub(crate) fn transform_range(&self, rng: &Range<u64>) -> (Option<Range<u64>>,RangeResidue) {
        let src = &self.src_base;
        match (src.contains(&rng.start), src.contains(&(rng.end-1))) {
            (true, true) =>
                // Range fully contained in mapping
                (Some(self.shift(rng.start)..self.shift(rng.end)), RangeResidue::None),
            (true, false) =>
                // Range starts in mapping but extends beyond
                (Some(self.shift(rng.start)..self.shift(src.end)), RangeResidue::Single(src.end..rng.end)),
            (false, true) =>
                // Range ends in mapping but starts before
                (Some(self.shift(src.start)..self.shift(rng.end)), RangeResidue::Single(rng.start..src.start)),
            (false, false) =>{
                if rng.end <= src.start || rng.start >= src.end {
                    // Range completely outside mapping
                    (None, RangeResidue::Single(rng.clone()))
                } else {
                    // Range overlaps mapping in the middle
                    (Some(self.shift(src.start)..self.shift(src.end)),
                        RangeResidue::Double(rng.start..src.start,src.end..rng.end))
                }
            }
        }
    }
}
```

### Step 6: Processing Multiple Ranges

Now we need to handle multiple ranges through a map with multiple mappings:

```rust
impl Transform<Rc<[Range<u64>]>> for Map {
    fn transform(&self, seeds: Rc<[Range<u64>]>) -> (Rc<[Range<u64>]>,MapType) {
        let mut flip: Vec<Range<u64>> = seeds.as_ref().into();
        let mut flop = Vec::with_capacity(seeds.len()*2);
        let mut out = Vec::with_capacity(seeds.len());

        for mapping in self.mappings.iter() {
            while let Some(rng) = flip.pop() {
                // map input range into mapped and residual range(s)
                let (mapped, residual) = mapping.transform_range(&rng);
                // push mapped range to the output
                if let Some(r) = mapped { out.push(r) };
                // push residual to the queue for processing by subsequent mappings
                match residual {
                    RangeResidue::Single(a) => flop.push(a),
                    RangeResidue::Double(a, b) => flop.extend([a,b]),
                    _ => (),
                }
            }
            // flip/flop the pointers to the queues' memory allocation
            std::mem::swap::<Vec<Range<u64>>>(&mut flip, &mut flop);
        }
        // add remaining residual ranges following the processing of all mappings
        flip.extend(out);

        (flip.into(), self.dest)
    }
}
```

This clever implementation:
1. Uses two queues (flip/flop) to process ranges through mappings
2. Separates ranges into mapped portions (which go directly to output) and residual portions (which need further processing)
3. Efficiently reuses memory allocations for better performance

### Step 7: Enhancing the Pipeline for Ranges

Finally, we extend our pipeline to handle ranges:

```rust
impl Run<Rc<[Range<u64>]>> for Pipeline {
    fn run(&self, seeds: Rc<[Range<u64>]>, mut map_type: MapType) -> Rc<[Range<u64>]> {
        let mut out = seeds.clone();

        while let Some(map) = self.maps.get(&map_type) {
             (out, map_type) = map.transform(out);
        }
        out
    }
}
```

### Step 8: Solving Part 2

With our range-based transformation in place, we can solve Part 2:

```rust
let ranges = pipeline.run(seeds.get_ranges(), MapType::Seed);
let min = ranges
    .iter()
    .min_by_key(|r| r.start)
    .unwrap();
```

Now instead of processing billions of individual seed values, we efficiently process ranges of values, making the solution performant.

## Conclusion

This solution demonstrates powerful programming principles:
1. **Abstraction**: Using traits and enums to model the problem domain
2. **Efficiency**: Processing ranges instead of individual values
3. **Memory management**: Carefully managing allocations with flip/flop queues
4. **Algorithm design**: Breaking down a complex problem into manageable components

By implementing range-based transformations, we achieve an elegant solution that handles the complexity of part 2 without sacrificing performance.
