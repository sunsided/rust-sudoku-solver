// TODO: https://stackoverflow.com/questions/27673674/is-there-a-way-to-create-a-data-type-that-only-accepts-a-range-of-values
// TODO: See https://docs.rs/array2d/0.2.1/array2d/

use crate::prelude::*;
use crate::State;
use std::collections::{BTreeSet, HashSet};
use std::mem::MaybeUninit;
use std::rc::Rc;
use std::vec::Vec;
use visitor::{AcceptVisitor, Visitor};

pub struct Game {
    pub width: usize,
    pub height: usize,
    valid_symbols: [Value; 9],
    initial_state: State,
    pub groups: Vec<Rc<IndexSet>>,
    group_lookup: [usize; 81],
}

impl Game {
    /// Initializes a standard Sudoku board from values in row-major order.
    pub fn new(state: [ValueOption; 81]) -> Game {
        let symbols = build_default_symbols();
        let groups = build_set_of_default_groups();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game {
            width: 9,
            height: 9,
            valid_symbols: symbols,
            initial_state: State::new(state),
            groups,
            group_lookup,
        }
    }

    pub fn new_with_groups(state: [ValueOption; 81], groups: Vec<Rc<IndexSet>>) -> Game {
        let symbols = build_default_symbols();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game {
            width: 9,
            height: 9,
            valid_symbols: symbols,
            initial_state: State::new(state),
            groups,
            group_lookup,
        }
    }

    pub fn new_empty() -> Game {
        let symbols = build_default_symbols();
        let groups = build_set_of_default_groups();
        let group_lookup = build_default_index_to_group_lookup(&groups);
        Game {
            width: 9,
            height: 9,
            valid_symbols: symbols,
            initial_state: State::new([None; 81]),
            groups,
            group_lookup,
        }
    }

    pub fn new_example() -> Game {
        Game::new([
            Some(Value::try_from(5).unwrap()),
            Some(Value::try_from(3).unwrap()),
            None,
            None,
            Some(Value::try_from(7).unwrap()),
            None,
            None,
            None,
            None,
            Some(Value::try_from(6).unwrap()),
            None,
            None,
            Some(Value::try_from(1).unwrap()),
            Some(Value::try_from(9).unwrap()),
            Some(Value::try_from(5).unwrap()),
            None,
            None,
            None,
            None,
            Some(Value::try_from(9).unwrap()),
            Some(Value::try_from(8).unwrap()),
            None,
            None,
            None,
            None,
            Some(Value::try_from(6).unwrap()),
            None,
            Some(Value::try_from(8).unwrap()),
            None,
            None,
            None,
            Some(Value::try_from(6).unwrap()),
            None,
            None,
            None,
            Some(Value::try_from(3).unwrap()),
            Some(Value::try_from(4).unwrap()),
            None,
            None,
            Some(Value::try_from(8).unwrap()),
            None,
            Some(Value::try_from(3).unwrap()),
            None,
            None,
            Some(Value::try_from(1).unwrap()),
            Some(Value::try_from(7).unwrap()),
            None,
            None,
            None,
            Some(Value::try_from(2).unwrap()),
            None,
            None,
            None,
            Some(Value::try_from(6).unwrap()),
            None,
            Some(Value::try_from(6).unwrap()),
            None,
            None,
            None,
            None,
            Some(Value::try_from(2).unwrap()),
            Some(Value::try_from(8).unwrap()),
            None,
            None,
            None,
            None,
            Some(Value::try_from(4).unwrap()),
            Some(Value::try_from(1).unwrap()),
            Some(Value::try_from(9).unwrap()),
            None,
            None,
            Some(Value::try_from(5).unwrap()),
            None,
            None,
            None,
            None,
            Some(Value::try_from(8).unwrap()),
            None,
            None,
            Some(Value::try_from(7).unwrap()),
            Some(Value::try_from(9).unwrap()),
        ])
    }

