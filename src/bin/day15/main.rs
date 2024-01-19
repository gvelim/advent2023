mod hash;
mod operation;

use hash::{HashLen};
use operation::Operation;
use std::collections::VecDeque;

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

use std::rc::Rc;

#[derive(Debug)]
struct LensLibrary {
    boxes: [VecDeque<(Rc<str>,usize)>;256]
}

const ARRAY_REPEAT_VALUE: VecDeque<(std::rc::Rc<str>, usize)> = VecDeque::new();
impl Default for LensLibrary {
    fn default() -> Self {
        LensLibrary {
            boxes: [ARRAY_REPEAT_VALUE; 256]
        }
    }
}
impl LensLibrary {
    fn process(&mut self, op: &Operation) -> bool {
        match op {
            Operation::Remove(_) => self.remove_focal_length(&op),
            Operation::Store(_,_) => self.store_focal_length(&op),
        }
    }
    fn remove_focal_length(&mut self, op: &Operation) -> bool {
        let Operation::Remove(l) = op else { return false };
        self.boxes
            .get_mut( l.hash_algo() )
            .map(|boxes|{
                let pos = boxes.iter().position(|(label,_)| label.eq(&l));
                if let Some(index) = pos {
                    boxes.remove(index).is_some()
                } else {
                    false
                }
            })
            .unwrap_or(false)
    }
    fn store_focal_length(&mut self, op: &Operation) -> bool {
        let Operation::Store(l,fl) = op else { return false };

        self.boxes
            .get_mut( l.hash_algo() )
            .map(|boxes| {
                if !boxes
                    .iter_mut()
                    .filter(|(label,_)| label.eq(&l))
                    .any(|(_,focal_length)| {
                        *focal_length = *fl;
                        true
                    })
                {
                    boxes.push_back((l.clone(),*fl));
                }
                true
            })
            .unwrap_or(false)
    }
    fn boxes(&self) -> impl Iterator<Item=(usize,&VecDeque<(std::rc::Rc<str>, usize)>)> + '_ {
        self.boxes
            .iter()
            .enumerate()
            .filter(|(_,b0x)| !b0x.is_empty())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_initialization_sequence() {
        let mut lb = LensLibrary::default();
        let ops = INPUT
            .split(',')
            .map(|op| op.parse::<Operation>().expect("ops"))
            .inspect(|op| print!("{:?} -> ",op))
            .map(|op| lb.process(&op))
            .inspect(|op| println!("{:?}",op))
            .all(|_| true);
        println!("{:?}",lb.boxes().collect::<Rc<[_]>>());
    }
    #[test]
    fn test_process_box() {
        let mut lb = LensLibrary::default();
        lb.process( & "rn=1".parse::<Operation>().expect("Ops"));
        lb.process( & "cm-".parse::<Operation>().expect("Ops"));
        lb.process( & "qp=3".parse::<Operation>().expect("Ops"));
        lb.process( & "cm=2".parse::<Operation>().expect("Ops"));
        lb.process( & "qp-".parse::<Operation>().expect("Ops"));
        lb.process( & "pc=4".parse::<Operation>().expect("Ops"));
        lb.process( & "ot=9".parse::<Operation>().expect("Ops"));
        lb.process( & "ab=5".parse::<Operation>().expect("Ops"));
        lb.process( & "pc-".parse::<Operation>().expect("Ops"));
        lb.process( & "pc=6".parse::<Operation>().expect("Ops"));
        lb.process( & "ot=7".parse::<Operation>().expect("Ops"));
        println!("{:?}",lb.boxes().collect::<Rc<[_]>>());
    }

}