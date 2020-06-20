// TODO: https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
// TODO: See https://docs.rs/array2d/0.2.1/array2d/

use visitor::{Visitor, AcceptVisitor};

pub type Cell = Option<u8>;

#[derive(Debug)]
pub struct Board {
    cells: [[Cell; 9]; 9]
}

impl Board {
    pub fn new(cells: [[Cell; 9]; 9]) -> Board {
        Board { cells }
    }

    pub fn new_empty() -> Board {
        Board { cells: [[None; 9]; 9] }
    }

    pub fn cell(&self, x: usize, y: usize) -> Cell {
        assert!(x < 9 && y < 9);
        self.cells[x][y]
    }
}

impl AcceptVisitor<Board> for Board {
    fn accept<V: Visitor<Board>>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    fn create_row(row: u8) -> [crate::Cell; 9] {
        let y = row * 10;

        // The official way to not have to initialize arrays upon creation.
        let mut array: [MaybeUninit<crate::Cell>; 9] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..9 {
            array[i] = MaybeUninit::new(Some(i as u8 + y));
        }
        array[5] = MaybeUninit::new(None);

        unsafe { std::mem::transmute::<_, [crate::Cell; 9]>(array) }
    }

    #[test]
    fn construction_works() {
        let board = crate::Board::new([
            create_row(0),
            create_row(1),
            create_row(2),
            create_row(3),
            create_row(4),
            create_row(5),
            create_row(6),
            create_row(7),
            create_row(8)]);

        assert_eq!(board.cell(4, 2), Some(42));
        assert_eq!(board.cell(0, 5), None);
    }
}
