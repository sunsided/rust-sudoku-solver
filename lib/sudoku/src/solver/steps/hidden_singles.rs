use crate::GameState;
use crate::game::Placement;
use crate::prelude::{Index, Value};
use crate::solver::candidates::SetOfMoveCandidates;
use crate::solver::steps::{StrategyError, StrategyMove};
use log::trace;
use std::collections::{HashMap, HashSet};

pub fn hidden_singles(
    state: &mut GameState,
    candidates: &SetOfMoveCandidates,
) -> Result<StrategyMove, StrategyError> {
    let mut value_indexes: HashMap<Value, HashSet<Index>> = HashMap::new();
    for candidate_set in candidates.iter() {
        for placement in &candidate_set.moves {
            value_indexes
                .entry(placement.value)
                .or_default()
                .insert(placement.index);
        }
    }

    // Return a single hidden single per call so the solver can re-derive
    // candidates between placements. Detecting multiple in one pass can
    // surface inconsistent inferences that share a peer.
    for y in 0..state.game.height {
        let row_indexes: Vec<Index> = (0..state.game.width)
            .map(|x| state.xy_to_index(x, y))
            .collect();
        if let Some(placement) = find_hidden_single_in_unit(state, &value_indexes, &row_indexes) {
            return Ok(into_applied(placement));
        }
    }

    for x in 0..state.game.width {
        let col_indexes: Vec<Index> = (0..state.game.height)
            .map(|y| state.xy_to_index(x, y))
            .collect();
        if let Some(placement) = find_hidden_single_in_unit(state, &value_indexes, &col_indexes) {
            return Ok(into_applied(placement));
        }
    }

    for group in state.game.groups.iter() {
        let group_indexes: Vec<Index> = group.iter().collect();
        if let Some(placement) = find_hidden_single_in_unit(state, &value_indexes, &group_indexes) {
            return Ok(into_applied(placement));
        }
    }

    Ok(StrategyMove::None)
}

fn into_applied(placement: Placement) -> StrategyMove {
    trace!(
        "  * Hidden single {value} at index {index}",
        value = placement.value,
        index = placement.index
    );
    StrategyMove::Applied(vec![placement])
}

fn find_hidden_single_in_unit(
    state: &GameState,
    value_indexes: &HashMap<Value, HashSet<Index>>,
    unit: &[Index],
) -> Option<Placement> {
    for value in state.valid_symbols() {
        let Some(indexes_for_value) = value_indexes.get(value) else {
            continue;
        };
        let mut single: Option<Index> = None;
        let mut count = 0u32;
        for &idx in unit {
            if indexes_for_value.contains(&idx) {
                single = Some(idx);
                count += 1;
                if count > 1 {
                    break;
                }
            }
        }
        if count == 1 {
            if let Some(idx) = single {
                return Some(Placement::new(*value, idx));
            }
        }
    }
    None
}

