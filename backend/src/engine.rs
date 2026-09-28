use std::collections::HashSet;

use crate::board::Board;
use crate::types::*;
use crate::game::{ apply_move };
use crate::rules::{ legal_moves, legal_placements, opponent };

pub fn all_legal_placements(state: &GameState) -> Vec<Move> {
    let positions = legal_placements(&state.board, state.turn);
    let pieces = &state.unplaced[&state.turn];

    positions
        .iter()
        .flat_map(|&pos| {
            pieces.keys().map(move |&kind| Move::Place { kind, at: pos })
        })
        .collect()
}

pub fn all_legal_movement(state: &GameState) -> Vec<Move> {
    // Go over all our pieces on the board
    state.board.stacks
        .iter()
        .filter(|(_, stack)| stack.last().map(|piece| piece.owner) == Some(state.turn))
        .flat_map(|(pos, _)| legal_moves(pos, state))
        .collect()
}

pub fn all_legal_moves(state: &GameState) -> Vec<Move> {
    // TODO: If this list is every empty it must return a pass move
    all_legal_movement(state)
        .into_iter()
        .chain(all_legal_placements(state))
        .collect()
}

// State evaluation heuristics
fn mobility(state: &GameState, player: Player) -> i32 {
    let p = GameState { turn: player, ..state.clone() };
    all_legal_moves(&p).len() as i32
}

pub fn evaluate(state: &GameState, player: Player) -> i32 {
    mobility(state, player) - mobility(state, opponent(player))
}

const WIN_SCORE: i32 = 1_000_000;

pub fn minimax(state: &GameState, depth: u32, maximising: Player) -> i32 {
    match state.result {
        Some(GameResult::Win(p)) => {
            let score = WIN_SCORE + depth as i32;
            if p == maximising { score } else { -score }
        }
        Some(GameResult::Draw) => 0,
        None if depth == 0 => evaluate(state, maximising),
        None => {
            let scores = all_legal_moves(state)
                .into_iter()
                .map(|mv| minimax(&apply_move(state, mv), depth - 1, maximising));

            let best = if state.turn == maximising { scores.max() } else {scores.min() };
            best.expect("never and empty list")
        }
    }
}

pub fn best_move(state: &GameState, depth: u32) -> Move {
    all_legal_moves(state)
        .into_iter()
        .map(|mv| {
            let score = minimax(&apply_move(state, mv), depth - 1, state.turn);
            (mv, score)
        })
        .max_by_key(|&(_, score)| score)
        .map(|(mv, _)| mv)
        .expect("all_legal_moves never returns an mepty list")
}

#[cfg(test)]
mod engine_tests;