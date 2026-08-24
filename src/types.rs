use std::collections::HashMap;

use crate::board::Board;

/* Coordinate system is Axial */
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position { pub q: i32, pub r: i32 }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind { Bee, Spider, Beetle, Grasshopper, Ant }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Player { White, Black }
 
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub owner: Player,
}


/* Game state is the structure from which the game engine will be able to determine legal moves
and the solver will be able to effectively solve the game */
pub struct GameState {
    pub board: Board,
    pub turn: Player,
    pub unplaced: HashMap<Player, Vec<PieceKind>>,
}

pub enum Move {
    Place { kind: PieceKind, at: Position },
    Move { from: Position, to: Position},
}