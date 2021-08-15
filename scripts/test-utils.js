import path from 'path';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import { getImportsObject, runWasmFile } from './wasm-utils';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export const ROOT_DIR = path.join(__dirname, '..');
export const TEST_DIR = path.join(ROOT_DIR, 'test');

export const SOURCE_EXTENSION = '.lt';
export const PARSER_BINARY_PATH = path.join(ROOT_DIR, 'target', 'debug', 'lotus-parser');
export const WAT2WASM_BINARY_PATH = 'wat2wasm';

export const SRC_DIR_NAME = 'src';
export const OUTPUT_FILE_NAME = 'output.txt';
export const WAT_FILE_NAME = 'module.wat';
export const WASM_FILE_NAME = 'module.wasm';

export function compileParser() {
    try {
        execSync('cargo build -p lotus-parser', { stdio: 'inherit' });

        return true;
    } catch (e) {
        return false;
    }
}

function runCommand(command, inheritStdio) {
    let result = '';
    let success = false;
    let options = {};

    if (inheritStdio) {
        options.stdio = 'inherit';
    }

    try {
        result = execSync(command, options)?.toString('utf8');
        success = true;
    } catch (error) {
        if (!inheritStdio) {
            result = error.stderr.toString() + error.stdout.toString();
        }
    }

    return { result, success };
}

function compileLotus(inputPath, outputPath, inheritStdio) {
    return runCommand(`${PARSER_BINARY_PATH} ${inputPath} ${outputPath} ${inheritStdio ? '' : '--silent'}`, inheritStdio);
}

function compileWat(inputPath, outputPath, inheritStdio) {
    return runCommand(`${WAT2WASM_BINARY_PATH} ${inputPath} -o ${outputPath}`, inheritStdio);
}

async function runWasm(wasmPath, inheritStdio, displayMemory) {
    let lines = [];
    let log = inheritStdio ? console.log : value => lines.push(value.toString());
    let importsObject = getImportsObject({ log });

    let instance = await runWasmFile(wasmPath, importsObject);

    if (displayMemory) {
        let memory = new Uint32Array(instance.exports.memory.buffer); 
        
        for (let i = 0; i < 16; ++i) {
            console.log(`${i.toString().padStart(2, ' ')}: ${memory[i]}`);
        }
    }

    let result = lines.join('\n');
    let success = true;

    return { result, success };
}

export async function runTest(sourceDirPath, buildDirectory, { inheritStdio = false, displayMemory = false } = {}) {
    let watPath = path.join(buildDirectory, WAT_FILE_NAME);
    let wasmPath = path.join(buildDirectory, WASM_FILE_NAME);
    let commandChain = [
        () => compileLotus(sourceDirPath, watPath, inheritStdio),
        () => compileWat(watPath, wasmPath, inheritStdio),
        () => runWasm(wasmPath, inheritStdio, displayMemory)
    ];

    let actualOutput = '';

    for (let command of commandChain) {
        let { result, success } = await command();

        actualOutput += result;

        if (!success) {
            break;
        }
    }

    return actualOutput;
}