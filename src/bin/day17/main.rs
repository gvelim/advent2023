use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use std::str::FromStr;

fn main() {}

struct CityMap {
    map: Rc<[Rc<[u8]>]>,
}

impl FromStr for CityMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CityMap {
            map: s
                .lines()
                .map(|line| line.bytes().map(|c| c - b'0').collect::<Rc<[u8]>>())
                .collect::<Rc<_>>(),
        })
    }
}

impl Debug for CityMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.map.iter().for_each(|line| {
            writeln!(f, "{:?}", line).expect("Ops");
        });
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::CityMap;

    #[test]
    fn test_parse_map() {
        let input = std::fs::read_to_string("src/bin/day17/sample.txt").expect("File Not Found!");
        let map = input.parse::<CityMap>().expect("ops");

        assert_eq!(
            map.map,
            [
                [2, 4, 1, 3, 4, 3, 2, 3, 1, 1, 3, 2, 3].into(),
                [3, 2, 1, 5, 4, 5, 3, 5, 3, 5, 6, 2, 3].into(),
                [3, 2, 5, 5, 2, 4, 5, 6, 5, 4, 2, 5, 4].into(),
                [3, 4, 4, 6, 5, 8, 5, 8, 4, 5, 4, 5, 2].into(),
                [4, 5, 4, 6, 6, 5, 7, 8, 6, 7, 5, 3, 6].into(),
                [1, 4, 3, 8, 5, 9, 8, 7, 9, 8, 4, 5, 4].into(),
                [4, 4, 5, 7, 8, 7, 6, 9, 8, 7, 7, 6, 6].into(),
                [3, 6, 3, 7, 8, 7, 7, 9, 7, 9, 6, 5, 3].into(),
                [4, 6, 5, 4, 9, 6, 7, 9, 8, 6, 8, 8, 7].into(),
                [4, 5, 6, 4, 6, 7, 9, 9, 8, 6, 4, 5, 3].into(),
                [1, 2, 2, 4, 6, 8, 6, 8, 6, 5, 5, 6, 3].into(),
                [2, 5, 4, 6, 5, 4, 8, 8, 8, 7, 7, 3, 5].into(),
                [4, 3, 2, 2, 6, 7, 4, 6, 5, 5, 5, 3, 3].into()
            ].into()
        )
    }
}
