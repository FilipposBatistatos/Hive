import init, { init_panic_hook, apply_move_json, get_legal_placements, legal_moves_for_piece_json, new_game } from "./src/wasm/Hive.js";

async function main() {
    await init();
    init_panic_hook();
    const app = Elm.Main.init({ node: document.getElementById("app") });

    app.ports.requestNewGame.subscribe(() => {
        const result = new_game();
        //console.log("raw wasm json:", JSON.stringify(result));
        app.ports.receiveInitialState.send(result);
    });

    app.ports.requestLegalPlacements.subscribe((state) => {
        app.ports.receiveLegalPlacements.send(get_legal_placements(state));
    });

    app.ports.requestApplyMove.subscribe(([state, mv]) => {
        app.ports.receiveNewState.send(apply_move_json(state, mv));
    });

    app.ports.requestMovesForPiece.subscribe(([state, pos]) => {
        console.log("requestMovesForPiece fired, pos:", pos);
        const result = legal_moves_for_piece_json(state, pos);
        console.log("legal_moves_for_piece_json result:", result);
        app.ports.receiveMovesForPiece.send(result);
    });
}

main();