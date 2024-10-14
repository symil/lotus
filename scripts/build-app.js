#!/usr/bin/env node

import { execSync } from 'child_process';
import { existsSync, mkdirSync, rmSync, statSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const ROOT_DIR = join(__dirname, '..');
const COMPILER_PATH = join(ROOT_DIR, 'target', 'release', 'lotus-compiler');

function main() {
    let argv = process.argv.slice(2);
    let inputDir = argv[0];

    if (!inputDir) {
        exitWithError(`missing input directory`);
    }

    if (!existsSync(inputDir)) {
        exitWithError(`path ${inputDir} does not exists`);
    }

    if (!statSync(inputDir).isDirectory()) {
        exitWithError(`path ${inputDir} is not a directory`);
    }

    let buildDir = join(inputDir, 'build');
    let watPath = join(buildDir, 'build.wat');
    let wasmPath = watPath.replace('.wat', '.wasm');

    logStep(`Creating build directory`);
    rmSync(buildDir, { recursive: true, force: true });
    mkdirSync(buildDir);

    logStep(`Compiling source to WAT`);
    execSync(`${COMPILER_PATH} ${inputDir} ${watPath} --app --silent`);

    logStep(`Compiling WAT to WASM`);
    execSync(`wat2wasm --debug-names ${watPath} -o ${wasmPath}`);
}

function exitWithError(message) {
    console.error(`Error: ${message}.`);
    process.exit(1);
}

function logStep(message) {
    console.log(`> ${message}...`);
}

main();