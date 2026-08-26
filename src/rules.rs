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

fn can_place(board: &Board, position: Position, player: Player) -> bool {
    if board.stacks.is_empty() {
        return true;
    }

    !is_occupied(position, board)
        && is_on_hive(board, position, None)
        && !adjacent_to_opponent(board, position, player)
}

fn adjacent_to_opponent(board: &Board, position: Position, player: Player) -> bool {
    neighbors(position)
        .into_iter()
        .filter(|n| is_occupied(*n, board))
        .filter_map(|n| board.stacks.get(&n))
        .filter_map(|stack| stack.last())
        .any(|piece| piece.owner != player)
}

fn legal_placements(board: &Board, player: Player) -> HashSet<Position> {
    if board.stacks.is_empty() {
        return HashSet::from([Position { q: 0, r: 0 }]); // First piece ideally placed in the origin
    }

    if board.stacks.len() == 1 {
        // This is the second move and therefor, has to be adjacent to a different collor
        if let Some(&single_pos) = board.stacks.keys().next() {
            return HashSet::from_iter(neighbors(single_pos));
        }
    }


    board.stacks
        .keys()
        .flat_map(|pos| neighbors(*pos))
        .filter(|&candidate| can_place(board, candidate, player))
        .collect()
}

fn is_occupied(pos: Position, board: &Board) -> bool {
    // Returns whether a position on the board contains a piece
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
    // Part of preserves hive executing the DFS to ensure that every piece is reachable
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
    // Uses DFS to ensure that all the pieces are connected with each other 
    // TODO: Evaluate performance bottle neck of this approach    

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
    // Returns the axial coordinates for the positions of all the neighoring cells
    let directions = vec![(0, 1), (1, -1), (1, 0), (0, -1), (-1, 1), (-1, 0)];

    directions
        .iter() // Iterate over the collection
        .map(|(dq, dr)| Position { q: pos.q + dq, r: pos.r + dr }) // Apply this function 
        .collect() // Collect the iterator into a container
    // Because there is no ; this is the return value, so we never mutate directions
}

fn bee_moves(pos: &Position, board: &Board) -> Vec<Move> {
    // Returns the legal moves for the bee piece
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

fn ant_moves(pos: &Position, board: &Board) -> Vec<Move> {
    // Returns the legal ant moves

    if !preserves_hive(board, *pos) {
        return Vec::<Move>::new();
    }

    /* The ant can crawl around with no distance limit */
    // Generate the local bee moves,
    // Ensure they are traversable
    // Repeat until there are no more legal moves

    fn visit(board: &Board, pos: Position, mut visited: HashSet<Position>) -> HashSet<Position> {
        if visited.contains(&pos) {
            return visited;
        }

        visited.insert(pos);
        neighbors(pos)
            .into_iter()
            .filter(|&candidate| !is_occupied(candidate, board))
            .filter(|&candidate| is_on_hive(board, candidate, Some(pos))) 
            .filter(|&candidate| can_slide(board, pos, candidate))
            .fold(visited, |acc, pos| visit(board, pos, acc))
    }

    visit(board, *pos, HashSet::<Position>::new())
        .into_iter()
        .map(|candidate| Move::Move {from: *pos, to: candidate})
        .collect::<Vec<Move>>()
}

#[cfg(test)]
mod expect_tests {
    use super::*;
    use expect_test::expect;

    fn render_moves(moves: &[Move]) -> String {
        // Helper function to visualise pieces on the board
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
    
    #[test]
    fn correct_ant_moves() {
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
        let moves = ant_moves(&Position {q: 0, r: 0}, &board);
        let output = render_moves(&moves);

        expect![[r#"
            . . . A A
             . A A . A
              A . . . A
               A . . A .
                A A A . ."#]].assert_eq(&output);
    }

    #[test]
    fn placing_only_with_no_enemy_neighbors() {
        let occupied_positions = vec![
            Position {q: 1, r: 0},
            Position {q: 1, r: 1},
            Position {q: 0, r: 2},
            Position {q: -1, r: 2},
            Position {q: -1, r: 1},
        ];
        
        let board = occupied_positions.iter().enumerate().fold(Board::new(), |board, (index, &pos)| {
            board.place_piece(pos, Piece {kind: PieceKind::Ant, owner: if index % 2 == 0 { Player::White } else { Player::Black} })
        });
        let placements = legal_placements(&board, Player::White)
            .into_iter()
            .map(|p| Move::Place {kind: PieceKind::Bee, at: p })
            .collect::<Vec<Move>>();    
        let output = render_moves(&placements);

        expect![[r#"
            . . . A A
             . A A . .
              A . . . .
               . . . . .
                . . A . ."#]].assert_eq(&output);
    }

    #[test]
    fn legal_place_when_board_has_only_one_piece() {
        let board = Board::new().place_piece(Position {q: 0, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::White});
        let placements = legal_placements(&board, Player::Black)
            .into_iter()
            .map(|p| Move::Place {kind: PieceKind::Bee, at: p })
            .collect::<Vec<Move>>();
        let output = render_moves(&placements);

        expect![[r#"
            . A A
             A . A
              A A ."#]].assert_eq(&output);
    }

    #[test]
    fn legal_place_when_board_is_empty() {
        let board = Board::new();
        let placements = legal_placements(&board, Player::White)
            .into_iter()
            .map(|p| Move::Place {kind: PieceKind::Bee, at: p})
            .collect::<Vec<Move>>();
        let output = render_moves(&placements);

        expect!["A"].assert_eq(&output);
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

#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::TestRunner;
    use proptest::strategy::ValueTree;
    
    fn arbitrary_board(steps: usize) -> impl Strategy<Value = Board> {
        prop::collection::vec(any::<usize>(), steps).prop_map(move |choices| {
            choices.into_iter().enumerate().fold(Board::new(), |board, (step, choice)| {
                let player = if step % 2 == 0 { Player::White } else { Player::Black };
                let candidates: Vec<Position> = legal_placements(&board, player).into_iter().collect();
                
                if candidates.is_empty() {
                    return board; // Nowhere is legal to place, skip this step rather than panic
                }

                let pos = candidates[choice % candidates.len()];
                board.place_piece(pos, Piece { kind: PieceKind::Ant, owner: player })
            })
        })
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
} 