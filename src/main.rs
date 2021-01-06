use model::Game;

mod model;

fn main() {
    let mut game = Game::new();
    game.run_game();
}
