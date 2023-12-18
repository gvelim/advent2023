use std::str::FromStr;

fn main() {

}

#[derive(Debug,PartialEq)]
struct Field {
    data: Vec<char>,
    width: usize
}

impl Field {
    fn get(&self, x:usize, y:usize) -> Option<char> {
        if x < self.width && y < self.data.len() / self.width {
            Some(self.data[y*self.width + x])
        } else {
            None
        }
    }
}

impl FromStr for Field {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.split('\n').peekable();

        let width = input.peek().map(|line| line.len()).expect("Can't get field width");
        let mut data = input.flat_map(|line| line.chars()).collect::<Vec<_>>();
        
        Ok( Field { width, data } )
    }
}

#[cfg(test)]
mod test {
    use crate::Field;

    static INPUT: &str = "..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ...";

    #[test]
    fn test_boundaries() {
        let f = INPUT.parse::<Field>().expect("Can't parse field");
        
        assert_eq!(Some('7'), f.get(3, 0));
        assert_eq!(Some('F'), f.get(2, 0));
        assert_eq!(Some('J'), f.get(2, 1));
        assert_eq!(Some('.'), f.get(2, 2));
        assert_eq!(Some('J'), f.get(4, 3));
    }

    #[test]
    fn test_parse_map() {
        assert_eq!(
            INPUT.parse::<Field>().expect("Can't parse field"),
            Field { data: vec!['.', '.', 'F', '7', '.', '.', 'F', 'J', '|', '.', 'S', 'J', '.', 'L', '7', '|', 'F', '-', '-', 'J', 'L', 'J', '.', '.', '.'], width: 5 }
        );
    }

}
