use crate::visitor::BoardVisitor;
use crate::Board;

pub struct AsciiPrinter {}

impl AsciiPrinter {
    pub fn new() -> AsciiPrinter {
        AsciiPrinter{}
    }
}

impl BoardVisitor for AsciiPrinter {
    type Result = ();

    fn visit(&self, data: &Board) -> Self::Result {
        for y in 0..9 {
            for x in 0..9 {
                print_cell(data, x, y);
            }
            print_line_break(y);
        }
    }
}

fn print_cell(data: &Board, x: usize, y: usize) {
    let cell = data.cell(x, y);
    match cell {
        None => print!("·"),
        Some(value) => print!("{}", value)
    }
    print_spaces(x);
}

fn print_spaces(x: usize) {
    if x >= 8 {
        return
    }
    print!(" ");
    if (x + 1) % 3 == 0 {
        print!("  ")
    }
}

fn print_line_break(y: usize) {
    println!();
    if (y + 1) % 3 == 0 && y < 8 {
        println!("                     ")
    }
}
