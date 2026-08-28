use std::collections::HashSet;

use crate::board::Board;
use crate::types::*;

fn legal_moves(pos: &Position, game: &GameState) -> Vec<Move> {
    // Generates legal moves based on the selected piece
    
    // If the bee is not placed you cannot move anything
    if !game.unplaced[&game.turn]
        .iter()
        .any(|n| *n == PieceKind::Bee) {
            return vec![];                
    }

    // Ensure the stack is not empty
    if let Some(piece) = game.board.stacks
        .get(pos)
        .and_then(|stack| stack.last()) {
            
        if piece.owner != game.turn {
            return vec![];
        }

        return match piece.kind {
            PieceKind::Bee => bee_moves(pos, &game.board),
            PieceKind::Ant => ant_moves(pos, &game.board),
            PieceKind::Spider => spider_moves(pos, &game.board),
        };
    }
    vec![]
}

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

fn flanking_positions(a: Position, b: Position) -> HashSet<Position> {
    let a_neighbors: HashSet<Position> = neighbors(a).into_iter().collect();
    let b_neighbors: HashSet<Position> = neighbors(b).into_iter().collect();

    a_neighbors.intersection(&b_neighbors).copied().collect()
}

fn can_slide(board: &Board, from: Position, to: Position, piece_height: usize) -> bool {
    // Apply the freedom to move rule: a gap is passable if at least one of the 
    // flanking positions are shorter or empty than the height of the piece passing through
    
    flanking_positions(from, to)
        .iter()
        .any(|&p| stack_height(board, &p) < piece_height)
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
    match from {
        None => {
            neighbors(to)
                .into_iter()
                .any(|n| is_occupied(n, board))
        } 
        Some(from) => {
            let from_neighbors: HashSet<Position> = neighbors(from).into_iter().collect();
            let to_neighbors: HashSet<Position> = neighbors(to).into_iter().collect();
            
            from_neighbors.contains(&to)
                && from_neighbors
                    .intersection(&to_neighbors)
                    .any(|&n| is_occupied(n, board))
        }
    }
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

fn stack_height(board: &Board, pos: &Position) -> usize {
    board.stacks.get(&pos).map_or(0, |stack| stack.len())
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
        .filter(|&candidate| can_slide(board, *pos, candidate, 1))
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
            .filter(|&candidate| can_slide(board, pos, candidate, 1))
            .fold(visited, |acc, pos| visit(board, pos, acc))
    }

    visit(board, *pos, HashSet::<Position>::new())
        .into_iter()
        .map(|candidate| Move::Move {from: *pos, to: candidate})
        .collect::<Vec<Move>>()
}

fn spider_moves(pos: &Position, board: &Board) -> Vec<Move> {
    // Returns a vector of the legals moves for spider
    if !preserves_hive(board, *pos) {
        return Vec::<Move>::new();
    }
    
    // The spider moves exactly 3 spots away from where it is in the same nature are the ant
    fn visit(board: &Board, pos: Position, visited: HashSet<Position>, steps_remaining: u8) -> HashSet<Position> {
        if steps_remaining == 0 {
            return HashSet::from([pos]); // Only the final landing spot 
        }
        
        neighbors(pos)
            .into_iter()
            .filter(|&n| !visited.contains(&n))
            .filter(|&n| !is_occupied(n, board))
            .filter(|&n| is_on_hive(board, n, Some(pos)))
            .filter(|&n| can_slide(board, pos, n, 1))
            .fold(HashSet::new(), |acc, n| {
                let mut next_visited = visited.clone();
                next_visited.insert(n);
                acc.union(&visit(board, n, next_visited, steps_remaining - 1)).copied().collect()
            })
    }
    
    visit(board, *pos, HashSet::from([*pos]), 3)
        .into_iter()
        .map(|candidate| Move::Move{ from: *pos, to: candidate })
        .collect()
}

fn beetle_moves(pos: &Position, board: &Board) -> Vec<Move> {
    // Hive preservation only applies on ground level
    if stack_height(board, pos) == 1 && !preserves_hive(board, *pos) {
        return vec![];
    }

    neighbors(*pos)
        .into_iter()
        .filter(|p| is_on_hive(board, *p, Some(*pos)) || is_occupied(*p, board))
        .filter(|p| can_slide(board, *p, *pos, stack_height(board, pos)) || stack_height(board, p) >= stack_height(board, pos)) 
        .map(|p| Move::Move { from: *pos, to: p})
        .collect()
}

#[cfg(test)]
mod rules_test;