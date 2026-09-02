use crate::board::Board;
use crate::types::*;
use crate::rules::*;

pub fn apply_move(state: &GameState, mv: Move) -> GameState {
    let new_board = match mv {
        Move::Place {kind, at} => state.board.place_piece(at, Piece { kind, owner: state.turn }),
        Move::Move {from, to} => state.board.move_piece(from, to),
    };

    let affected_position = match mv {
        Move::Place {at, ..} => at,
        Move::Move {to, ..} => to,
    };

    let new_unplaced = match mv {
        Move::Place { kind, .. } => {
            let mut unplaced = state.unplaced.clone();
            if let Some(hand) = unplaced.get_mut(&state.turn) {
                hand.retain(|&k, count| {
                    if k == kind {
                        *count -= 1;
                        *count > 0
                    } else {
                        true
                    }
                });
            }
            unplaced
        }
        Move::Move { .. } => state.unplaced.clone(),
    };
    
    GameState {
        board: new_board.clone(),
        turn: if state.turn == Player::White { Player::Black } else { Player::White },
        turn_number: state.turn_number + 1,
        unplaced: new_unplaced,
        result: is_game_over(&new_board, affected_position),
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
    let surrounded_bee_owners: Vec<Player> = neighbors(pos)
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