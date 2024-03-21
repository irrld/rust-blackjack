use strum_macros::EnumIter;

use crate::card::Card;
use crate::game::Game;
use crate::PlayerController;

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum PlayerState {
    Playing,
    Stand,
    Bust,
    Win,
}

#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum PlayerAction {
    Hit,
    Stand,
    None,
}

pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub state: PlayerState,
    pub controller: Box<dyn PlayerController>,
}

impl Player {
    pub fn new(name: &str, controller: Box<dyn PlayerController>) -> Self {
        Self {
            name: name.to_string(),
            hand: Vec::new(),
            state: PlayerState::Playing,
            controller,
        }
    }
}

pub struct Dealer {
    pub hand: Vec<Card>,
}

pub trait Handed {
    fn count(&self, game: &Game) -> u8;

    fn add_card(&mut self, card: Card);

    fn get_hand(&self) -> &Vec<Card>;
}

impl Handed for Player {
    fn count(&self, game: &Game) -> u8 {
        let mut value = 0;
        for x in &self.hand {
            value += game.value_of_card(x.card_value, x.card_type);
        }
        return value;
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
}

impl Handed for Dealer {
    fn count(&self, game: &Game) -> u8 {
        let mut value = 0;
        for x in &self.hand {
            value += game.value_of_card(x.card_value, x.card_type);
        }
        return value;
    }

    fn add_card(&mut self, card: Card) {
        self.hand.push(card);
    }

    fn get_hand(&self) -> &Vec<Card> {
        &self.hand
    }
}
