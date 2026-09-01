# HIVE
Moving the project to rust for better ability to test

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