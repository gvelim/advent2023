use std::collections::HashMap;
use std::str::FromStr;

use super::map::*;

pub(crate) struct Pipeline {
    maps: HashMap<MapType,Map>
}
impl Pipeline {
    pub(crate) fn run(&self, start: (u64, MapType)) -> u64 {
        let (mut out, mut next) = start;
        // print!("{:?}->",(out,next));
        loop {
            if self.maps.contains_key(&next) {
                (out, next) = self.maps
                    .get(&next).unwrap()
                    .transform(out);
                // print!("{:?}->",(out,next));
            } else {
                // println!("Finished = {:?}",(out,next));
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
