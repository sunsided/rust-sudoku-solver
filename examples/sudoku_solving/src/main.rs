use visitor::prelude::*;
use sudoku::visualization::ascii::{AsciiBoardPrinter, AsciiGroupPrinter};
use sudoku::{GameState, Game};
use sudoku::solver::solve;

fn main() {
    let game = GameState::new(Game::new_example_nonomino());
    let game = GameState::new(Game::new_example());
    let board_visitor = AsciiBoardPrinter::new();
    let group_visitor = AsciiGroupPrinter::new();

    let solution = solve(&game);

    println!("Groups:");
    game.accept(&group_visitor);

    println!("\nInitial state:");
    game.accept(&board_visitor);

    println!("\nSolution:");
    solution.accept(&board_visitor);

    let valid = solution.validate();
    println!("\nSolution valid: {}", if valid { "yes" } else { "no" });
}
