use crate::board::Board;
use crate::types::*;


/*
fn legal_moves(pos: &Position, game: &GameState) -> Vec<Move> {
    match kind {
        PieceKind::Bee => bee_moves(pos, game.board),
        _ => // Empty vector
    }

} */

fn is_occupied(pos: Position, board: &Board) -> bool {
    board.stacks.get(&pos).is_some()
}

fn neighbors(pos: Position) -> Vec<Position> {
    let directions = vec![(0, 1), (1, -1), (1, 0), (0, -1), (-1, 1), (-1, 0)];

    directions
        .iter() // Iterate over the collection
        .map(|(dq, dr)| Position { q: pos.q + dq, r: pos.r + dr }) // Apply this function 
        .collect() // Collect the iterator into a container
    // Because there is no ; this is the return value, so we never mutate directions
}

fn bee_moves(pos: &Position, board: &Board) -> Vec<Move> {
    /*  Iterate over all the neighbors of the piece
        valid moves are the moves where there are no neighbors */
    neighbors(*pos)
        .into_iter()
        .filter(|&candidate| !is_occupied(candidate, board))
        .map(|candidate| Move::Move {from: *pos, to: candidate })
        .collect()
}

#[cfg(test)]
mod expect_tests {
    use super::*;
    use expect_test::expect;

    fn render_moves(moves: &[Move]) -> String {
        let visualised = moves.iter().fold(Board::new(), |b, mov| match mov {
            Move::Move { to, .. } => b.place_piece(*to, Piece { kind: PieceKind::Ant, owner: Player::White }),
            Move::Place { at, .. } => b.place_piece(*at, Piece { kind: PieceKind::Ant, owner: Player::White }),
        });
        visualised.snapshot()
    }

    #[test]
    fn correct_neighbor_positions() {
        let board = neighbors(Position { q: 0, r: 0 })
            .into_iter()
            .fold(Board::new(), |board, pos| {
                board.place_piece(pos, Piece { kind: PieceKind::Ant, owner: Player::White })
            });

        let output = board.snapshot();

        expect![[r#"
            . A A
             A . A
              A A ."#]].assert_eq(&output);
    }

    #[test]
    fn correct_bee_moves() {
        let occupied_positions = vec![
            Position {q: 1, r: 0},
            Position {q: 0, r: -1},
        ];
        
        let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
            board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
        });
        let moves = bee_moves(&Position {q: 0, r: 0}, &board);
        let output = render_moves(&moves);

        expect![[r#"
            . . A
             A . .
              A A ."#]].assert_eq(&output);
    }
}