#![feature(iter_collect_into)]
mod valley;
mod pattern;

use crate::valley::Valley;

fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Ops!");
    let mut valley = input.parse::<Valley>().expect("Ops!");

    let t = std::time::Instant::now();
    println!("Part 1 : {:?} - {:?}", valley.summarise_notes(), t.elapsed());

    valley.fix_smudged_mirrors();

    let t = std::time::Instant::now();
    println!("Part 2 : {:?} - {:?}", valley.summarise_notes(), t.elapsed());
}

#[cfg(test)]
mod test {
    use crate::pattern::Pattern;
    use super::*;

    #[test]
    fn test_fix_smudge() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let mut valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter_mut()
            .inspect(|p| println!("Before: {:?}",p))
            .map(|p| p.fix_smudge())
            .inspect(|p| println!("After: {:?}",p))
            .all(|_| true);
    }
    #[test]
    fn test_calculate_smudged_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        let _out = valley.patterns.iter()
            .map(|pat| {
                let h = Pattern::find_smudge(&pat.p).max();
                println!("===========");
                let v = Pattern::find_smudge(&pat.t).max();
                (h, v)
            })
            .inspect(|p| println!("Hor: {:?}\nVer: {:?}",p.0,p.1))
            .all(|_| true);
    }

    #[test]
    fn test_calculate_sample_input() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let mut valley = input.parse::<Valley>().expect("Ops!");

        assert_eq!(valley.summarise_notes(),405);

        valley.fix_smudged_mirrors();
        println!("{:?}",valley);

        assert_eq!(valley.summarise_notes(),400);
    }
    #[test]
    fn test_find_vertical_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample_p2.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_vertical_mirror_max())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }
    #[test]
    fn test_find_horizontal_mirror() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.iter()
            .inspect(|p| print!("{:?} -> ",&p))
            .map(|p| p.find_horizontal_mirror_max())
            .inspect(|p| println!("{:?} -> ",&p))
            .all(|_| true);
    }

    #[test]
    fn test_parse() {
        let input = std::fs::read_to_string("src/bin/day13/sample.txt").expect("Ops!");
        let valley = input.parse::<Valley>().expect("Ops!");

        valley.patterns.into_iter().for_each(|pat| println!("{:?}\n",pat))
    }
}