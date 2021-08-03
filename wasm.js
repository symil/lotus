import fs from 'fs';
import { WASI } from 'wasi';

async function main() {
    let wasi = new WASI();
    let importObject = { wasi_snapshot_preview1: wasi.wasiImport };

    let wasm = await WebAssembly.compile(fs.readFileSync('./build/module.wasm'));
    let instance = await WebAssembly.instantiate(wasm, importObject);

    wasi.start(instance);
}

main();