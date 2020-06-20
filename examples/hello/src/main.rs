use visitor::prelude::*;
use sudoku::visualization::ascii::AsciiPrinter;
use sudoku::{GameState, Game};
use sudoku::solver::DefaultSolver;

fn main() {
    let game = GameState::new(Game::new_example());
    let visitor = AsciiPrinter::new();

    let solution = DefaultSolver::solve(&game);

    println!("Initial state:");
    game.accept(&visitor);

    println!("\nSolution:");
    solution.accept(&visitor);
}
