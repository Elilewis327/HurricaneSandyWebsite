import init from "./pkg/meteo.js";

const runWasm = async () => {
    const wasm = await init("./pkg/meteo_bg.wasm");
    wasm.main()
};

runWasm();