use crate::operation::{Operation, FocalLength, Label};
use crate::hash::HashLen;

type Len = (Label,FocalLength);

#[derive(Debug)]
pub(crate) struct LensLibrary {
    boxes: [Vec<Len>;256]
}

const BOXES: Vec<Len> = Vec::new();

impl Default for LensLibrary {
    fn default() -> Self {
        LensLibrary {
            boxes: [BOXES; 256]
        }
    }
}

impl LensLibrary {
    pub(crate) fn focusing_power(&self) -> usize {
        self.boxes()
            .map(|(idx,b0x)|{
                b0x.iter()
                    .enumerate()
                    .map(|(i,len)| (idx+1) * (i+1) * len.1 )
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
    pub(crate) fn initiation(&mut self, op: &Operation) -> bool {
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
                if let Some(index) = boxes.iter().position(|(label,_)| label.eq(&l)) {
                    boxes.remove(index);
                    true
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
                    boxes.push((l.clone(),*fl));
                }
                true
            })
            .unwrap_or(false)
    }
    fn boxes(&self) -> impl Iterator<Item=(usize,&Vec<Len>)> + '_ {
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
        INPUT
            .split(',')
            .map(|op| op.parse::<Operation>().expect("ops"))
            .inspect(|op| print!("{:?} -> ",op))
            .map(|op| lb.initiation(&op))
            .inspect(|op| println!("{:?}",op))
            .last();
        println!("LensLibrary: {:?}\nFocusing power: {}",lb.boxes().collect::<std::rc::Rc<[_]>>(),lb.focusing_power());
        assert_eq!(lb.focusing_power(),145);
    }
}