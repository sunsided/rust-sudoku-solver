use crate::{GameState, CellValue};
use std::collections::{HashSet, BTreeSet, BTreeMap};
use std::collections::hash_map::RandomState;

type Index = usize;
type CellValuesTuple = (Index, usize, BTreeSet<u32>);

pub struct DefaultSolver {}

impl DefaultSolver {
    pub fn solve(game: &GameState) -> GameState {
        let valid_symbols = DefaultSolver::collect_valid_symbols(game);

        let mut stack = Vec::<GameState>::new();
        stack.insert(stack.len(), game.fork());

        while stack.len() > 0 {
            let open_cells = DefaultSolver::find_open_cells(&state, &valid_symbols);

            for (index, size, candidates) in open_cells.iter() {
                if *size > 1 {
                    break;
                }

                // TODO: Iterate over trivial solutions first!

                println!("Trivial move!");

                let value = *candidates.iter().next().unwrap();
                state = state.place_and_fork(*index, value);
            }
        }



        println!("{}", open_cells.len());
        state
    }

    /// Finds the open cells and returns them in order of ascending move options.
    fn find_open_cells(game: &GameState, valid_symbols: &BTreeSet<Option<u32>>) -> Vec<(usize, usize, BTreeSet<u32>)> {
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

            let value = (*index, missing_values.len(), missing_values);
            open_cells.insert(open_cells.len(), value);
        }

        // Order by possible moves, ascending.
        open_cells.sort_unstable_by_key(|tuple| tuple.1);
        open_cells
    }

    fn collect_valid_symbols(game: &GameState) -> BTreeSet<CellValue> {
        let mut symbols: BTreeSet<CellValue> = BTreeSet::new();
        for symbol in game.symbols() {
            symbols.insert(Some(*symbol));
        }
        symbols
    }
}
