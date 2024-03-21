mod card;
mod player;
mod game;

use std::cmp::PartialEq;
use strum::IntoEnumIterator;
use crate::card::{Card, CardType};
use crate::card::CardValue;
use crate::game::{Game, GameState};
use crate::player::{Handed, Player, PlayerAction};

fn generate_deck() -> Vec<Card> {
    let mut deck = Vec::new();
    for card_type in CardType::iter() {
        for card_value in CardValue::iter() {
            deck.push(Card::new(card_type.clone(), card_value.clone()));
        }
    }
    // shuffle
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    deck
}

trait PlayerController {
    fn get_action(&self, game: &Game) -> PlayerAction;
}

struct ConsolePlayerController;

impl PlayerController for ConsolePlayerController {
    fn get_action(&self, game: &Game) -> PlayerAction {
        println!("Enter your action: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        match input.trim() {
            "hit" => PlayerAction::Hit,
            "stand" => PlayerAction::Stand,
            _ => PlayerAction::None
        }
    }
}

struct AllInPlayerController;

impl PlayerController for AllInPlayerController {
    fn get_action(&self, game: &Game) -> PlayerAction {
        PlayerAction::Hit
    }
}

fn print_hand(game: &Game, handed: &dyn Handed) {
    for card in handed.get_hand() {
        println!("{}", card);
    }
    println!("Total Value: {}", handed.count(game));
}

fn main() {
    let mut game = Game::new(generate_deck());
    game.add_player(Player::new("Player 1", Box::new(ConsolePlayerController)));
    game.add_player(Player::new("Player 2", Box::new(AllInPlayerController)));
    game.init();
    while game.get_game_state() == GameState::Playing {
        game.play();
    }
    game.print_result();
}
