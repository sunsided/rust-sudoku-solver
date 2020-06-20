use visitor::prelude::*;
use sudoku::prelude::*;
use sudoku::visualization::ascii::AsciiPrinter;

fn main() {
    let game = GameState::new(Game::new_example());
    let visitor = AsciiPrinter::new();
    game.accept(&visitor);
}
