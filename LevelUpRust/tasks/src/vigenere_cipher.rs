use std::collections::HashMap;

pub struct Vigenara {
    letter_to_index: HashMap<char, usize>,
    index_to_letter: HashMap<usize, char>,
    key: String,
}

impl Vigenara {
    pub fn new(key: &str) -> Self {
        let letter_to_index = HashMap::from([
            ('A', 0),
            ('B', 1),
            ('C', 2),
            ('D', 3),
            ('E', 4),
            ('F', 5),
            ('G', 6),
            ('H', 7),
            ('I', 8),
            ('J', 9),
            ('K', 10),
            ('L', 11),
            ('M', 12),
            ('N', 13),
            ('O', 14),
            ('P', 15),
            ('Q', 16),
            ('R', 17),
            ('S', 18),
            ('T', 19),
            ('U', 20),
            ('V', 21),
            ('W', 22),
            ('X', 23),
            ('Y', 24),
            ('Z', 25),
        ]);

        let index_to_letter = HashMap::from([
            (0, 'A'),
            (1, 'B'),
            (2, 'C'),
            (3, 'D'),
            (4, 'E'),
            (5, 'F'),
            (6, 'G'),
            (7, 'H'),
            (8, 'I'),
            (9, 'J'),
            (10, 'K'),
            (11, 'L'),
            (12, 'M'),
            (13, 'N'),
            (14, 'O'),
            (15, 'P'),
            (16, 'Q'),
            (17, 'R'),
            (18, 'S'),
            (19, 'T'),
            (20, 'U'),
            (21, 'V'),
            (22, 'W'),
            (23, 'X'),
            (24, 'Y'),
            (25, 'Z'),
        ]);
        Self {
            letter_to_index,
            index_to_letter,
            key: key.to_owned(),
        }
    }

    pub fn encrypt(&self, s: &str) -> String {
        let key_iterator = self
            .key
            .chars()
            .map(|c| self.letter_to_index.get(&c).unwrap());

        s.chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| self.letter_to_index.get(&c.to_ascii_uppercase()).unwrap())
            .zip(key_iterator.cycle())
            .map(|(a, b)| (a + b) % 26)
            .map(|ref c| self.index_to_letter.get(c).unwrap())
            .collect::<String>()
    }

    pub fn decrypt(&self, encrypted_string: &str) -> String {
        let key_iterator = self
            .key
            .chars()
            .map(|c| self.letter_to_index.get(&c).unwrap());

        encrypted_string
            .chars()
            .filter(|c| c.is_alphabetic())
            .map(|c| self.letter_to_index.get(&c.to_ascii_uppercase()).unwrap())
            .zip(key_iterator.cycle())
            .map(|(a, b)| (26 + a - b) % 26)
            .map(|ref c| self.index_to_letter.get(c).unwrap())
            .collect::<String>()
    }
}
