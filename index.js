import init from "./pkg/meteo.js";

const runWasm = async () => {
    const wasm = await init("./pkg/meteo_bg.wasm");

    const canvas = document.getElementById("canvas");

    const ctx = canvas.getContext("2d");

   
};

runWasm();