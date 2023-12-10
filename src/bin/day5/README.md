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
The following data structure captures the behaviours. 
```
MapType { seed,soil, .. , humidity } <-- location doesn't have a map
Pipeline
   + HashMap( MapType, Map )
        +-- Map
            +--- name: MapType
            +--- next: MapType
            +--- Mappings: Vec<Mapping>
                    +--- inp_range: Range<u64>  <--- (src_base .. src_base + length)
                    +--- dst_offset: u64
        +-- Map
```
Pipeline has the `run(start: (u64,MapType))` 
1. which takes the seed and starting `MapType` i.e. for seed-to-soil is `seed`, 
2. extracts the seed map instance i.e. seed-to-soil, from the hashmap and 
3. runs the map's `map::transform(inp:u64)->(u64,MapType)` to produce the output and suitable `MapType`
4. repeat step 1 using the output of the first iteration until there is no target maptype, i.e. location 
