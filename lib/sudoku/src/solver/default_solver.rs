use crate::{GameState, CellValue};
use std::collections::BTreeSet;

type Index = usize;
type CellValuesTuple = (Index, usize, BTreeSet<u32>);
type TrivialCellValuesTuple = (Index, BTreeSet<u32>);

pub struct DefaultSolver {}

impl DefaultSolver {
    pub fn solve(game: &GameState) -> GameState {
        let valid_symbols = DefaultSolver::collect_valid_symbols(game);

        let mut stack = Vec::<GameState>::new();
        stack.push(game.fork());

        while stack.len() > 1 {
            let state = stack.last().unwrap();
            let (trivial_cells, _open_cells) = DefaultSolver::find_open_cells(&state, &valid_symbols);

            // Iterate over trivial solutions first
            for (index, candidates) in trivial_cells.iter() {
                assert_eq!(candidates.len(), 1);
                println!("Trivial move!");

                let value = *candidates.iter().next().unwrap();
                stack.push(state.place_and_fork(*index, value));
            }
        }

        let final_state = stack.last();
        final_state.unwrap().fork()
    }

    /// Finds the open cells and returns them in order of ascending move options.
    fn find_open_cells(game: &GameState, valid_symbols: &BTreeSet<Option<u32>>) -> (Vec<TrivialCellValuesTuple>, Vec<CellValuesTuple>) {
        let mut trivial_cells = Vec::<TrivialCellValuesTuple>::new();
        let mut open_cells = Vec::<CellValuesTuple>::new();

        // Iterate each empty cell
        for index in &game.missing {
            let (x, y) = game.index_to_xy(*index);

            // Determine the symbols used in the context of the current cell.
            let mut values: BTreeSet<CellValue> = BTreeSet::new();
            values.extend(game.get_column_values(x));
            values.extend(game.get_row_values(y));
            values.extend(game.get_group_values(x, y));

            // Determine the remaining possible moves for the current cell.
            let missing_values: BTreeSet<u32> = valid_symbols.difference(&values).map(|x| x.unwrap()).collect();
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

    fn collect_valid_symbols(game: &GameState) -> BTreeSet<CellValue> {
        let mut symbols: BTreeSet<CellValue> = BTreeSet::new();
        for symbol in game.symbols() {
            symbols.insert(Some(*symbol));
        }
        symbols
    }
}
