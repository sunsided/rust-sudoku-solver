use crate::game::CollectType;
use crate::prelude::{Index, Value};
use crate::solver::candidates::SetOfMoveCandidates;
use crate::solver::steps::{StrategyError, StrategyMove};
use crate::{GameState, Placement};
use log::trace;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
struct Twins {
    peer_a: Index,
    peer_b: Index,
    moves: HashSet<Value>,
}

impl Hash for Twins {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.peer_a.hash(state);
        self.peer_b.hash(state);
    }
}

pub fn naked_twins(
    state: &mut GameState,
    candidates: &SetOfMoveCandidates,
) -> Result<StrategyMove, StrategyError> {
    let mut twins: HashSet<Twins, RandomState> = HashSet::default();

    // For each cell we have possible moves for, find those with exactly two possible values.
    for candidate in candidates.iter().filter(|c| c.len() == 2) {
        let candidate_values = candidate.value_hashset();

        // Find all peers of the current index and obtain their move candidates.
        // keep those that have exactly two values.
        let indexes = state.peer_indexes_by_index(candidate.index, true, CollectType::Empty);
        for peer in candidates.iter().filter(|peer| {
            peer.len() == 2 && peer.index != candidate.index && indexes.contains(&peer.index)
        }) {
            debug_assert_ne!(candidate.index, peer.index);
            let peer_values = peer.value_hashset();
            if !peer_values.eq(&candidate_values) {
                continue;
            }

            // Register the pair for later processing.
            // If the exact same entry already existed, we can ignore it - it's the twin.
            // To ensure this, we always order the indexes in ascending order.
            if twins.insert(Twins {
                peer_a: candidate.index.min(peer.index),
                peer_b: candidate.index.max(peer.index),
                moves: peer_values,
            }) {
                trace!(
                    "  * Naked twins at indexes {index1} and {index2}",
                    index1 = candidate.index,
                    index2 = peer.index,
                );
            }
        }
    }

    if twins.is_empty() {
        return Ok(StrategyMove::None);
    }

    let mut eliminations = Vec::default();
    for twin in twins.iter() {
        let shared_indexes = shared_group_indexes(state, twin.peer_a, twin.peer_b);
        for peer in candidates
            .iter()
            .filter(|peer| shared_indexes.contains(&peer.index))
        {
            for possible_value in twin.moves.iter() {
                let placement = Placement::new(*possible_value, peer.index);
                if peer.moves.contains(&placement) {
                    eliminations.push(placement);
                }
            }
        }
    }

    if eliminations.is_empty() {
        return Ok(StrategyMove::None);
    }

    trace!(
        "  * Naked twins eliminate {count} candidates",
        count = eliminations.len()
    );

    Ok(StrategyMove::EliminateOnly(eliminations))
}

/// Returns indexes that are peers of both twin cells, matching the peer logic
/// used elsewhere in the solver.
fn shared_group_indexes(state: &GameState, a: Index, b: Index) -> HashSet<Index> {
    let peers_a: HashSet<Index> = state
        .peer_indexes_by_index(a, true, CollectType::All)
        .into_iter()
        .collect();
    let peers_b: HashSet<Index> = state
        .peer_indexes_by_index(b, true, CollectType::All)
        .into_iter()
        .collect();
    peers_a.intersection(&peers_b).copied().collect()
}
