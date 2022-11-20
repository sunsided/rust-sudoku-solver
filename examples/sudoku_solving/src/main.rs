mod command;

use crate::command::build_command;
use std::time::Instant;
use sudoku::solver::solve;
use sudoku::visualization::ascii::{AsciiBoardPrinter, AsciiGroupPrinter};
use sudoku::{Game, GameState};
use visitor::prelude::*;

fn main() {
    // Enable logging with RUST_LOG=debug
    env_logger::init();

    let matches = build_command().get_matches();
    let game = if matches.get_flag("normal") {
        GameState::new(Game::new_example())
    } else if matches.get_flag("nonomino") {
        GameState::new(Game::new_example_nonomino())
    } else if matches.get_flag("hypersudoku") {
        GameState::new(Game::new_example_hypersudoku())
    } else {
        unimplemented!()
    };

    let board_visitor = AsciiBoardPrinter::new();
    let group_visitor = AsciiGroupPrinter::new();

    println!("Groups:");
    game.accept(&group_visitor);

    println!("\nInitial state:");
    game.accept(&board_visitor);

    println!("\nSearching for solution ...");
    let now = Instant::now();

    let solution = solve(&game);

    println!(
        "Search terminated after {} s.",
        now.elapsed().subsec_micros() as f64 * 1e-6
    );

    println!("\nSolution:");
    solution.accept(&board_visitor);

    let valid = solution.validate(false);
    println!("\nSolution valid: {}", if valid { "yes" } else { "no" });
}
