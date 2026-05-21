mod hidden_singles;
mod lone_singles;
mod naked_twins;

use crate::solver::candidates::SetOfMoveCandidates;
use crate::{GameState, Placement};
pub(super) use hidden_singles::hidden_singles;
pub(super) use lone_singles::lone_singles;
pub(super) use naked_twins::naked_twins;

pub type StrategyFn =
    fn(&mut GameState, &SetOfMoveCandidates) -> Result<StrategyMove, StrategyError>;

pub enum StrategyMove {
    /// The strategy yielded no result.
    None,
    /// Placements the solver should apply to the board and eliminate
    /// from all peers. Strategies must not mutate `state` themselves.
    Applied(Vec<Placement>),
    /// Candidates the solver should remove from the candidate set without
    /// touching the board state.
    EliminateOnly(Vec<Placement>),
}

#[derive(thiserror::Error, Debug)]
pub enum StrategyError {
    #[error("The board is invalid")]
    BoardInvalid,
}
