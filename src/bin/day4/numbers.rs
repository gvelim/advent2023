use std::{
    collections::HashSet, fmt::Display, str::FromStr
};

#[derive(Debug,PartialEq)]
pub enum NumbersErrors {
    InvalidDigit,
    EmptyInput
}

impl Display for NumbersErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumbersErrors::InvalidDigit => write!(f,"Invalid number found"),
            NumbersErrors::EmptyInput => write!(f,"No numbers have been provided for parsing"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Numbers(pub(crate) HashSet<u32>);
impl FromStr for Numbers {
    type Err = NumbersErrors;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input
            .split_ascii_whitespace()
            .map(u32::from_str)
            .collect::<Result<HashSet<u32>,_>>()
        {
            Ok(set) if !set.is_empty() => Ok(Numbers(set)),
            Err(_) => Err(NumbersErrors::InvalidDigit),
            _ => Err(NumbersErrors::EmptyInput)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_numbers_errors() {
        let input = " ";
        match input.parse::<Numbers>() {
            Ok(r) => panic!("this should not succeed with result {:?}",r),
            Err(e) => assert_eq!(e, NumbersErrors::EmptyInput),
        }
    }
}
