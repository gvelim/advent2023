use std::str::FromStr;
use super::mapping::*;
use rayon::prelude::*;

#[derive(Debug,Hash,Eq,PartialEq,Copy, Clone)]
pub(crate) enum MapType {
    Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location
}

impl FromStr for MapType {
    type Err = String;

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
            _ => Err(format!("Cannot convert to MapType {s}"))
        }
    }
}

#[derive(Debug,PartialEq)]
pub(crate) struct Map {
    pub(crate) map: MapType,
    pub(crate) dest: MapType,
    pub(crate) mappings: Vec<Mapping>
}

impl Map {
    pub(crate) fn transform(&self, input: u64) -> (u64,MapType) {
        self.mappings.iter()
            .filter_map(|tx| {
                if tx.src_base.contains(&input) {
                    Some(tx.dst_base + input - tx.src_base.start)
                } else {
                    None
                }
            })
            .next()
            .and_then(|o| Some((o, self.dest)))
            .or_else(|| Some((input, self.dest)))
            .unwrap()
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = s.split("\n\n").next().unwrap().lines();

        let mut map_type = maps
            .next().unwrap()
            .split_whitespace()
            .next().unwrap()
            .split("-to-")
            .map(|map| map.parse::<MapType>().expect("map_type::Ops!"));

        Ok(Map {
            map: map_type.next().unwrap(),
            dest: map_type.next().unwrap(),
            mappings: maps
                .map(|m| m.parse::<Mapping>().expect("mapping::Ops"))
                .collect::<Vec<_>>()
        })
    }
}