mod card;
mod dealer;
mod engine;
mod game;
mod player;

use crate::game::Game;

fn main() {
    let mut game: Game = Game::new();
    game.play_five_draw();
}
