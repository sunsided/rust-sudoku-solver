use crate::game::Placement;
use crate::solver::candidates::SetOfMoveCandidates;
use crate::GameState;

pub fn lone_singles(state: &mut GameState, candidates: &SetOfMoveCandidates) -> Vec<Placement> {
    let mut applied = Vec::new();

    for candidate in candidates.iter() {
        if !candidate.is_trivial() {
            continue;
        }

        for r#move in &candidate.moves {
            state.apply_move(r#move);
        }

        applied.extend(candidate.moves);
    }

    applied
}
