use std::collections::HashSet;

use crate::board::Board;
use crate::types::*;
use crate::rules::{ legal_moves, legal_placements };

pub fn all_legal_placements(state: &GameState) -> Vec<Move> {
    let positions = legal_placements(state.board, state.player);
    let pieces = state.unplaced[&state.player];

    positions
        .iter()
        .flat_map(|&pos| {
            pieces.keys().map(move |&kind| Move::Place { kind, at: pos })
        })
        .collect()
}

proptest! {
    #[test]
    fn legal_placement_moves_are_positions_times_hand(
        board in arbitrary_board(8),
        player in arbitrary_player(),
        hand in arbitrary_hand(),
    ) {
        let state = GameState {
            board: board.clone(),
            turn: player,
            turn_number: 1,
            unplaced: HashMap::from([
                (player, hand.clone()),
                (opponent(player), HashMap::new()),
            ]),
            result: None,
        };

        let moves = all_legal_placements(&state);
        let positions = legal_placements(&board, player);

        prop_assert_eq!(moves.len(), positions.len() * hand.len());

        let mut seen = HashSet::new();
        for mv in &moves {
            match mv {
                Move::Place {kind, at} => {
                    prop_assert!(positions.contains(at));
                    prop_assert!(hand.contains_key(kind));
                    prop_assert!(seen.insert((*kind, *at)), "Duplicate position produced");
                }
                Move::Move { .. } => prop_assert!(false, "Move instead of placement")
            }
        }
    }
}