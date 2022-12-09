use log::debug;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::iter::IntoIterator;

use crate::game::{CollectType, Placement};
use crate::prelude::*;
use crate::solver::candidates::{find_move_candidates, MoveCandidates, SetOfMoveCandidates};
use crate::solver::steps::{hidden_singles, lone_singles};
use crate::GameState;

pub fn solve(game: &GameState) -> GameState {
    // Strategies to apply in the given order.
    // TODO: Apply naked twins strategy
    let strategies: Vec<StrategyFn> = vec![lone_singles, hidden_singles];

    let valid_symbols = collect_valid_symbols(game);
    let initial_candidates = find_move_candidates(&game, &valid_symbols);

    let mut stack = Vec::new();
    stack.push((game.clone(), initial_candidates));

    let mut tried_moves = HashSet::new();

    'stack: while let Some((mut state, mut candidates)) = stack.pop() {
        debug!("Stack depth: {}", stack.len());

        if candidates.is_empty() {
            debug!("  - ! No more candidates; returning");
            return state;
        }

        if !is_solvable(&state, &candidates) {
            debug!("  - ! State is unsolvable; skipping");
            continue 'stack;
        }

        let mut applied_some = true;
        while applied_some {
            applied_some = false;

            debug!("  - {} candidates remaining", candidates.total_len());

            for strategy in strategies.iter() {
                applied_some |=
                    match apply_simple_strategy_repeatedly(strategy, &mut state, &mut candidates) {
                        Ok(applied) => applied,
                        Err(_) => {
                            // branch is invalid
                            continue 'stack;
                        }
                    };
            }
        }

        // Sanity check.
        if !is_solvable(&state, &candidates) {
            continue 'stack;
        }

        // If the state didn't change, we need to fork.
        let mut sorted_candidates: Vec<MoveCandidates> = Vec::from_iter(candidates.iter());
        sorted_candidates.sort_unstable_by_key(|v| v.moves.len());

        for candidate_set in sorted_candidates {
            'candidates: for candidate in candidate_set.moves {
                let key = (state.id().clone(), candidate.clone());

                if tried_moves.contains(&key) {
                    debug!("  ! Found move that was already tried; skipping.");
                    continue 'candidates;
                }

                tried_moves.insert(key);

                let branch = state.apply_and_fork(candidate.index, candidate.value);
                let branch_candidates = find_move_candidates(&branch, &valid_symbols);

                // We remove (not eliminate!) the candidate we just tried and requeue the current
                // branch if it still contains options.
                candidates.remove_candidate(&candidate);
                if !candidates.is_empty() {
                    debug!("  + Pushing base branch");
                    stack.push((state, candidates));
                }

                debug!(
                    "  + Branching; {} candidates to explore",
                    branch_candidates.total_len()
                );
                stack.push((branch, branch_candidates));
                continue 'stack;
            }
        }

        // All possible options were exhausted - this branch is a dead end.
        if state.validate(false) {
            debug!("Solved.");
            return state;
        }
    }

    unreachable!()
}

pub type StrategyFn = fn(&mut GameState, &SetOfMoveCandidates) -> Vec<Placement>;

fn apply_simple_strategy_repeatedly(
    strategy: &StrategyFn,
    state: &mut GameState,
    candidates: &mut SetOfMoveCandidates,
) -> Result<bool, bool> {
    let mut applied_some = false;
    loop {
        match apply_simple_strategy_once(strategy, state, candidates) {
            Ok(true) => {
                applied_some = true;
                continue;
            }
            Ok(false) => {
                return Ok(applied_some);
            }
            Err(e) => return Err(e),
        }
    }
}

fn apply_simple_strategy_once(
    strategy: &StrategyFn,
    mut state: &mut GameState,
    mut candidates: &mut SetOfMoveCandidates,
) -> Result<bool, bool> {
    let applied = strategy(&mut state, &candidates);
    if applied.is_empty() {
        return Ok(false);
    }

    eliminate_many(&state, &mut candidates, applied.into_iter());
    debug!(
        "  - Candidates left after applying strategy: {}.",
        candidates.total_len()
    );

    // If an invalid move was made here or the board isn't solvable, leave this branch.
    if !state.validate(true) {
        debug!("  ! Branch is invalid.");
        return Err(false);
    }

    Ok(true)
}

fn eliminate(state: &GameState, candidates: &mut SetOfMoveCandidates, placement: &Placement) {
    let peers = state.peer_indexes_by_index(placement.index, false, CollectType::All);
    for peer_index in peers {
        candidates.remove_candidate(&Placement::new(placement.value, peer_index));
    }
}

fn eliminate_many(
    state: &GameState,
    candidates: &mut SetOfMoveCandidates,
    applied: impl Iterator<Item = Placement>,
) {
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
