#!/usr/bin/env node

import fs from 'fs';
import { WASI } from 'wasi';

async function main() {
    let argv = process.argv.slice(2);
    let inputWasmPath = argv[0];

    if (!inputWasmPath) {
        exit(`missing input wasm path`);
    }

    if (!fs.existsSync(inputWasmPath)) {
        exit(`file \`${inputWasmPath}\` does not exist`);
    }

    let wasi = new WASI();
    let importObject = {
        log: {
            i32: value => console.log(value),
            special: () => console.log('SPECIAL')
        }
    };

    let wasm = await WebAssembly.compile(fs.readFileSync(inputWasmPath));
    let instance = await WebAssembly.instantiate(wasm, importObject);

    wasi.start(instance);
}

function exit(error) {
    console.error(error);
    process.exit(1);
}

main();