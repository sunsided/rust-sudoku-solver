pub use sudoku::visitor::AcceptVisitor;
use sudoku::visualization::ascii::AsciiPrinter;
use sudoku::Board;

fn main() {
    let board = sudoku::Board::new([
        [Some(5), Some(3), None, None, Some(7), None, None, None, None],
        [Some(6), None, None, Some(1), Some(9), Some(5), None, None, None],
        [None, Some(9), Some(8), None, None, None, None, Some(6), None],

        [Some(8), None, None, None, Some(6), None, None, None, Some(3)],
        [Some(4), None, None, Some(8), None, Some(3), None, None, Some(1)],
        [Some(7), None, None, None, Some(2), None, None, None, Some(6)],

        [None, Some(6), None, None, None, None, Some(2), Some(8), None],
        [None, None, None, Some(4), Some(1), Some(9), None, None, Some(5)],
        [None, None, None, None, Some(8), None, None, Some(7), Some(9)]]);

    println!("{:?}", board);

    let visitor = AsciiPrinter::new();
    board.accept(&visitor);
}
