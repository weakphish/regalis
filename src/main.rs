use model::Game;

mod model;

fn main() {
    let game = Game::new();
    game.run_game();
}
