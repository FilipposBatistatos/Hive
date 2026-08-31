use std::collections::HashMap;

use crate::board::Board;

/* Coordinate system is Axial */
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position { pub q: i32, pub r: i32 }

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind { Bee, Spider, Beetle, Grasshopper, Ant }

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Player { White, Black }
 
#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub owner: Player,
}


/* Game state is the structure from which the game engine will be able to determine legal moves
and the solver will be able to effectively solve the game */
#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub board: Board,
    pub turn: Player,
    pub turn_number: u32,
    pub unplaced: HashMap<Player, HashMap<PieceKind, u8>>,
    pub result: Option<GameResult>
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GameResult {
    Win(Player),
    Draw,
}

pub enum Move {
    Place { kind: PieceKind, at: Position },
    Move { from: Position, to: Position},
}
