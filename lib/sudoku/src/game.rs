// TODO: https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
// TODO: See https://docs.rs/array2d/0.2.1/array2d/

use visitor::{Visitor, AcceptVisitor};
use std::collections::HashSet;
use std::vec::Vec;
use std::collections::hash_map::RandomState;
use std::rc::Rc;
use std::mem::MaybeUninit;

pub type State = Option<u32>;
type IndexSet = HashSet<usize, RandomState>;

pub struct Game {
    width: usize,
    height: usize,
    valid_symbols: [i32; 9],
    initial_state: [State; 81],
    groups: Vec<Rc<IndexSet>>,
    group_lookup: [Rc<IndexSet>; 81]
}

impl Game {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(state: [State; 81]) -> Game {
        let symbols = build_default_symbols();
        let groups = build_set_of_default_groups();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game { width: 9, height: 9, valid_symbols: symbols, initial_state: state, groups, group_lookup }
    }

    pub fn new_empty() -> Game {
        let symbols = build_default_symbols();
        let groups = build_set_of_default_groups();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game { width: 9, height: 9, valid_symbols: symbols, initial_state: [None; 81], groups, group_lookup }
    }

    pub fn cell(&self, x: usize, y: usize) -> State {
        assert!(x < self.width && y < self.height);
        self.initial_state[index(x, y, self.width)]
    }
}

impl AcceptVisitor<Game> for Game {
    fn accept<V: Visitor<Game>>(&self, visitor: &V) -> V::Result {
        visitor.visit(self)
    }
}

fn index(x: usize, y: usize, width: usize) -> usize {
    x + y * width
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
    use crate::game::{build_default_group, index};

    fn create_matrix() -> [crate::State; 81] {
        let mut array: [MaybeUninit<crate::State>; 81] = unsafe { MaybeUninit::uninit().assume_init() };

        for y in 0u8..9 {
            let offset = y * 9;
            for x in 0u8..9 {
                let index = (x + offset) as usize;
                let value = Some((x + y * 10) as u32);
                array[index] = MaybeUninit::new(value);
            }
        }

        array[5 * 9 + 0] = MaybeUninit::new(None);

        unsafe { std::mem::transmute::<_, [crate::State; 81]>(array) }
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
