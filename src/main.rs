// this is a texas holdem poker game and ai

use architecture::GameState;

pub mod game_functionality;
pub mod architecture;
fn main() {
    let mut game_state = GameState::initialise();
    game_state
}