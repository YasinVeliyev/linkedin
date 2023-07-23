#[derive(Debug, PartialEq)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Jack,
    Queen,
    King,
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Hand { cards: vec![] }
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn value(&self) -> usize {
        let values: Vec<usize> = self
            .cards
            .iter()
            .map(|c| match c {
                Card::Ace => 0,
                Card::Two => 2,
                Card::Three => 3,
                Card::Four => 4,
                Card::Five => 5,
                Card::Six => 6,
                Card::Seven => 7,
                Card::Eight => 8,
                Card::Nine => 9,
                Card::Jack => 10,
                Card::Queen => 10,
                Card::King => 10,
            })
            .collect();

        values
            .iter()
            .map(|c| {
                if c == &0 && values.iter().sum::<usize>() <= 10 {
                    11
                } else if c != &0 {
                    *c
                } else {
                    1 as usize
                }
            })
            .sum()
    }

    pub fn is_loosing_hand(&self) -> bool {
        self.value() > 21
    }
}

