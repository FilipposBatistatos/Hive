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