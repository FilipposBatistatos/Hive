use std::collections::{ HashMap, HashSet };

use crate::board::Board;
use crate::types::*;
use crate::rules::*;

pub fn apply_move(state: &GameState, mv: Move) -> GameState {
    let next = GameState {
        turn: opponent(state.turn),
        turn_number: state.turn_number + 1,
        ..state.clone()
    };

    match mv {
        Move::Pass => next,
        Move::Place { kind, at } => {
            let board = state.board.place_piece(at, Piece { kind, owner: state.turn });
            GameState {
                result: is_game_over(&board, at),
                unplaced: take_from_hand(&state.unplaced, state.turn, kind),
                board,
                ..next
            }
        }
        Move::Move { from, to} => {
            let board = state.board.move_piece(from, to);
            GameState {
                result: is_game_over(&board, to),
                board,
                ..next
            }
        }
    }
}

fn is_surrounded(board: &Board, pos: Position) -> bool {
    neighbors(pos)
        .into_iter()
        .filter(|&p| is_occupied(p, board))
        .count() == 6
}

pub fn is_game_over(board: &Board, pos: Position) -> Option<GameResult> {
    // Collect the neighbors of the piece that was just move/placed
    let candidate_positions: Vec<Position> = neighbors(pos)
        .into_iter()
        .chain(std::iter::once(pos))
        .collect();
        
    // Collect the players who's bees are surrounded
    let surrounded_bee_owners: Vec<Player> = candidate_positions
        .into_iter()
        .filter(|&pos| is_surrounded(board,pos)) // Find pos that are surrounded
        .filter_map(|pos| board.stacks.get(&pos)) // Get the stacks from those positions
        .flat_map(|stack| stack.iter()) // Flatten the stack so we can see all the pieces present
        .filter(|piece| piece.kind == PieceKind::Bee) // Check for bees
        .map(|piece| piece.owner) // Get the bee owner
        .collect();
        

    match (surrounded_bee_owners.contains(&Player::White), surrounded_bee_owners.contains(&Player::Black)) {
        (true, true) => Some(GameResult::Draw),
        (true, false) => Some(GameResult::Win(Player::Black)),
        (false, true) => Some(GameResult::Win(Player::White)),
        (false, false) => None,
    }
}

fn take_from_hand(
    unplaced: &HashMap<Player, HashMap<PieceKind, u8>>,
    player: Player,
    kind: PieceKind,
) -> HashMap<Player, HashMap<PieceKind, u8>> {
    let mut updated = unplaced.clone();
    if let Some(hand) = updated.get_mut(&player) {
        hand.retain(|&k, count| {
            if k == kind { *count -= 1; *count > 0} else { true }
        });
    }
    updated
}