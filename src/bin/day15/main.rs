mod hash;
mod operation;
mod lenslib;

use hash::HashLen;
use operation::Instruction;
use lenslib::ParabolicReflector;

fn main() {
    let mut input = std::fs::read_to_string("./src/bin/day15/input.txt").expect("Ops");
    input.pop();

    let t = std::time::Instant::now();
    let sum = input
        .split([',','\n'])
        .map(|label| label.hash_algo() )
        .sum::<usize>();

    println!("Part 1 : Sum of Hashes = {sum} - {:?}", t.elapsed());
    assert_eq!(sum,506869);

    let t = std::time::Instant::now();
    let mut lb = ParabolicReflector::default();
    input
        .split([',','\n'])
        .map(|op| op.parse::<Instruction>().expect("Failed to parse operations"))
        .map(|op| lb.initiation(&op))
        .last();

    println!("Part 2 Focusing power: {} - {:?}",lb.focusing_power(), t.elapsed());
    assert_eq!(lb.focusing_power(),271384);
}
