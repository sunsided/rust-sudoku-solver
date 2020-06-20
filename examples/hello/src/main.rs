use sudoku::visualization::ascii::AsciiPrinter;
use sudoku::{Game, GameState};
use visitor::prelude::*;

fn main() {
    let game = GameState::new(Game::new_example());
    let visitor = AsciiPrinter::new();
    game.accept(&visitor);
}
