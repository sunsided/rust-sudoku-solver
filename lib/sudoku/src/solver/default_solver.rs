use std::collections::{HashSet, BTreeMap, HashMap};
use log::debug;

use crate::prelude::*;
use crate::GameState;
use crate::game::{Placement, CollectType};
use crate::solver::candidates::{find_move_candidates, MoveCandidates, SetOfMoveCandidates};
use crate::solver::steps::{lone_singles, naked_twins, hidden_singles};
use std::iter::FromIterator;
use std::iter::IntoIterator;

pub fn solve(game: &GameState) -> GameState {
    let valid_symbols = collect_valid_symbols(game);
    let initial_candidates = find_move_candidates(&game, &valid_symbols);

    let mut stack = Vec::new();
    stack.push((game.clone(), initial_candidates));

    let mut tried_moves = HashSet::new();

    'stack: while let Some((mut state, mut candidates)) = stack.pop() {
        debug!("Stack depth: {}", stack.len());

        if candidates.is_empty() {
            return state;
        }

        if !is_solvable(&state, &candidates) {
            continue 'stack
        }

        let mut applied_some = true;
        while applied_some {
            applied_some = false;

            debug!("  - {} candidates remaining", candidates.total_len());

            // Apply lone singles strategy.
            if let Ok(applied) = apply_simple_strategy(&(lone_singles as StrategyFn), &mut state, &mut candidates) {
                applied_some |= applied;
            }
            else {
                continue 'stack;
            }

            // Apply hidden singles strategy.
            if let Ok(applied) = apply_simple_strategy(&(hidden_singles as StrategyFn), &mut state, &mut candidates) {
                applied_some |= applied;
            }
            else {
                continue 'stack;
            }

            /*
            // Apply naked twins strategy.
            if naked_twins(&mut state, &mut candidates) {
                // If an invalid move was made here or the board isn't solvable, leave this branch.
                if !state.validate(true) {
                    println!("Branch is invalid.");
                    continue 'stack;
                }

                applied_some = true;
            }
             */
        }

        // Sanity check.
        if !is_solvable(&state, &candidates) {
            continue 'stack
        }

        // If the state didn't change, we need to fork.
        let mut sorted_candidates: Vec<MoveCandidates> = Vec::from_iter(candidates.iter());
        sorted_candidates.sort_by_key(|v| v.moves.len());

        for candidate_set in sorted_candidates {
            for candidate in candidate_set.moves {
                let key = (state.id().clone(), candidate.clone());
                assert!(!tried_moves.contains(&key));

                tried_moves.insert(key);

                let branch = state.apply_and_fork(candidate.index, candidate.value);
                let branch_candidates = find_move_candidates(&branch, &valid_symbols);

                assert!(branch.validate(true));

                // We remove (not eliminate!) the candidate we just tried and requeue the current
                // branch if it still contains options.
                candidates.remove_candidate(&candidate);
                if !candidates.is_empty() {
                    debug!("  + Pushing base branch");
                    stack.push((state, candidates));
                }

                debug!("  + branching; {} candidates to explore", branch_candidates.total_len());
                stack.push((branch, branch_candidates));
                continue 'stack;
            }
        }

        // All possible options were exhausted - this branch is a dead end.
        if state.validate(false) {
            return state;
        }
    }

    panic!()
}

pub type StrategyFn = fn(&mut GameState, &SetOfMoveCandidates) -> Vec<Placement>;

fn apply_simple_strategy(strategy: &StrategyFn, mut state: &mut GameState, mut candidates: &mut SetOfMoveCandidates) -> Result<bool, bool> {
    let mut applied_some = true;
    while applied_some {
        applied_some = false;

        let applied = strategy(&mut state, &candidates);
        if !applied.is_empty() {
            eliminate_many(&state, &mut candidates, applied.into_iter());
            debug!("  - Candidates left after applying strategy: {}.", candidates.total_len());

            // If an invalid move was made here or the board isn't solvable, leave this branch.
            if !state.validate(true) {
                debug!("  ! Branch is invalid.");
                return Err(false);
            }

            applied_some = true;
        }
    }
    Ok(applied_some)
}

fn eliminate(state: &GameState, candidates: &mut SetOfMoveCandidates, placement: &Placement) {
    let peers = state.peer_indexes_by_index(placement.index, false, CollectType::All);
    for peer_index in peers {
        candidates.remove_candidate(&Placement::new(placement.value, peer_index));
    }
}

fn eliminate_many(state: &GameState, candidates: &mut SetOfMoveCandidates, applied: impl Iterator<Item=Placement>) {
    for placement in applied {
        eliminate(state, candidates, &placement);
    }
}

fn is_solvable(state: &GameState, candidates: &SetOfMoveCandidates) -> bool {
    state.empty_cells.len() == candidates.len()
}

fn collect_valid_symbols(game: &GameState) -> HashSet<Value> {
    let mut symbols = HashSet::new();
    for symbol in game.valid_symbols() {
        symbols.insert(*symbol);
    }
    symbols
}

fn to_value_vec(set: HashSet<Placement>) -> Vec<Value> {
    let mut vec = Vec::new();
    for value in set.into_iter() {
        vec.push(value.value);
    }
    vec
}