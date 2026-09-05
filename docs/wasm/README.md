# HIVE
Functional Programming in rust

## Development commands
Wasm building
```bash
cargo install wasm-pack // To install wasm-pack

wasm-pack build --target web --out-dir ./app/src/wasm/
```

Elm to Js
```bash
elm make ./src/Main.elm --output=elm.js
```

Python server
```bash
python -m http.server 8000
```

## Structure
The front end is written in Elm while the backend is written in rust. 
The structure is split into a front end under the directory `app` and the backend which is is the directory `backend`. 

The front end uses elm - which should probably be split into multiple files but at the moment is one big one. Within the front end, there is a `wasm` folder where the backend web assembly lives and the front end can query it for commands. 

The back end lives between a number of files under `src`. There are four functions exposed from the backend to the front end. 

`new_game`: No arguments, returns a new initial state.

`legal_moves_for_piece_json`: Takes a copy of the state and the relevant board position, returns a list of legal moves.

`get_legal_movements`: Takes a copy of the state and returns a list of legal placements.

`apply_move_json`: Takes a copy of the state and a move, returns the new state with the move applied. 