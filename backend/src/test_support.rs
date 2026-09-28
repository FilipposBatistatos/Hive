use crate::board::Board;
use crate::rules::{ legal_moves, legal_placements };
use crate::types::*;

use std::collections::HashMap;
use proptest::prelude::*;
use proptest::test_runner::TestRunner;
use proptest::strategy::ValueTree;

const PIECE_KINDS: [PieceKind; 5] = [
    PieceKind::Bee,
    PieceKind::Spider,
    PieceKind::Beetle,
    PieceKind::Grasshopper,
    PieceKind::Ant,
];

pub fn arbitrary_board(steps: usize) -> impl Strategy<Value = Board> {
    prop::collection::vec((any::<usize>(), any::<usize>()), steps).prop_map(move |choices| {
        choices.into_iter().enumerate().fold(Board::new(), |board, (step, (pos_choice, kind_choice))| {
            let player = if step % 2 == 0 { Player::White } else { Player::Black };
            let candidates: Vec<Position> = legal_placements(&board, player).into_iter().collect();
            
            if candidates.is_empty() {
                return board; // Nowhere is legal to place, skip this step rather than panic
            }

            let pos = candidates[pos_choice % candidates.len()];
            let kind = PIECE_KINDS[kind_choice % PIECE_KINDS.len()];
            board.place_piece(pos, Piece { kind: kind, owner: player })
        })
    })
}

pub fn arbitrary_hand() -> impl Strategy<Value = HashMap<PieceKind, u8>> {
    prop::collection::hash_map(
        prop_oneof![
            Just(PieceKind::Bee),
            Just(PieceKind::Ant),
            Just(PieceKind::Beetle),
            Just(PieceKind::Spider),
            Just(PieceKind::Grasshopper),
        ], 
        1u8..=3,
        0..=5,
    )
}

pub fn arbitrary_player() -> impl Strategy<Value = Player> {
    prop_oneof![Just(Player::White), Just(Player::Black)]
}

#[test]
fn generated_boards_have_balanced_piece_counts() {
    let mut runner = TestRunner::default();
    let steps = 10;

    let tree = arbitrary_board(steps).new_tree(&mut runner).unwrap();
    let board = tree.current();

    let white_count = board.stacks
        .values()
        .flatten()
        .filter(|piece| piece.owner == Player::White)
        .count();

    let black_count = board.stacks
        .values()
        .flatten()
        .filter(|piece| piece.owner == Player::Black)
        .count();
    
    assert_eq!(white_count + black_count, steps, "Every step should place exactly one piece");
    assert_eq!(white_count, black_count, "Even step count should split evenly between players");
}