mod field;
mod direction;
mod elf;

use crate::field::Field;

fn main() {
    let input = std::fs::read_to_string("src/bin/day10/input.txt").expect("Can't read input");
    let f = Field::parse(input.as_str(),'S');

    let mut elf = f.get_walking_elf(None);

    let dirs = elf.possible_directions();
    println!("Available directions {:?}",dirs);
    elf.dir = *dirs.iter().next().expect("Ops! cannot find valid direction to go!");

    let count = elf
        .take_while(|p| 'S'.ne(p))
        // .inspect(|p| print!("{p},"))
        .count() + 1;

    println!("Part 1 : Total steps: {}, furthest away: {}", count, count/2)

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::direction::Direction::{Down, Left, Right, Up};
    static INPUT: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";

    #[test]
    fn test_pipe_waking() {
        let f = Field::parse(INPUT,'S');
        let elf = f.get_walking_elf(None);

        assert_eq!(
            elf.take(16).collect::<Vec<_>>(),
            ['J', 'F', 'J', 'F', '7', '|', 'L', '7', 'J', '-', '-', 'F', 'J', 'L', '|']
        );
    }
    #[test]
    fn test_direction() -> Result<(),()> {
        let f = Field::parse(INPUT,'S');

        let mut dir = Up.pipe_exit( f.get_pipe((2, 0)).unwrap()  ).unwrap();
        assert_eq!(dir, Right);
        dir = dir.pipe_exit(f.get_pipe((3, 0)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit(f.get_pipe((3, 1)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit(f.get_pipe((3, 2)).unwrap()).unwrap();
        assert_eq!(dir, Right);
        dir = dir.pipe_exit( f.get_pipe((4, 2)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        dir = dir.pipe_exit( f.get_pipe((4, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get_pipe((3, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get_pipe((2, 3)).unwrap()).unwrap();
        assert_eq!(dir, Left);
        dir = dir.pipe_exit( f.get_pipe((1, 3)).unwrap()).unwrap();
        assert_eq!(dir, Down);
        Ok(())
    }
    #[test]
    fn test_boundaries() {
        let f = Field::parse(INPUT,'S');
        
        assert_eq!(Some('S'), f.get_pipe(f.start));
        assert_eq!(Some('7'), f.get_pipe((3, 0)));
        assert_eq!(Some('F'), f.get_pipe((2, 0)));
        assert_eq!(Some('J'), f.get_pipe((2, 1)));
        assert_eq!(Some('.'), f.get_pipe((2, 2)));
        assert_eq!(Some('J'), f.get_pipe((4, 3)));
        assert_eq!(Some('.'), f.get_pipe((4, 4)));
        assert_eq!(None, f.get_pipe((8, 3)));
        assert_eq!(None, f.get_pipe((3, 6)));
        assert_eq!(None, f.get_pipe((8, 6)));
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
