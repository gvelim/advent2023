
mod engine;

use crate::engine::*;

fn main() {

}

mod test {
    use super::*;

    static INPUT: &str =
        "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";

    #[test]
    fn test_parse_engine_schematic() {
        let es = INPUT.parse::<EngineSchematic>().expect("Ops!");
        println!("{:?}",es);
    }
    #[test]
    fn test_engine_extract_part_numbers() {
        let es = INPUT.parse::<EngineSchematic>().expect("Ops!");

        let sum = es.part_numbers()
            .inspect(|pn| print!("F::{:?}", pn))
            .map(|pn| pn.number)
            .sum::<u32>();

        assert_eq!(sum,4361)
    }



}