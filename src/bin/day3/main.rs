
mod engine;
mod parts;

use crate::engine::*;

fn main() {
    let input = std::fs::read_to_string("src/bin/day3/input.txt").expect("Ops!");
    let es = input.parse::<EngineSchematic>().expect("Ops!");

    let sum = es
        .part_numbers()
        .map(|pn| pn.number)
        .sum::<u32>();

    println!("Par 1 - Sum: {sum}");

    let sum = es
        .get_gears_part_numbers('*')
        .map(|d| d.iter().map(|d| d.number).product::<u32>())
        .sum::<u32>();

    println!("Par 1 - Sum: {sum}");
}

#[cfg(test)]
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
        println!("{:?}\n{:?}",es.partnums,es.symbols);
        let sum = es.part_numbers()
            .inspect(|pn| print!("F::{:?}", pn))
            .map(|pn| pn.number)
            .sum::<u32>();

        assert_eq!(sum,4361)
    }

    #[test]
    fn test_engine_extract_with_symbol() {
        let es = INPUT.parse::<EngineSchematic>().expect("Ops!");

        let sum = es.get_gears_part_numbers('*')
            .inspect(|d| println!("{:?},",d))
            .map(|d| d.iter().map(|d| d.number).product::<u32>())
            .sum::<u32>();

        println!("{:?}",sum);
        assert_eq!(467835,sum)

    }
}