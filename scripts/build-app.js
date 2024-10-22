#!/usr/bin/env node

import { execSync } from 'child_process';
import { existsSync, mkdirSync, rmSync, statSync, writeFileSync } from 'fs';
import { join, resolve } from 'path';
import esbuild from 'esbuild';
import { OUTPUT_WASM_FILE_NAME, OUTPUT_WAT_FILE_NAME, SERVER_EXTERNAL_MODULES } from '../javascript/constants.js';
import { ROOT_DIR_PATH } from '../javascript/paths.js';

const COMPILER_PATH = join(ROOT_DIR_PATH, 'target', 'release', 'lotus-compiler');
const CLIENT_ENTRY_PATH = join(ROOT_DIR_PATH, 'javascript', 'client-entry-point.js');
const SERVER_ENTRY_PATH = join(ROOT_DIR_PATH, 'javascript', 'server-entry-point.js');

async function main() {
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

    let buildDir = resolve(inputDir, 'build');
    let watPath = join(buildDir, OUTPUT_WAT_FILE_NAME);
    let wasmPath = join(buildDir, OUTPUT_WASM_FILE_NAME);
    let clientOutputPath = join(buildDir, 'client-bundle.js');
    let serverOutputPath = join(buildDir, 'server-bundle.js');
    let packageJsonPath = join(buildDir, 'package.json');

    logStep(`Creating build directory`);
    rmSync(buildDir, { recursive: true, force: true });
    mkdirSync(buildDir);

    logStep(`Compiling source to WAT`);
    runCommand(`${COMPILER_PATH} ${inputDir} ${watPath} --app --silent`);

    logStep(`Compiling WAT to WASM`);
    runCommand(`wat2wasm --debug-names ${watPath} -o ${wasmPath}`);

    logStep(`Compiling client bundle...`);
    await buildBundle(CLIENT_ENTRY_PATH, clientOutputPath, false);

    logStep(`Compiling server bundle...`);
    await buildBundle(SERVER_ENTRY_PATH, serverOutputPath, true);

    logStep(`Creating package.json...`);
    createPackageJson(packageJsonPath);

    logStep('Installing external packages...');
    runCommand(`npm install`);
}

function exitWithError(message) {
    console.error(`Error: ${message}.`);
    process.exit(1);
}

function logStep(message) {
    console.log(`> ${message}...`);
}

function runCommand(command) {
    execSync(command, { stdio: 'inherit' });
}


async function buildBundle(inputPath, outputPath, isServer) {
    return esbuild
        .build({
            absWorkingDir: process.cwd(),
            entryPoints: [inputPath],
            bundle: true,
            outfile: outputPath,
            minify: false,
            sourcemap: true,
            platform: isServer ? 'node' : 'browser',
            format: 'esm',
            external: ['node:*', ...SERVER_EXTERNAL_MODULES],
            define: {},
            loader: {},
        })
        .then(() => true)
        .catch((err) => {
            return false;
        });
}

function createPackageJson(outputPath) {
    let json = {
        type: "module",
        dependencies: {
            "express": "latest",
            "ws": "latest",
        },
    };
    let content = JSON.stringify(json, null, '  ');

    writeFileSync(outputPath, content);

}

main();