use crate::Direction::{Down, Left, Right, Up};

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").expect("Can't read input");
    let f = Field::parse(input.as_str(),'S');

    let mut elf = f.walk_pipe();
    elf.dir = Direction::Down;
    let count = elf
        .take_while(|p| 'S'.ne(p))
        .inspect(|p| print!("{p},"))
        .count() + 1;

    println!("Part 1 : Steps {}, furthest {}", count, count/2)

}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction { Up, Right, Down, Left }

impl Direction {
    fn pipe_exit(&self, c: char) -> Option<Self> {
        use Direction::*;
        match (self, c) {
            (Left|Right , '-') => Some(*self),
            (Up|Down , '|') => Some(*self),
            (Down, 'L') => Some(Right),
            (Left, 'L') => Some(Up),
            (Down, 'J') => Some(Left),
            (Right, 'J') => Some(Up),
            (Up, '7') => Some(Left),
            (Right, '7') => Some(Down),
            (Up, 'F') => Some(Right),
            (Left, 'F') => Some(Down),
            _ => None
        }
    }
}

#[derive(Debug,PartialEq)]
struct Elf<'a> {
    field: &'a Field,
    pos: (usize,usize),
    dir: Direction
}

impl Elf<'_> {
    fn step_one(&mut self) -> Option<char> {
        let pos = match self.dir {
            Up => (self.pos.0, self.pos.1-1),
            Right => (self.pos.0+1, self.pos.1),
            Down => (self.pos.0, self.pos.1+1),
            Left => (self.pos.0-1, self.pos.1),
        };
        // is this a valid position ?
        self.field.get(pos)
            .and_then(|p|
                // Can we enter the new pipe from current direction ?
                self.dir.pipe_exit(p)
                    // new pipe is connected to current hence move one step
                    .and_then(|dir| {
                        self.pos = pos;
                        self.dir = dir;
                        Some(p)
                    })
            )
    }
}

impl Iterator for Elf<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.step_one()
    }
}

#[derive(Debug,PartialEq)]
struct Field {
    data: Vec<char>,
    width: usize,
    start: (usize,usize)
}

impl Field {
    fn get(&self, pos: (usize,usize)) -> Option<char> {
        if pos.0 < self.width && pos.1 < self.data.len() / self.width {
            Some(self.data[pos.1*self.width + pos.0])
        } else {
            None
        }
    }
    fn parse(s: &str, start: char) -> Field {
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
    fn walk_pipe(&self) -> Elf {
        Elf {
            field: self,
            pos: self.start,
            dir: Right,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static INPUT: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";

    #[test]
    fn test_pipe_waking() {
        let f = Field::parse(INPUT,'S');
        let elf = f.walk_pipe();

        assert_eq!(
            elf.take(16).collect::<Vec<_>>(),
            ['J', 'F', 'J', 'F', '7', '|', 'L', '7', 'J', '-', '-', 'F', 'J', 'L', '|']
        );
    }
    #[test]
    fn test_direction() -> Result<(),()> {
        let f = Field::parse(INPUT,'S');

        let mut dir = Up.pipe_exit( f.get((2, 0)).unwrap()  ).unwrap();
        assert_eq!(dir, Right);
        dir = dir.pipe_exit(f.get((3, 0)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit(f.get((3, 1)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit(f.get((3, 2)).unwrap()).unwrap();
        assert_eq!(dir, Right);
        dir = dir.pipe_exit( f.get((4, 2)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit( f.get((4, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get((3, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get((2, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get((1, 3)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        Ok(())
    }
    #[test]
    fn test_boundaries() {
        let f = Field::parse(INPUT,'S');
        
        assert_eq!(Some('S'), f.get(f.start));
        assert_eq!(Some('7'), f.get((3, 0)));
        assert_eq!(Some('F'), f.get((2, 0)));
        assert_eq!(Some('J'), f.get((2, 1)));
        assert_eq!(Some('.'), f.get((2, 2)));
        assert_eq!(Some('J'), f.get((4, 3)));
        assert_eq!(Some('.'), f.get((4, 4)));
        assert_eq!(None, f.get((8, 3)));
        assert_eq!(None, f.get((3, 6)));
        assert_eq!(None, f.get((8, 6)));
    }
    #[test]
    fn test_parse_map() {
        let f = Field::parse(INPUT,'S');
        assert_eq!(
            f,
            Field {
                data: vec!['.', '.', 'F', '7', '.', '.', 'F', 'J', '|', '.', 'S', 'J', '.', 'L', '7', '|', 'F', '-', '-', 'J', 'L', 'J', '.', '.', '.'],
                width: 5,
                start: (0,2)
            }
        );
    }

}
