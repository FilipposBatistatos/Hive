use std::collections::HashSet;

use crate::board::Board;
use crate::types::*;
use crate::rules::{ legal_moves, legal_placements };

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

pub fn evaluate(state: &GameState, player: player) -> i32 {
    mobility(state, player) - mobility(state, opponent(player))
}

