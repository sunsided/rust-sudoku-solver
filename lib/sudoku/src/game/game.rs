// TODO: https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
// TODO: See https://docs.rs/array2d/0.2.1/array2d/

use visitor::{Visitor, AcceptVisitor};
use std::collections::HashSet;
use std::vec::Vec;
use std::collections::hash_map::RandomState;
use std::rc::Rc;
use std::mem::MaybeUninit;
use crate::{State, CellValue};
use crate::game::index;

type IndexSet = HashSet<usize, RandomState>;

pub struct Game {
    pub width: usize,
    pub height: usize,
    valid_symbols: [i32; 9],
    initial_state: State,
    groups: Vec<Rc<IndexSet>>,
    group_lookup: [Rc<IndexSet>; 81]
}


impl Game {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(state: [CellValue; 81]) -> Game {
        let symbols = build_default_symbols();
        let groups = build_set_of_default_groups();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game { width: 9, height: 9, valid_symbols: symbols,
            initial_state: State::new(state),
            groups, group_lookup }
    }

    pub fn new_empty() -> Game {
        let symbols = build_default_symbols();
        let groups = build_set_of_default_groups();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game { width: 9, height: 9, valid_symbols: symbols,
            initial_state: State::new([None; 81]),
            groups, group_lookup }
    }

    pub fn new_example() -> Game {
        Game::new([
            Some(5), Some(3), None, None, Some(7), None, None, None, None,
            Some(6), None, None, Some(1), Some(9), Some(5), None, None, None,
            None, Some(9), Some(8), None, None, None, None, Some(6), None,

            Some(8), None, None, None, Some(6), None, None, None, Some(3),
            Some(4), None, None, Some(8), None, Some(3), None, None, Some(1),
            Some(7), None, None, None, Some(2), None, None, None, Some(6),

            None, Some(6), None, None, None, None, Some(2), Some(8), None,
            None, None, None, Some(4), Some(1), Some(9), None, None, Some(5),
            None, None, None, None, Some(8), None, None, Some(7), Some(9)])
    }

    pub fn cell(&self, x: usize, y: usize) -> CellValue {
        self.initial_state.cell(x, y, self.width, self.height)
    }

    pub fn fork_state(&self) -> State {
        self.initial_state.fork()
    }

    pub fn group_at(&self, x: usize, y: usize) -> &IndexSet {
        &self.group_lookup[index(x, y, self.width)]
    }

    pub fn symbols(&self) -> &[i32; 9] {
        &self.valid_symbols
    }
}

impl AcceptVisitor<State> for Game {
    fn accept<V: Visitor<State>>(&self, visitor: &V) -> V::Result {
        visitor.visit(&self.initial_state)
    }
}

/// Builds a default group rooted at the specified offsets.
fn build_default_symbols() -> [i32; 9] {
    [1, 2, 3, 4, 5, 6, 7, 8, 9]
}

/// Builds a default group rooted at the specified offsets.
fn build_default_group(x_offset: usize, y_offset: usize) -> Rc<IndexSet> {
    let mut set = HashSet::new();
    for y in (0 + y_offset)..(3 + y_offset) {
        set.insert(index(0 + x_offset, y, 9));
        set.insert(index(1 + x_offset, y, 9));
        set.insert(index(2 + x_offset, y, 9));
    }
    Rc::new(set)
}

/// Builds the set of default groups.
fn build_set_of_default_groups() -> Vec<Rc<IndexSet>> {
    let mut groups = Vec::new();
    for y in (0..9).step_by(3) {
        for x in (0..9).step_by(3) {
            let set = build_default_group(x, y);
            groups.insert(groups.len(), set);
        }
    }
    groups
}

/// Builds a reverse index of each cell to its group.
fn build_default_index_to_group_lookup(groups: &Vec<Rc<IndexSet>>) -> [Rc<IndexSet>; 81] {
    let mut group_lookup: [MaybeUninit<Rc<IndexSet>>; 81] = unsafe { MaybeUninit::uninit().assume_init() };

    for group in groups {
        for index in group.iter() {
            group_lookup[*index] = MaybeUninit::new(group.clone());
        }
    }

    unsafe { std::mem::transmute::<_, [Rc<IndexSet>; 81]>(group_lookup) }
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;
    use crate::game::game::build_default_group;
    use crate::game::state::index;

    fn create_matrix() -> [crate::CellValue; 81] {
        let mut array: [MaybeUninit<crate::CellValue>; 81] = unsafe { MaybeUninit::uninit().assume_init() };

        for y in 0u8..9 {
            let offset = y * 9;
            for x in 0u8..9 {
                let index = (x + offset) as usize;
                let value = Some((x + y * 10) as u32);
                array[index] = MaybeUninit::new(value);
            }
        }

        array[5 * 9 + 0] = MaybeUninit::new(None);

        unsafe { std::mem::transmute::<_, [crate::CellValue; 81]>(array) }
    }

    #[test]
    fn construction_works() {
        let board = crate::Game::new(create_matrix());

        assert_eq!(board.cell(4, 2), Some(24));
        assert_eq!(board.cell(0, 5), None);
    }

    #[test]
    fn elements_in_group() {
        for y in (0usize..9).step_by(3) {
            for x in (0usize..9).step_by(3) {
                let set = build_default_group(x, y);
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
                let set = build_default_group(x, y);
                assert_eq!(set.contains(&index(0 + x, 3 + y, 9)), false);
                assert_eq!(set.contains(&index(3 + x, 0 + y, 9)), false);
            }
        }
    }

    #[test]
    fn group_lookup_works() {
        let board = crate::Game::new(create_matrix());
        let group = board.group_lookup[index(4, 4, 9)].clone();

        for y in 3..6 {
            assert!(group.contains(&index(3, y, 9)));
            assert!(group.contains(&index(4, y, 9)));
            assert!(group.contains(&index(5, y, 9)));
        }
    }
}