    pub fn new_example_nonomino() -> Game {
        let mut index_set = Vec::new();
        index_set.push(Rc::new(hashset!(0, 1, 2, 9, 10, 11, 18, 27, 28)));
        index_set.push(Rc::new(hashset!(3, 12, 13, 14, 23, 24, 25, 34, 35)));
        index_set.push(Rc::new(hashset!(4, 5, 6, 7, 8, 15, 16, 17, 26)));
        index_set.push(Rc::new(hashset!(19, 20, 21, 22, 29, 36, 37, 38, 39)));
        index_set.push(Rc::new(hashset!(30, 31, 32, 33, 40, 47, 48, 49, 50)));
        index_set.push(Rc::new(hashset!(41, 42, 43, 44, 51, 58, 59, 60, 61)));
        index_set.push(Rc::new(hashset!(45, 46, 55, 56, 57, 66, 67, 68, 77)));
        index_set.push(Rc::new(hashset!(54, 63, 64, 65, 72, 73, 74, 75, 76)));
        index_set.push(Rc::new(hashset!(52, 53, 62, 69, 70, 71, 78, 79, 80)));

        Game::new_with_groups(
            [
                Some(Value::try_from(3).unwrap()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Value::try_from(4).unwrap()),
                None,
                None,
                Some(Value::try_from(2).unwrap()),
                None,
                Some(Value::try_from(6).unwrap()),
                None,
                Some(Value::try_from(1).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(1).unwrap()),
                None,
                Some(Value::try_from(9).unwrap()),
                None,
                Some(Value::try_from(8).unwrap()),
                None,
                Some(Value::try_from(2).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(5).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(6).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(2).unwrap()),
                None,
                None,
                None,
                None,
                None,
                Some(Value::try_from(1).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(9).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(8).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(8).unwrap()),
                None,
                Some(Value::try_from(3).unwrap()),
                None,
                Some(Value::try_from(4).unwrap()),
                None,
                Some(Value::try_from(6).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(4).unwrap()),
                None,
                Some(Value::try_from(1).unwrap()),
                None,
                Some(Value::try_from(9).unwrap()),
                None,
                None,
                Some(Value::try_from(5).unwrap()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Value::try_from(7).unwrap()),
            ],
            index_set,
        )
    }

    pub fn new_example_hypersudoku() -> Game {
        let mut index_set = Vec::new();

        // Regular grid.
        index_set.push(Rc::new(hashset!(0, 1, 2, 9, 10, 11, 18, 19, 20)));
        index_set.push(Rc::new(hashset!(3, 4, 5, 12, 13, 14, 21, 22, 23)));
        index_set.push(Rc::new(hashset!(6, 7, 8, 15, 16, 17, 24, 25, 26)));
        index_set.push(Rc::new(hashset!(27, 28, 29, 36, 37, 38, 45, 46, 47)));
        index_set.push(Rc::new(hashset!(30, 31, 32, 39, 40, 41, 48, 49, 50)));
        index_set.push(Rc::new(hashset!(33, 34, 35, 42, 43, 44, 51, 52, 53)));
        index_set.push(Rc::new(hashset!(54, 55, 56, 63, 64, 65, 72, 73, 74)));
        index_set.push(Rc::new(hashset!(57, 58, 59, 66, 67, 68, 75, 76, 77)));
        index_set.push(Rc::new(hashset!(60, 61, 62, 69, 70, 71, 78, 79, 80)));

        // Windows
        index_set.push(Rc::new(hashset!(10, 11, 12, 19, 20, 21, 28, 29, 30)));
        index_set.push(Rc::new(hashset!(14, 15, 16, 23, 24, 25, 32, 33, 34)));
        index_set.push(Rc::new(hashset!(46, 47, 48, 55, 56, 57, 64, 65, 66)));
        index_set.push(Rc::new(hashset!(50, 51, 52, 59, 60, 61, 68, 69, 70)));

        Game::new_with_groups(
            [
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Value::try_from(1).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(2).unwrap()),
                None,
                None,
                None,
                None,
                Some(Value::try_from(3).unwrap()),
                Some(Value::try_from(4).unwrap()),
                None,
                None,
                None,
                None,
                Some(Value::try_from(5).unwrap()),
                Some(Value::try_from(1).unwrap()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Value::try_from(6).unwrap()),
                Some(Value::try_from(5).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(7).unwrap()),
                None,
                Some(Value::try_from(3).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(8).unwrap()),
                None,
                None,
                None,
                Some(Value::try_from(3).unwrap()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Value::try_from(8).unwrap()),
                None,
                None,
                None,
                None,
                Some(Value::try_from(5).unwrap()),
                Some(Value::try_from(8).unwrap()),
                None,
                None,
                None,
                None,
                Some(Value::try_from(9).unwrap()),
                None,
                None,
                Some(Value::try_from(6).unwrap()),
                Some(Value::try_from(9).unwrap()),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
            ],
            index_set,
        )
    }

    pub fn cell(&self, x: usize, y: usize) -> ValueOption {
        self.initial_state.cell_at_xy(x, y, self.width, self.height)
    }

    pub fn fork_state(&self) -> State {
        self.initial_state.clone()
    }

    pub fn group_id(&self, x: usize, y: usize) -> usize {
        self.group_lookup[index(x, y, self.width)]
    }

    pub fn group_at(&self, x: usize, y: usize) -> &IndexSet {
        let idx = self.group_id(x, y);
        &self.groups[idx]
    }

    pub fn valid_symbols(&self) -> &[Value; 9] {
        &self.valid_symbols
    }
}

