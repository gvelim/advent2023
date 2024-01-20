use crate::operation::{Instruction, FocalLength, Label};

type Len = (Label,FocalLength);

#[derive(Debug)]
pub(crate) struct ParabolicReflector {
    boxes: [Vec<Len>;256]
}

const BOXES: Vec<Len> = Vec::new();

impl Default for ParabolicReflector {
    fn default() -> Self {
        ParabolicReflector {
            boxes: [BOXES; 256]
        }
    }
}

impl ParabolicReflector {
    pub(crate) fn focusing_power(&self) -> usize {
        self.boxes_iter()
            .map(|(idx,b0x)|{
                b0x.iter()
                    .enumerate()
                    .map(|(i,len)| (idx+1) * (i+1) * len.1 )
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
    pub(crate) fn initiation(&mut self, op: &Instruction) -> bool {
        self.boxes
            .get_mut( op.hash() )
            .map(|boxes| {
                let pos = boxes.iter().position(|(label,_)| label.eq(op.label()));
                match (pos,op) {
                    (Some(i),Instruction::Remove(_)) => { boxes.remove(i); true }
                    (Some(i), Instruction::Store(_, fl)) => { boxes[i].1 = *fl; true }
                    (None, Instruction::Store(l, fl)) => { boxes.push((l.clone(),*fl)); true }
                    (None, Instruction::Remove(_)) => false
                }
            })
            .unwrap_or(false)
    }
    fn boxes_iter(&self) -> impl Iterator<Item=(usize, &Vec<Len>)> + '_ {
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
        let mut lb = ParabolicReflector::default();
        INPUT
            .split(',')
            .map(|op| op.parse::<Instruction>().expect("ops"))
            .inspect(|op| print!("{:?} -> ",op))
            .map(|op| lb.initiation(&op))
            .inspect(|op| println!("{:?}",op))
            .last();
        println!("LensLibrary: {:?}\nFocusing power: {}", lb.boxes_iter().collect::<std::rc::Rc<[_]>>(), lb.focusing_power());
        assert_eq!(lb.focusing_power(),145);
    }
}