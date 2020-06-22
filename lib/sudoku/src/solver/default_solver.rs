use crate::{GameState, CellValue};
use std::collections::BTreeSet;

type Index = usize;
type CellValuesTuple = (Index, usize, BTreeSet<u32>);
type TrivialCellValuesTuple = (Index, BTreeSet<u32>);

pub struct DefaultSolver {}

impl DefaultSolver {
    pub fn solve(game: &GameState) -> GameState {
        let valid_symbols = Self::collect_valid_symbols(game);

        let mut stack = Vec::<GameState>::new();
        stack.push(game.fork());

        while let Some(mut state) = stack.pop() {
            let (trivial_cells, _open_cells) = Self::find_open_cells(&state, &valid_symbols);

            // TODO: Only expand trivial moves for now.
            if trivial_cells.is_empty() {
                stack.push(state);
                break;
            }

            let new_state = Self::apply_trivial_moves(state, trivial_cells);
            stack.push(new_state);
            break;
        }

        let final_state = stack.pop();
        final_state.unwrap().fork()
    }

    fn apply_trivial_moves(state: GameState, trivial_cells: Vec<(usize, BTreeSet<u32>)>) -> GameState {
        trivial_cells.iter().fold(state, |current, (index, candidates)| Self::apply_trivial_move(current, index, candidates))
    }

    fn apply_trivial_move(current: GameState, index: &usize, candidates: &BTreeSet<u32>) -> GameState {
        assert_eq!(candidates.len(), 1);

        let (x, y) = current.index_to_xy(*index);
        let value = *candidates.iter().next().unwrap();
        println!("Trivial move - placing [{}] at ({}, {})", value, x, y);

        current.place_and_fork(*index, value)
    }

    /// Finds the open cells and returns them in order of ascending move options.
    fn find_open_cells(game: &GameState, valid_symbols: &BTreeSet<u32>) -> (Vec<TrivialCellValuesTuple>, Vec<CellValuesTuple>) {
        let mut trivial_cells = Vec::<TrivialCellValuesTuple>::new();
        let mut open_cells = Vec::<CellValuesTuple>::new();

        // Iterate each empty cell
        for index in &game.missing {
            let (x, y) = game.index_to_xy(*index);

            // Determine the symbols used in the context of the current cell.
            let mut values = BTreeSet::new();
            let column = game.get_column_values(x);
            let row = game.get_row_values(y);
            let group = game.get_group_values(x, y);

            values.extend(column);
            values.extend(row);
            values.extend(group);

            // Determine the remaining possible moves for the current cell.
            let missing_values: BTreeSet<u32> = valid_symbols.difference(&values).map(|x| *x).collect();
            let length = missing_values.len();

            if length == 1 {
                let value = (*index, missing_values);
                trivial_cells.insert(trivial_cells.len(), value);
            }
            else {
                let value = (*index, missing_values.len(), missing_values);
                open_cells.insert(open_cells.len(), value);
            }
        }

        // Order by possible moves, ascending.
        open_cells.sort_unstable_by_key(|tuple| tuple.1);
        (trivial_cells, open_cells)
    }

    fn collect_valid_symbols(game: &GameState) -> BTreeSet<u32> {
        let mut symbols = BTreeSet::new();
        for symbol in game.symbols() {
            symbols.insert(*symbol);
        }
        symbols
    }
}
