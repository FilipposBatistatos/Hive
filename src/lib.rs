mod types;
mod board;
mod rules;
mod game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn new_game() -> JsValue {
    let state = initial_game_state();
    serde_wasm_bindgen::to_value(&state).unwrap();
}

#[wasm_bindgen]
pub fn legal_moves_for_piece_json(state_js: JsValue, pos_js) {
    let state: GameState = serde_wasm_bindgen::from_value(state_js).unwrap();
    let pos: Position = serde_wasm_bindgen::from_value(pos_js).unwrap();
    let moves = legal_moves(&pos, &state);
    serde_wasm_bindgen::to_value(&moves).unwrap()
}

#[wasm_bindgen]
pub fn get_legal_placements(state_js: JsValue) { 
    let state: GameState = serde_wasm_bindgen::from_value(state_js).unwrap();
    let positions = legal_placements(state.board, state.turn);
    serde_wasm_bindgen::to_value(&moves).unwrap()
}

#[wasm_bindgen]
pub fn apply_move_json(state_js: JsValue, mv_js: JsValue) -> JsValue {
    let state: GameState = serde_wasm_bindgen::from_value(state_js).unwrap();
    let mv: Move = serde_wasm_bindgen::from_value(mv_js).unwrap();
    let new_state = apply_move(&state, mv);
    serde_wasm_bindgen::to_value(&new_state).unwrap()
}