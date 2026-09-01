mod types;
mod board;
mod rules;
mod game;

use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use serde::Serialize;

use crate::types::*;
use crate::game::*;
use crate::board::*;
use crate::rules::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize)]
struct BoardWire {
    stacks: Vec<(Position, Vec<Piece>)>,
}

#[derive(Serialize)]
struct GameStateWire {
    board: BoardWire,
    turn: Player,
    turn_number: u32,
    unplaced: Vec<(Player, Vec<(PieceKind, u8)>)>,
    result: Option<GameResult>,
}

impl From<&GameState> for GameStateWire {
    fn from(state: &GameState) -> Self {
        GameStateWire {
            board: BoardWire {
                stacks: state.board.stacks.iter().map(|(&p, v)| (p, v.clone())).collect(),
            },
            turn: state.turn,
            turn_number: state.turn_number,
            unplaced: state.unplaced.iter()
                .map(|(&player, kinds)| (player, kinds.iter().map(|(&k, &v)| (k, v)).collect()))
                .collect(),
            result: state.result,
        }
    }
}

fn to_js(value: &impl Serialize) -> JsValue {
    serde_wasm_bindgen::to_value(value).unwrap()
}

#[wasm_bindgen]
pub fn new_game() -> JsValue {
    let state = initial_game_state();
    to_js(&GameStateWire::from(&state))
}

#[wasm_bindgen]
pub fn legal_moves_for_piece_json(state_js: JsValue, pos_js: JsValue) -> JsValue {
    let state: GameState = serde_wasm_bindgen::from_value(state_js).unwrap();
    let pos: Position = serde_wasm_bindgen::from_value(pos_js).unwrap();
    let moves = legal_moves(&pos, &state);
    to_js(&moves)
}

#[wasm_bindgen]
pub fn get_legal_placements(state_js: JsValue) -> JsValue { 
    let state: GameState = serde_wasm_bindgen::from_value(state_js).unwrap();
    let positions = legal_placements(&state.board, state.turn);
    to_js(&positions)
}

#[wasm_bindgen]
pub fn apply_move_json(state_js: JsValue, mv_js: JsValue) -> JsValue {
    let state: GameState = serde_wasm_bindgen::from_value(state_js).unwrap();
    let mv: Move = serde_wasm_bindgen::from_value(mv_js).unwrap();
    let new_state = apply_move(&state, mv);
    to_js(&GameStateWire::from(&new_state))
}

fn initial_game_state() -> GameState {
    let unplaced = HashMap::from([
        (PieceKind::Bee, 1),
        (PieceKind::Ant, 3),
        (PieceKind::Beetle, 2),
        (PieceKind::Spider, 2),
        (PieceKind::Grasshopper, 3),
    ]);

    GameState {
        board : Board::new(),
        turn: Player::White,
        turn_number: 1,
        unplaced: HashMap::from([
            (Player::White, unplaced.clone()),
            (Player::Black, unplaced.clone()),
        ]),
        result: None,
    }
}