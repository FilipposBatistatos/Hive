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

fn main() {
    println!("Hello world");
}
