use std::{ops::Range, rc::Rc, str::FromStr};
use super::mapping::*;
use super::error::*;

#[derive(Debug,Hash,Eq,PartialEq,Copy, Clone)]
pub(crate) enum MapType {
    Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location
}

impl FromStr for MapType {
    type Err = MapTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(MapType::Seed),
            "soil" => Ok(MapType::Soil),
            "fertilizer" => Ok(MapType::Fertilizer),
            "water" => Ok(MapType::Water),
            "light" => Ok(MapType::Light),
            "temperature" => Ok(MapType::Temperature),
            "humidity" => Ok(MapType::Humidity),
            "location" => Ok(MapType::Location),
            _ => Err(MapTypeError::UnknownMapType)
        }
    }
}

#[derive(Debug,PartialEq)]
pub(crate) struct Map {
    map: MapType,
    dest: MapType,
    mappings: Rc<[Mapping]>
}

impl Map {
    pub(crate) fn id(&self) -> MapType {
        self.map
    }
}

pub trait Transform<T> {
    fn transform(&self, seed: T) -> (T,MapType) where T: Clone;
}

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
            // flip/flop the pointers to the queues' memory allocation:
            // one is now empty and the other has the ranges for processing by the next mapping
            // so we avoid temporary vector and subsequenly heap allocation
            std::mem::swap::<Vec<Range<u64>>>(&mut flip, &mut flop);
            // println!("{:?}",(self.map, mapping,&queue1));
        }
        // add remaining residual ranges following the processing of all mappings
        flip.extend(out);

        (flip.into(), self.dest)
    }
}

impl FromStr for Map {
    type Err = MapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = s
            .split("\n\n")
            .next()
            .ok_or(MapError::ParseInputFormatInvalid)?
            .lines();

        let mut map_type = maps
            .next().ok_or(MapError::MissingMapType)?
            .split_whitespace()
            .next().ok_or(MapError::MissingMapType)?
            .split("-to-")
            .map(|map| map.parse::<MapType>());

        Ok(Map {
            map: map_type.next().ok_or(MapError::ParseInputFormatInvalid)??,
            dest: map_type.next().ok_or(MapError::ParseInputFormatInvalid)??,
            mappings: maps
                .map(|m| m.parse::<Mapping>())
                .collect::<Result<Rc<[_]>,MappingError>>()?
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Seeds;

    #[test]
    fn test_map_transform_ranges() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let mut split = input.split("\n\n");
        let seeds = split.next().unwrap().parse::<Seeds>().expect("Ops!");
        let map = split.next().unwrap().parse::<Map>().expect("Ops!");

        println!("{:?}",
            map.transform(seeds.get_ranges())
        );
    }

    #[test]
    fn test_map_transform() {
        let input = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let mut split = input.split("\n\n");
        let seeds = split.next().unwrap().parse::<Seeds>().expect("Ops!");
        let map = split.next().unwrap().parse::<Map>().expect("Ops!");

        seeds.iter()
            .inspect(|seed| print!("Input: {seed}"))
            .map(|&seed| map.transform(seed))
            .for_each(|d| println!(" -> {:?}",d));

        assert_eq!(
            seeds.iter().map(|&seed| map.transform(seed)).collect::<Vec<_>>(),
            [(81, MapType::Soil), (14, MapType::Soil), (57, MapType::Soil),(13, MapType::Soil)]
        )
    }

    #[test]
    fn test_parse_map() {
        let data = std::fs::read_to_string("./src/bin/day5/sample.txt").expect("Ops!");
        let input = data.split("\n\n").nth(1).unwrap();

        let map = input.parse::<Map>().expect("Map::Ops!");
        println!("{:?}",map);
        assert_eq!(
            map,
            Map { map: MapType::Seed, dest: MapType::Soil,
                    mappings: vec![
                        Mapping { src_base: 98..100, dst_base: 50 },
                        Mapping { src_base: 50..98, dst_base: 52 }
                    ].into()
                }
        )
    }
    #[test]
    fn test_parse_map_erros() {
        let input = [
        ("soul-to-fertilizer map:\n\
            39 0 15", MapError::InvalidMapType),
        ("fertilizer-to-water map:\n\
            57 7 A",MapError::InvalidMappingValues),
        ("fertilizer-to-water map:\n\
            57 4",MapError::InvalidMappingValues),
        ("fertilizer-too-water map:\n\
            57 4 9",MapError::InvalidMapType),
        ("fertilizer water map:\n\
            57 4",MapError::ParseInputFormatInvalid)
        ];

        for (test,err) in input {
            match test.parse::<Map>() {
                Ok(_) => panic!("Test case [{test:?}] should not succeed!"),
                Err(e) => {
                    println!("Received [{e:?}], Expected [{err:?}] in {test:}");
                    assert_eq!(e, err)
                }
            }
        }
    }
}
