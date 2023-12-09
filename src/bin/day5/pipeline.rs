use std::collections::HashMap;
use std::str::FromStr;

use super::map::*;

pub(crate) struct Pipeline {
    maps: HashMap<MapType,Map>
}
impl Pipeline {
    pub(crate) fn run(&self, start: (u32, MapType)) -> u32 {
        let (mut out, mut next) = start;
        loop {
            if self.maps.contains_key(&next) {
                print!("({:?})->", (out, next));
                (out, next) = self.maps
                    .get(&next).unwrap()
                    .transform(out);
                println!("({:?})", (out, next));
            } else {
                println!();
                break out
            }
        }
    }
}

impl FromStr for Pipeline {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split = input.split("\n\n").skip(1);
        Ok(
            Pipeline {
                maps: split
                    .into_iter()
                    .map(|map| map.parse::<Map>().expect("Ops!"))
                    .map(|map| (map.map, map))
                    .collect::<HashMap<MapType,Map>>()
            }
        )
    }
}