impl AcceptVisitor<State> for Game {
    fn accept<V: Visitor<State>>(&self, visitor: &V) -> V::Result {
        visitor.visit(&self.initial_state)
    }
}

/// Builds a default group rooted at the specified offsets.
fn build_default_symbols() -> [Value; 9] {
    [
        Value::try_from(1).unwrap(),
        Value::try_from(2).unwrap(),
        Value::try_from(3).unwrap(),
        Value::try_from(4).unwrap(),
        Value::try_from(5).unwrap(),
        Value::try_from(6).unwrap(),
        Value::try_from(7).unwrap(),
        Value::try_from(8).unwrap(),
        Value::try_from(9).unwrap(),
    ]
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
            groups.push(set);
        }
    }
    groups
}

fn groups_valid(groups: &Vec<Rc<IndexSet>>) -> bool {
    let mut set = BTreeSet::<usize>::new();
    for group in groups {
        set.extend(group.iter());
    }
    set.len() == 81
}

/// Builds a reverse index of each cell to its group.
fn build_default_index_to_group_lookup(groups: &Vec<Rc<IndexSet>>) -> [usize; 81] {
    assert!(groups_valid(&groups));

    let mut group_lookup: [MaybeUninit<usize>; 81] = unsafe { MaybeUninit::uninit().assume_init() };

    for gid in 0..groups.len() {
        let group = &groups[gid];
        for index in group.iter() {
            group_lookup[*index] = MaybeUninit::new(gid);
        }
    }

    unsafe { std::mem::transmute::<_, [usize; 81]>(group_lookup) }
}

#[cfg(test)]
mod tests {
    use crate::game::game::build_default_group;
    use crate::prelude::*;
    use std::mem::MaybeUninit;

    fn create_matrix() -> [ValueOption; 81] {
        let mut array: [MaybeUninit<ValueOption>; 81] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for y in 0u8..9 {
            let offset = y * 9;
            for x in 0u8..9 {
                let index = (x + offset) as usize;
                let value = Some((x + y * 10) as Value);
                array[index] = MaybeUninit::new(value);
            }
        }

        array[5 * 9 + 0] = MaybeUninit::new(None);

        unsafe { std::mem::transmute::<_, [ValueOption; 81]>(array) }
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
        let group = board.group_at(4, 4).clone();

        for y in 3..6 {
            assert!(group.contains(&index(3, y, 9)));
            assert!(group.contains(&index(4, y, 9)));
            assert!(group.contains(&index(5, y, 9)));
        }
    }
}
