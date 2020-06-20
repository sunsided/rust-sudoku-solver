// TODO: https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
// TODO: See https://docs.rs/array2d/0.2.1/array2d/

use visitor::{Visitor, AcceptVisitor};

pub type Cell = Option<u8>;

fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

pub struct Board {
    width: usize,
    height: usize,
    cells: [Cell; 9 * 9]
}

impl Board {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(cells: [Cell; 9 * 9]) -> Board {
        Board { width: 9, height: 9, cells }
    }

    pub fn new_empty() -> Board {
        Board { width: 9, height: 9, cells: [None; 9* 9] }
    }

    pub fn cell(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.width && y < self.height);
        self.cells[index(x, y, self.width)]
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

    fn create_matrix() -> [crate::Cell; 81] {
        let mut array: [MaybeUninit<crate::Cell>; 81] = unsafe { MaybeUninit::uninit().assume_init() };

        for y in 0u8..9 {
            let offset = y * 9;
            for x in 0u8..9 {
                let index = (x + offset) as usize;
                let value = Some(x + y * 10);
                array[index] = MaybeUninit::new(value);
            }
        }

        array[5 * 9 + 0] = MaybeUninit::new(None);

        unsafe { std::mem::transmute::<_, [crate::Cell; 81]>(array) }
    }

    #[test]
    fn construction_works() {
        let board = crate::Board::new(create_matrix());

        assert_eq!(board.cell(4, 2), Some(24));
        assert_eq!(board.cell(0, 5), None);
    }
}
