use super::*;

use std::collections::{HashMap, HashSet};
use expect_test::expect;

use crate::test_support::*;
use crate::types::*;
use crate::board::Board;

#[test]
fn minimax_winning_move() {
    let board = Board::new()
        .place_piece(Position {q: 0, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::White})
        .place_piece(Position {q: 0, r: -1}, Piece {kind: PieceKind::Ant, owner: Player::White})
        .place_piece(Position {q: -1, r: 1}, Piece {kind: PieceKind::Ant, owner: Player::White})
        .place_piece(Position {q: 0, r: 1}, Piece {kind: PieceKind::Ant, owner: Player::White})
        .place_piece(Position {q: -1, r: -1}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: -1, r: 0}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: 1, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::Black});

    let state = GameState {
        board: board,
        turn: Player::Black,
        turn_number: 8,
        unplaced: HashMap::from([
            (Player::White, HashMap::from([
                (PieceKind::Ant, 1),
                (PieceKind::Spider, 1),
            ])),
            (Player::Black, HashMap::from([
                (PieceKind::Grasshopper, 2),
            ])),
        ]),
        result: None,
    };

    expect![[r#"
        Move {
            from: Position {
                q: -1,
                r: -1,
            },
            to: Position {
                q: 1,
                r: -1,
            },
        }
    "#]].assert_debug_eq(&best_move(&state, 1));
}

// TODO: Ensure move order is deterministic, otherwise this test might fail
#[test]
fn minimax_blocks_winning_move() {
    let board = Board::new()
        .place_piece(Position {q: 0, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::White})
        .place_piece(Position {q: 0, r: -1}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: -1, r: 1}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: 0, r: 1}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: -1, r: -1}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: -1, r: 0}, Piece {kind: PieceKind::Ant, owner: Player::Black})
        .place_piece(Position {q: -1, r: 2}, Piece {kind: PieceKind::Ant, owner: Player::White})
        .place_piece(Position {q: 1, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::Black});

    let state = GameState {
        board: board,
        turn: Player::White,
        turn_number: 8,
        unplaced: HashMap::from([
            (Player::White, HashMap::from([
                (PieceKind::Ant, 1),
                (PieceKind::Spider, 1),
            ])),
            (Player::Black, HashMap::from([
                (PieceKind::Grasshopper, 2),
            ])),
        ]),
        result: None,
    };

    expect![[r#"
        Move {
            from: Position {
                q: -1,
                r: 2,
            },
            to: Position {
                q: -1,
                r: -2,
            },
        }
    "#]].assert_debug_eq(&best_move(&state, 2));
}

use proptest::prelude::*;

proptest! {
    #[test]
    fn all_legal_moves_is_never_empty(
        board in arbitrary_board(8),
        player in arbitrary_player(),
        hand in arbitrary_hand(),
    ) {
        let state = GameState {
            board: board,
            turn: player,
            turn_number: 1,
            unplaced: HashMap::from([
                (player, hand),
                (opponent(player), HashMap::new()),
            ]),
            result: None
        };

        prop_assert!(all_legal_moves(&state).len() > 0);
    }
}

// proptest! {
//     #[test]
//     fn all_legal_moves_only_contains_pass_if_otherwise_empty()
// }

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

proptest! {
    #[test]
    fn minimax_depth_one_picks_best_child(
        board in arbitrary_board(8),
        player in arbitrary_player(),
        perspective in arbitrary_player(),
        hand_a in arbitrary_hand(),
        hand_b in arbitrary_hand(),
    ) {
        let state = GameState {
            board: board,
            turn: player,
            turn_number: 1,
            unplaced: HashMap::from([
                (player, hand_a),
                (opponent(player), hand_b),
            ]),
            result: None,
        };

        let child_scores = all_legal_moves(&state)
            .into_iter()
            .map(|mv| minimax(&apply_move(&state, mv), 0, perspective));

        let expected = if state.turn == perspective {
            child_scores.max()
        } else {
            child_scores.min()
        }
        .expect("All legal moves never returns an empty list");
        
        prop_assert_eq!(minimax(&state, 1, perspective), expected);
    }
}

proptest! {
    #[test]
    fn alpha_beta_agrees_with_minimax(
        board in arbitrary_board(4),
        player in arbitrary_player(),
        perspective in arbitrary_player(),
        hand_a in arbitrary_hand(),
        hand_b in arbitrary_hand(), 
    ) {
        let state = GameState {
            board: board,
            turn: player,
            turn_number: 1,
            unplaced: HashMap::from([
                (player, hand_a),
                (opponent(player), hand_b),
            ]),
            result: None,
        };

        prop_assert_eq!(
            minimax(&state, 2, perspective),
            alpha_beta(&state, 2, i32::MIN, i32::MAX, perspective)
        );
    }
}