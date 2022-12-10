use crate::game::Placement;
use crate::solver::candidates::SetOfMoveCandidates;
use crate::solver::steps::{StrategyError, StrategyMove};
use crate::GameState;
use log::trace;

pub fn lone_singles(
    state: &mut GameState,
    candidates: &SetOfMoveCandidates,
) -> Result<StrategyMove, StrategyError> {
    let mut applied = Vec::new();

    for candidate in candidates.iter() {
        if !candidate.is_trivial() {
            continue;
        }

        for r#move in &candidate.moves {
            trace!(
                "  * Lone single {value} at index {index}",
                value = r#move.value,
                index = r#move.index
            );
            state.apply_move(r#move);
        }

        applied.extend(candidate.moves);
    }

    if applied.is_empty() {
        Ok(StrategyMove::None)
    } else {
        Ok(StrategyMove::Applied(applied))
    }
}
