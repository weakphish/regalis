use model::Game;

mod model;

fn main() {
    let game = Game::new();
    game.print_board();
}
