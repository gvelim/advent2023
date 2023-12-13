use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub(crate) struct Node {
    pub(crate) name: String,
    pub(crate) left: String,
    pub(crate) right: String
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split([' ', '=', '(', ')', ','])
            .filter(|&s| !s.is_empty());
        let node = Node {
            name: iter.next().unwrap().to_string(),
            left: iter.next().unwrap().to_string(),
            right: iter.next().unwrap().to_string(),
        };
        Ok(node)
    }
}