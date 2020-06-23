use visitor::prelude::*;
use sudoku::visualization::ascii::AsciiPrinter;
use sudoku::{GameState, Game};
use sudoku::solver::solve;

fn main() {
    let game = GameState::new(Game::new_example_nonomino());
    let visitor = AsciiPrinter::new();

    let solution = solve(&game);

    println!("Initial state:");
    game.accept(&visitor);

    println!("\nSolution:");
    solution.accept(&visitor);
}
