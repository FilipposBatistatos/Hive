use std::collections::HashMap;
use serde::{ Serialize, Deserialize };

use crate::types::*;

/* Board is the state of the game and the source of truth for most things */
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Board {
    pub stacks: HashMap<Position, Vec<Piece>>, // Piece can stack, therefore we need a structure to handle multiple pieces in the same location
}

impl Board {
    pub fn new() -> Self {
        Board { stacks: HashMap::new() }
    }

    pub fn place_piece(&self, position: Position, piece: Piece) -> Board {
        let mut new_stacks = self.stacks.clone();
        new_stacks.entry(position).or_insert_with(Vec::new).push(piece);
        Board { stacks: new_stacks }
    }
    
    pub fn remove_piece(&self, position: Position) -> Board {
        let mut new_stacks = self.stacks.clone();
        if let Some(stack) = new_stacks.get_mut(&position) {
            stack.pop();
            if stack.is_empty() {
                new_stacks.remove(&position);
            }
        }
        
        Board { stacks: new_stacks }
    }
    
    pub fn move_piece(&self, from: Position, to: Position) -> Board {
        let piece = *self.stacks
            .get(&from)
            .and_then(|stack| stack.last())
            .expect("move_piece called on an empty or missing position");

        self.remove_piece(from).place_piece(to, piece)
    }
}

#[cfg(test)]
impl Board {
    pub fn snapshot(&self) -> String {
        if self.stacks.is_empty() {
            return String::new();
        }    

        let positions: Vec<&Position> = self.stacks.keys().collect();
        let min_q = positions.iter().map(|p| p.q).min().unwrap();
        let max_q = positions.iter().map(|p| p.q).max().unwrap();
        let min_r = positions.iter().map(|p| p.r).min().unwrap();
        let max_r = positions.iter().map(|p| p.r).max().unwrap();

        (min_r..=max_r)
            .map(|r| {
                let indent = " ".repeat((r - min_r) as usize);
                let row: String = (min_q..=max_q)
                    .map(|q| match self.stacks.get(&Position { q, r }){
                        Some(stack) => piece_symbol(stack.last().unwrap()),
                        None => '.',
                    })
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                format!("{indent}{row}")
            })
            .collect::<Vec<_>>()
            .join("\n")
            }
}

#[cfg(test)]
fn piece_symbol(piece: &Piece) -> char {
    let letter = match piece.kind {
        PieceKind::Bee => 'q',
        PieceKind::Ant => 'a',
        PieceKind::Grasshopper => 'g',
        PieceKind::Beetle => 'b',
        PieceKind::Spider => 's',
    };

    match piece.owner {
        Player::White => letter.to_ascii_uppercase(),
        Player::Black => letter,
    }
}

#[cfg(test)]
mod expect_tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn places_three_pieces_staggered() {
        let board = Board::new()
            .place_piece(Position { q: 2, r: 0}, Piece {kind: PieceKind::Spider, owner: Player::White})
            .place_piece(Position { q: 0, r: 2}, Piece {kind: PieceKind::Ant, owner: Player::Black})
            .place_piece(Position { q: -1, r: 0}, Piece {kind: PieceKind::Bee, owner: Player::White});
        
        let to_string = board.snapshot();

        expect![[r#"
            Q . . S
             . . . .
              . a . ."#]].assert_eq(&to_string);
    }

    #[test]
    fn correctly_removes_piece() {
        let board = Board::new()
            .place_piece(Position { q: 0, r: 0}, Piece {kind: PieceKind::Spider, owner: Player::White})
            .place_piece(Position { q: 1, r: 0}, Piece {kind: PieceKind::Ant, owner: Player::Black})
            .remove_piece(Position {q: 1, r: 0});
        
        let to_string = board.snapshot();

        expect!["S"].assert_eq(&to_string);
    }

    #[test]
    fn correctly_moves_piece() {
        let board = Board::new()
            .place_piece(Position { q: 0, r: 0}, Piece {kind: PieceKind::Spider, owner: Player::White})
            .place_piece(Position { q: 1, r: 0}, Piece {kind: PieceKind::Ant, owner: Player::Black})
            .move_piece(Position {q: 1, r: 0}, Position {q: -1, r: 0});
        
        let to_string = board.snapshot();

        expect!["a S"].assert_eq(&to_string);
    }
}