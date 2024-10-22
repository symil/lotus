import fs from 'fs';
import { initializeWasm } from './wasm-initialization.js';

export async function runWasmCommandLine(wasmPath) {
    let wasmContent = fs.readFileSync(wasmPath, null);
    let wasmEnv = makeWasmEnv();
    let instance = await initializeWasm(wasmContent, wasmEnv);
    /** @type {any} */
    let exports = instance.exports;

    exports.main();
}

function makeWasmEnv() {
    return {
        log(string) {
            return console.log(string);
        },
    }
}