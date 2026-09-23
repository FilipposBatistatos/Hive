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
