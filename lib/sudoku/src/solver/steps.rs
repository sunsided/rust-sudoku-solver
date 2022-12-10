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
    /// Lists the moves to apply and to eliminate in all peers.
    Applied(Vec<Placement>),
    /// Lists the placements to eliminate.
    EliminateOnly(Vec<Placement>),
}

#[derive(thiserror::Error, Debug)]
pub enum StrategyError {
    #[error("The board is invalid")]
    BoardInvalid,
}
