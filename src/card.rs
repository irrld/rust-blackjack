use std::fmt;

use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum CardType {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[repr(u8)]
#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub card_type: CardType,
    pub card_value: CardValue,
}

impl Card {
    pub fn new(card_type: CardType, card_value: CardValue) -> Self {
        Self {
            card_type,
            card_value,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", &self.card_type, &self.card_value)
    }
}
