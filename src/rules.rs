use std::collections::HashSet;

use crate::board::Board;
use crate::types::*;


/*
fn legal_moves(pos: &Position, game: &GameState) -> Vec<Move> {
    match kind {
        PieceKind::Bee => bee_moves(pos, game.board),
        _ => Vec<Move> {},
    }

} */

fn is_occupied(pos: Position, board: &Board) -> bool {
    board.stacks.get(&pos).is_some()
}

fn can_slide(board: &Board, from: Position, to: Position) -> bool {
    // Apply the freedom to move rule - can a piece physically squeeze
    // between two adjacent positions

    let from_neighbors: HashSet<Position> = neighbors(from).into_iter().collect();
    let to_neighbors: HashSet<Position> = neighbors(to).into_iter().collect();

    let flanking: Vec<Position> = from_neighbors 
        .intersection(&to_neighbors)
        .copied()
        .collect();

    flanking.iter().any(|&p| !is_occupied(p, board))
}

fn connected_positions(occupied: &HashSet<Position>, start: Position) -> HashSet<Position> {
    fn visit(
        occupied: &HashSet<Position>,
        current: Position,
        mut visited: HashSet<Position>,
    ) -> HashSet<Position> {
        if visited.contains(&current) {
            return visited;
        }

        visited.insert(current);

        neighbors(current)
            .into_iter()
            .filter(|n| occupied.contains(n))
            .fold(visited, |acc, n| visit(occupied, n, acc))
    }

    visit(occupied, start, HashSet::new())
}

fn preserves_hive(board: &Board, from: Position) -> bool {
    // Does the hive maintain its integrity if this piece is removed from this position
    
    // Instead what if take the neighbors of the piece and make sure we can reach all the other ones 
    // Can we do that by maintaining a union find but not prune it... 

    let remaining_pieces: HashSet<Position> = board.stacks
        .keys()
        .copied()
        .filter(|&pos| pos != from)
        .collect();
    
    match remaining_pieces.iter().next() {
        None => true,
        Some(&start) => connected_positions(&remaining_pieces, start).len() == remaining_pieces.len(),
    }
}

fn is_on_hive(board: &Board, to: Position, from: Option<Position>) -> bool {
    // Ensures that possible moves are still on the hive,
    // and therefore wont break the one hive rule 

    neighbors(to)
        .into_iter()
        .filter(|&n| Some(n) != from) // Ensure we are not counting the position we are currently on
        .any(|n| is_occupied(n, board))
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
    if !preserves_hive(board, *pos) {
        return Vec::<Move>::new();
    }

    neighbors(*pos)
        .into_iter()
        .filter(|&candidate| !is_occupied(candidate, board))
        .filter(|&candidate| is_on_hive(board, candidate, Some(*pos)))
        .filter(|&candidate| can_slide(board, *pos, candidate))
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
            Position {q: 1, r: 1},
            Position {q: 0, r: 2},
            Position {q: -1, r: 2},
            Position {q: -1, r: 1},
        ];
        
        let board = occupied_positions.iter().fold(Board::new(), |board, &pos| {
            board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: Player::White })
        });
        let moves = bee_moves(&Position {q: 0, r: 0}, &board);
        let output = render_moves(&moves);

        expect![[r#"
            . . A
             A . ."#]].assert_eq(&output);
    }

    // Everything here really should be a prop test
    #[test]
    fn removing_a_bridge_piece_breaks_the_hive() {
        let board = Board::new()
            .place_piece(Position { q: 0, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White })
            .place_piece(Position { q: 1, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White })
            .place_piece(Position { q: 2, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White });
        
        assert!(!preserves_hive(&board, Position {q: 1, r: 0}));
        assert!(preserves_hive(&board, Position {q: 2, r: 0}));
    }

    #[test]
    fn cannot_slide_through_pinched_gap() {
        let board = Board::new()
            .place_piece(Position { q: 1, r: 0}, Piece { kind: PieceKind::Ant, owner: Player::White })
            .place_piece(Position { q: -1, r: 1}, Piece { kind: PieceKind::Ant, owner: Player::White });

        assert!(!can_slide(&board, Position { q: 0, r: 0 }, Position { q: 0, r: 1 }));
    }
}