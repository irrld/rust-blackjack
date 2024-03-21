use strum_macros::EnumIter;
use crate::card::{Card, CardType, CardValue};
use crate::player::{Dealer, Handed, Player, PlayerAction, PlayerState};
use crate::{print_hand};


#[derive(EnumIter, Debug, PartialEq, Clone, Copy)]
pub enum GameState {
    Playing,
    End
}

pub struct Game {
    players: Vec<Player>,
    dealer: Dealer,
    deck: Vec<Card>,
    game_state: GameState,
    rounds_played: u8
}

impl Game {

    pub fn new(deck: Vec<Card>) -> Self {
        Self {
            players: Vec::new(),
            dealer: Dealer {
                hand: Vec::new()
            },
            deck,
            game_state: GameState::Playing,
            rounds_played: 0
        }
    }

    pub fn value_of_card(&self, card_value: CardValue, card_type: CardType) -> u8 {
        match card_value {
            CardValue::Ace => 11,
            CardValue::Two => 2,
            CardValue::Three => 3,
            CardValue::Four => 4,
            CardValue::Five => 5,
            CardValue::Six => 6,
            CardValue::Seven => 7,
            CardValue::Eight => 8,
            CardValue::Nine => 9,
            CardValue::Ten => 10,
            CardValue::Jack => 10,
            CardValue::Queen => 10,
            CardValue::King => 10
        }
    }

    pub fn pick_card(&mut self) -> Card {
        self.deck.pop().unwrap()
    }

    pub fn is_winner(&self, player: &Player) -> bool {
        if player.count(&self) == 21 {
            return true
        }
        return false
    }

    pub fn is_bust(&self, player: &Player) -> bool {
        if player.count(&self) > 21 {
            return true
        }
        return false
    }

    pub fn deal_cards(&mut self) {
        let mut cards = Vec::new();
        for _ in 0..self.players.len() + 1 {
            cards.push(self.pick_card());
        }
        for player in self.players.iter_mut() {
            player.add_card(cards.pop().unwrap());
        }
        self.dealer.add_card(cards.pop().unwrap());
    }

    pub fn init(&mut self) {
        self.deal_cards();
    }

    pub fn play(&mut self) {
        println!("----------------");
        if self.rounds_played > 0 {
            println!("Dealer's hand:");
            print_hand(&self, &self.dealer);
            println!("----------------");
        }
        for i in 0..self.players.len() {
            println!("{}'s hand:", self.players[i].name);
            print_hand(self, &self.players[i]);
            println!("----------------");
        }

        for i in 0..self.players.len() {
            let player = self.players.get(i).unwrap();
            let mut new_state = player.state;
            if self.is_winner(player) {
                new_state = PlayerState::Win;
            } else if self.is_bust(player) {
                new_state = PlayerState::Bust;
            } else if player.state == PlayerState::Playing {
                let action = player.controller.get_action(&self);
                if action == PlayerAction::Hit {
                    let card = self.pick_card();
                    self.players[i].add_card(card);
                    println!("{} hits and gets {}!", self.players[i].name, card);
                    new_state = PlayerState::Playing;
                } else if action == PlayerAction::Stand {
                    new_state = PlayerState::Stand;
                }
            }
            self.players[i].state = new_state;
        }
        let dealer_count = self.dealer.count(&self);
        if dealer_count < 17 {
            let card = self.pick_card();
            println!("Dealer hits and gets {}!", card);
            self.dealer.add_card(card);
        }

        let dealer_count = self.dealer.count(&self);
        if dealer_count > 21 {
            println!("Dealer busts!");
            for i in 0..self.players.len() {
                if self.players[i].state != PlayerState::Bust {
                    self.players[i].state = PlayerState::Win;
                }
            }
            self.game_state = GameState::End;
            return;
        }

        let mut done = true;
        for i in 0..self.players.len() {
            if self.players[i].state == PlayerState::Playing {
                done = false;
                break;
            }
        }
        if done {
            for i in 0..self.players.len() {
                if self.players[i].state == PlayerState::Bust || self.players[i].state == PlayerState::Win {
                    continue
                }
                let player_count = self.players[i].count(&self);
                if player_count > dealer_count {
                    self.players[i].state = PlayerState::Win;
                } else if player_count < dealer_count {
                    self.players[i].state = PlayerState::Bust;
                } else {
                    self.players[i].state = PlayerState::Stand;
                }
            }
            self.game_state = GameState::End;
        }
        self.rounds_played += 1;
    }

    pub fn print_result(&self) {
        println!("----------------");
        println!("Dealer's hand:");
        print_hand(&self, &self.dealer);
        println!("----------------");
        for i in 0..self.players.len() {
            println!("{}'s hand:", self.players[i].name);
            print_hand(self, &self.players[i]);
            println!("----------------");
        }

        let mut all_bust = true;
        for i in 0..self.players.len() {
            if self.players[i].state == PlayerState::Win {
                println!("{} wins!", self.players[i].name);
                all_bust = false;
            } else if self.players[i].state == PlayerState::Bust {
                println!("{} busts!", self.players[i].name);
            } else {
                println!("{} stands!", self.players[i].name);
                all_bust = false;
            }
        }
        if all_bust {
            println!("Dealer wins!");
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn get_game_state(&self) -> GameState {
        self.game_state
    }
}