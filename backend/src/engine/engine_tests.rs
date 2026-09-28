use super::*;

use crate::test_support::*;
use crate::types::*;

use proptest::prelude::*;
use std::collections::{HashMap, HashSet};

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
                    prop_assert!(positions.contains(&at));
                    prop_assert!(hand.contains_key(&kind));
                    prop_assert!(seen.insert((kind, at)), "Duplicate position produced");
                }
                Move::Move { .. } => prop_assert!(false, "Move instead of placement"),
                Move::Pass => prop_assert!(false, "Pass instead of placement"),
            }
        }
    }
}

proptest! {
    #[test]
    fn evaluate_is_antisymmetric(
        board in arbitrary_board(8),
        player in arbitrary_player(),
        hand_a in arbitrary_hand(),
        hand_b in arbitrary_hand(),
    ) {
        let state = GameState {
            board,
            turn: player,
            turn_number: 1, 
            unplaced: HashMap::from([
                (player, hand_a),
                (opponent(player), hand_b),
            ]),
            result: None,
        };

        prop_assert_eq!(
            evaluate(&state, player),
            -evaluate(&state, opponent(player))
        );
    }
}