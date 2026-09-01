import init, { apply_move_json, get_legal_placements, new_game } from "./src/wasm/Hive.js";

async function main() {
    await init();
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
}

main();