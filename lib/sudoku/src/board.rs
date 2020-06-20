// TODO: https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
// TODO: See https://docs.rs/array2d/0.2.1/array2d/

use visitor::{Visitor, AcceptVisitor};
use std::collections::HashSet;
use std::vec::Vec;
use std::collections::hash_map::RandomState;

pub type Cell = Option<u8>;

pub struct Board {
    width: usize,
    height: usize,
    cells: [Cell; 9 * 9],
    groups: Vec<HashSet<usize, RandomState>>
}

impl Board {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(cells: [Cell; 9 * 9]) -> Board {
        let mut groups = build_set_of_default_sets();
        Board { width: 9, height: 9, cells, groups }
    }

    pub fn new_empty() -> Board {
        let mut groups = build_set_of_default_sets();
        Board { width: 9, height: 9, cells: [None; 81], groups }
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

fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
}

fn build_default_set(x_offset: usize, y_offset: usize, width: usize) -> HashSet<usize, RandomState> {
    let mut set = HashSet::new();
    for y in 0..3 {
        set.insert(index(0 + x_offset, y + y_offset, width));
        set.insert(index(1 + x_offset, y + y_offset, width));
        set.insert(index(2 + x_offset, y + y_offset, width));
    }
    set
}

fn build_set_of_default_sets() -> Vec<HashSet<usize, RandomState>> {
    let mut groups = Vec::new();
    for y in (0..9).step_by(3) {
        for x in (0..9).step_by(3) {
            let set = build_default_set(x, y, 9);
            groups.insert(groups.len(), set);
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;
    use crate::board::{build_default_set, index};

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

    #[test]
    fn elements_in_group() {
        for y in (0usize..9).step_by(3) {
            for x in (0usize..9).step_by(3) {
                let set = build_default_set(x, y, 9);
                assert_eq!(set.contains(&index(0 + x, 0 + y, 9)), true);
                assert_eq!(set.contains(&index(0 + x, 2 + y, 9)), true);
                assert_eq!(set.contains(&index(2 + x, 2 + y, 9)), true);
            }
        }
    }

    #[test]
    fn elements_not_in_group() {
        for y in (0usize..9).step_by(3) {
            for x in (0usize..9).step_by(3) {
                let set = build_default_set(x, y, 9);
                assert_eq!(set.contains(&index(0 + x, 3 + y, 9)), false);
                assert_eq!(set.contains(&index(3 + x, 0 + y, 9)), false);
            }
        }
    }
}
