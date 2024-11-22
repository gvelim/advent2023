use std::{
    collections::HashSet, fmt::Display, str::FromStr
};
use std::num::{IntErrorKind, ParseIntError};

#[derive(Debug,PartialEq)]
pub enum NumbersErrors {
    InvalidDigit,
    EmptyInput,
    Unknown
}

impl Display for NumbersErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumbersErrors::InvalidDigit => write!(f,"Invalid number found"),
            NumbersErrors::EmptyInput => write!(f,"No numbers have been provided for parsing"),
            _ => write!(f,"Unknown error incurred")
        }
    }
}

impl From<ParseIntError> for NumbersErrors {
    fn from(err: ParseIntError) -> NumbersErrors {
        match err.kind() {
            IntErrorKind::Empty => NumbersErrors::EmptyInput,
            IntErrorKind::InvalidDigit => NumbersErrors::InvalidDigit,
            IntErrorKind::PosOverflow => NumbersErrors::InvalidDigit,
            IntErrorKind::NegOverflow => NumbersErrors::InvalidDigit,
            IntErrorKind::Zero => NumbersErrors::InvalidDigit,
            _ => NumbersErrors::Unknown
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
            Err(e) => Err(e.into()),
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
