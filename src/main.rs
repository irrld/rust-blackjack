use crate::game::{generate_deck, Game, GameState};
use crate::player::{Player, PlayerAction};

mod card;
mod game;
mod player;

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
            _ => PlayerAction::None,
        }
    }
}

struct AllInPlayerController;

impl PlayerController for AllInPlayerController {
    fn get_action(&self, game: &Game) -> PlayerAction {
        PlayerAction::Hit
    }
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
