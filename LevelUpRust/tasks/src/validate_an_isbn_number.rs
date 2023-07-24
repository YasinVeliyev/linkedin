use std::cmp::Ordering::*;
use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;
pub struct ISBN13 {
    pub numbers: Vec<u8>,
    pub raw: String,
}

#[derive(Debug)]
pub enum InvalidIsbn {
    TooLong,
    TooShort,
    InvalidCharacter(usize, char),
    FailedChecksum,
}

impl FromStr for ISBN13 {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = vec![];
        for (index, c) in s.replace('-', "").char_indices() {
            match c.to_digit(10) {
                Some(v) => numbers.push(v as u8),
                None => return Err(InvalidIsbn::InvalidCharacter(index, c)),
            }
        }

        match Self::check_isbn(&numbers) {
            Ok(_) => Ok(Self {
                numbers,
                raw: s.to_string(),
            }),
            Err(err) => Err(err),
        }
    }
}

impl ISBN13 {
    pub fn check_isbn(numbers: &Vec<u8>) -> Result<bool, InvalidIsbn> {
        match numbers.len().cmp(&13) {
            Less => Err(InvalidIsbn::TooShort),
            Equal => {
                let end = *numbers.last().unwrap();
                let s: u8 = numbers[0..12]
                    .iter()
                    .enumerate()
                    .map(|(index, c)| if index % 2 == 0 { *c } else { c * 3 })
                    .sum::<u8>()
                    % 10
                    + end;
                if [0, 10].contains(&s) {
                    Ok(true)
                } else {
                    Err(InvalidIsbn::FailedChecksum)
                }
            }
            Greater => Err(InvalidIsbn::TooLong),
        }
    }
}

impl Display for InvalidIsbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidIsbn::TooLong => write!(
                f,
                "Error 'InvalidIsbn::TooLong' Length of ISBN-13 have to 13"
            ),
            InvalidIsbn::TooShort => write!(
                f,
                "Error 'InvalidIsbn::TooShort' Length of ISBN-13 have to 13"
            ),
            InvalidIsbn::InvalidCharacter(c, index) => {
                write!(f, "Error InvalidIsbn::InvalidCharacter {}  at {}", c, index)
            }
            InvalidIsbn::FailedChecksum => write!(f, "Error InvalidIsbn::FailedChecksum"),
        }
    }
}

impl Error for InvalidIsbn {}
