use visitor::prelude::*;
use sudoku::visualization::ascii::AsciiPrinter;
use sudoku::{GameState, Game};

fn main() {
    let game = GameState::new(Game::new_example());
    let visitor = AsciiPrinter::new();
    game.accept(&visitor);
}
