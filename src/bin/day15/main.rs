mod hash;
use crate::hash::HashLen;

fn main() {
    let mut input = std::fs::read_to_string("./src/bin/day15/input.txt").expect("Ops");
    input.pop();
    let split = input.split([',','\n']);

    let t = std::time::Instant::now();
    let sum = split
        .into_iter()
        .map(|label| label.hash_algo() )
        .sum::<usize>();

    println!("Part 1 : Sum of Hashes = {sum} - {:?}", t.elapsed());
    assert_eq!(sum,506869);
}

#[cfg(test)]
mod test {

}