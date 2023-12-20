use crate::direction::Direction::*;
use crate::elf::Elf;

#[derive(Debug,PartialEq)]
pub(crate) struct Field {
    pub(crate) data: Vec<char>,
    pub(crate) width: usize,
    pub(crate) start: (usize, usize)
}

impl Field {
    pub(crate) fn left_right_excluding(&self, pos:(usize,usize), pipe:char) -> (Option<&char>,Option<&char>) {
        let (lower_bound, curr, upper_bound) = (
            (pos.1)*self.width, pos.1*self.width + pos.0, (pos.1+1)*self.width - 1
        );
        (
            self.data[lower_bound..curr].iter().rev().find(|&c| pipe.ne(c)),
            self.data[curr+1..=upper_bound].iter().find(|&c| pipe.ne(c)),
        )
    }

    pub(crate) fn get_pipe(&self, pos: (usize, usize)) -> Option<char> {
        if pos.0 < self.width && pos.1 < self.data.len() / self.width {
            Some(self.data[pos.1*self.width + pos.0])
        } else {
            None
        }
    }
    pub(crate) fn get_walking_elf(&self, start: Option<(usize, usize)>) -> Elf {
        Elf {
            field: self,
            pos: start.unwrap_or(self.start),
            dir: Right,
        }
    }
    pub(crate) fn parse(s: &str, start: char) -> Field {
        let mut input = s.split('\n').peekable();
        let width = input.peek().map(|line| line.len()).expect("Can't get field width");
        let mut start_pos = 0;
        let data = input.flat_map(|line| line.chars())
            .enumerate()
            .map(|(i,c)| {
                if start.eq(&c) { start_pos = i; }
                c
            })
            .collect::<Vec<_>>();

        let start = ( start_pos % width, start_pos / width);

        Field { width, data, start }
    }
}
