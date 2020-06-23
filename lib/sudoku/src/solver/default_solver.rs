use crate::GameState;
use std::collections::BTreeSet;

type Index = usize;
type CellValuesTuple = (Index, Vec<u32>);
type TrivialCellValuesTuple = (Index, u32);


pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);

    let mut stack = Vec::new();
    stack.push(game.fork());

    while let Some(state) = stack.pop() {
        let (trivial_cells, open_cells) = find_move_candidates(&state, &valid_symbols);

        // TODO: Only expand trivial moves for now.
        if trivial_cells.is_empty() {
            assert!(open_cells.is_empty());
            stack.push(state);
            break;
        }

        let new_state = apply_trivial_moves(state, trivial_cells);
        stack.push(new_state);
    }

    let final_state = stack.pop();
    final_state.unwrap()
}

fn apply_trivial_moves(state: GameState, trivial_cells: Vec<(usize, u32)>) -> GameState {
    trivial_cells.iter().fold(state, |current, (index, candidates)|
        current.place_and_fork(*index, *candidates))
}

/// Finds the open cells and returns them in order of ascending move options.
fn find_move_candidates(state: &GameState, valid_symbols: &BTreeSet<u32>) -> (Vec<TrivialCellValuesTuple>, Vec<CellValuesTuple>) {
    let mut trivial_cells = Vec::new();
    let mut open_cells = Vec::new();

    for index in &state.empty_cells {
        let missing_values = collect_missing_values(index, state, valid_symbols);
        match missing_values.len() {
            1 => trivial_cells.push( (*index, *missing_values.iter().next().unwrap())),
            _ => open_cells.push((*index, missing_values))
        };
    }

    // Order by possible moves, ascending.
    open_cells.sort_unstable_by_key(|tuple| tuple.1.len());
    (trivial_cells, open_cells)
}

fn collect_missing_values(index: &usize, state: &GameState, valid_symbols: &BTreeSet<u32>) -> Vec<u32> {
    let (x, y) = state.index_to_xy(*index);

    // Determine the symbols used in the context of the current cell.
    let column = state.get_column_values(x);
    let row = state.get_row_values(y);
    let group = state.get_group_values(x, y);

    // Determine the remaining possible moves for the current cell.
    let values = join_btreeset!(column, row, group);
    valid_symbols.difference(&values).map(|x| *x).collect()
}

fn collect_valid_symbols(game: &GameState) -> BTreeSet<u32> {
    let mut symbols = BTreeSet::new();
    for symbol in game.symbols() {
        symbols.insert(*symbol);
    }
    symbols
}

