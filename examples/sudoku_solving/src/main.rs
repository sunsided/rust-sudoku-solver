use visitor::prelude::*;
use sudoku::visualization::ascii::{AsciiBoardPrinter, AsciiGroupPrinter};
use sudoku::{GameState, Game};
use sudoku::solver::solve;
use std::time::Instant;

fn main() {
    // Enable logging with RUST_LOG=debug
    env_logger::init();

    let game = GameState::new(Game::new_example_nonomino());
    // let game = GameState::new(Game::new_example());
    let board_visitor = AsciiBoardPrinter::new();
    let group_visitor = AsciiGroupPrinter::new();

    println!("Groups:");
    game.accept(&group_visitor);

    println!("\nInitial state:");
    game.accept(&board_visitor);

    println!("\nSearching for solution ...");
    let now = Instant::now();

    let solution = solve(&game);

    println!("Search terminated after {} s.", now.elapsed().subsec_micros() as f64 * 1e-6);

    println!("\nSolution:");
    solution.accept(&board_visitor);

    let valid = solution.validate(false);
    println!("\nSolution valid: {}", if valid { "yes" } else { "no" });
}
