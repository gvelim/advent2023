mod valley;
mod pattern;

use crate::pattern::Pattern;
use crate::valley::Valley;

fn main() {
    let input = std::fs::read_to_string("src/bin/day13/input.txt").expect("Ops!");
    let valley = input.parse::<Valley>().expect("Ops!");

    let t = std::time::Instant::now();
    println!("Part 1 : {:?} - {:?}", valley.summarise_notes(Pattern::find_perfect_reflection), t.elapsed());

    let t = std::time::Instant::now();
    println!("Part 2 : {:?} - {:?}", valley.summarise_notes(Pattern::find_smudged_reflection), t.elapsed());
}

