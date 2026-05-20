use crate::GameState;
use visitor::prelude::*;

pub struct AsciiBoardPrinter {}

impl Default for AsciiBoardPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl AsciiBoardPrinter {
    pub fn new() -> AsciiBoardPrinter {
        AsciiBoardPrinter {}
    }
}

impl Visitor<GameState> for AsciiBoardPrinter {
    type Result = ();

    fn visit(&self, data: &GameState) -> Self::Result {
        for y in 0..9 {
            for x in 0..9 {
                print_cell(data, x, y);
            }
            print_line_break(y);
        }
    }
}

fn print_cell(data: &GameState, x: usize, y: usize) {
    let cell = data.cell(x, y);
    match cell {
        None => print!("·"),
        Some(value) => print!("{}", value),
    }
    print_spaces(x);
}

fn print_spaces(x: usize) {
    if x >= 8 {
        return;
    }
    print!(" ");
    if (x + 1).is_multiple_of(3) {
        print!("  ")
    }
}

fn print_line_break(y: usize) {
    println!();
    if (y + 1).is_multiple_of(3) && y < 8 {
        println!("                     ")
    }
}
