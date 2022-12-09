use crate::GameState;
use visitor::prelude::*;

pub struct AsciiGroupPrinter {}

impl AsciiGroupPrinter {
    pub fn new() -> AsciiGroupPrinter {
        AsciiGroupPrinter {}
    }
}

impl Visitor<GameState> for AsciiGroupPrinter {
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
    let id = data.game.group_id(x, y);

    let tokens = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
        'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        '@', '#', '*', '%',
    ];

    print!("{}", tokens[(id as usize) % tokens.len()]);
    print_spaces(x);
}

fn print_spaces(x: usize) {
    if x >= 8 {
        return;
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
