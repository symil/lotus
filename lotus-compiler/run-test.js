#!/usr/bin/env node

import path from 'path';
import fse from 'fs-extra';
import assert from 'assert';
import { execSync } from 'child_process';
import { fileURLToPath } from 'url';
import chalk from 'chalk';
import { initializeWasm } from './js/wasm-initialization';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const ROOT_DIR = path.join(__dirname);
const TEST_DIR = path.join(ROOT_DIR, 'test');
const WORKSHOP_DIR = path.join(ROOT_DIR, 'workshop');
const BUILD_DIR = path.join(WORKSHOP_DIR, 'build');

const WAT2WASM_BINARY_PATH = 'wat2wasm';
const WAT2WASM_OPTIONS = ['--enable-bulk-memory'];

const MOCHA_BINARY_PATH = path.join(ROOT_DIR, 'node_modules', '.bin', 'mocha');

const SRC_DIR_NAME = 'src';
const OUTPUT_FILE_NAME = 'output.txt';
const WAT_FILE_NAME = 'module.wat';
const WASM_FILE_NAME = 'module.wasm';

const ARGV = process.argv.slice(2);

async function main() {
    let isMocha = process.argv.some(str => str.includes('mocha'));
    let commandLineNames = ARGV.filter(str => !str.startsWith('-'));
    let benchmark = hasOption('--benchmark', '-b');
    let overwrite = hasOption('--overwrite', '-o');
    let showDetails = hasOption('--details', '-d');
    let runAll = hasOption('--all', '-a');
    let createTest = overwrite || hasOption('--write', '-w');
    let forceRelease = hasOption('--release', '-r');
    let mode = ((forceRelease || benchmark || runAll || (!isMocha && commandLineNames.length > 1)) && !createTest) ? 'release' : 'debug';
    let overwriteExpectedOutput = hasOption('--overwrite-output', '-ov');
    let validate = hasOption('--validate', '-v');
    let inheritStdio = !createTest;
    let displayMemory = hasOption('--memory', '-m');
    let clean = hasOption('--clean');
    let onlyCompileWat = false;
    let testOptions = { inheritStdio, displayMemory, onlyCompileWat, showDetails, mode, validate, benchmark };

    if (!isMocha && !compileCompiler({ mode })) {
        process.exit(1);
    }

    if (isMocha) {
        let testsToRun = process.env.LOTUS_TESTS.split(' ');
        let validateOutput = !!process.env.LOTUS_VALIDATE_TESTS;

        if (testsToRun.length > 1) {
            mode = 'release';
        }

        describe('Lotus', () => {
            for (let dirName of testsToRun) {
                let testName = dirName;
                let dirPath = path.join(TEST_DIR, dirName);
                let sourcePath = path.join(dirPath, SRC_DIR_NAME);
                let expectedOutputPath = path.join(dirPath, OUTPUT_FILE_NAME);

                it(testName, async () => {
                    let actualOutput = await runTest(sourcePath, dirPath, { mode });
                    let expectedOutput = fse.readFileSync(expectedOutputPath, 'utf8');

                    if (validateOutput) {
                        fse.writeFileSync(expectedOutputPath, actualOutput, 'utf8');
                        expectedOutput = actualOutput;
                    }

                    assert.strictEqual(actualOutput, expectedOutput);
                });
            }
        });
    } else if (clean) {
        let fileList = readDirRecursive(TEST_DIR);
        let wasmFileList = fileList.filter(({ path }) => path.endsWith('.wat') || path.endsWith('.wasm'));
        let buildDirList = fileList.filter(({ type, path }) => type == 'directory' && path.endsWith('build'));

        for (let { path } of wasmFileList) {
            fse.unlinkSync(path);
        }

        for (let { path } of buildDirList) {
            fse.rmdirSync(path);
        }

        console.log(chalk.bold(`${wasmFileList.length} files removed`));
    } else if (createTest) {
        let testName = commandLineNames[0];

        if (!testName) {
            console.log(`${chalk.bold.red('error:')} missing test name`);
            process.exit(1);
        }

        let testDirPath = path.join(TEST_DIR, testName);
        let testDirSrcPath = path.join(testDirPath, SRC_DIR_NAME);
        let formattedDirPath = '`' + testDirPath.replace(ROOT_DIR + '/', '') + '`';

        if (fse.existsSync(testDirPath)) {
            if (overwrite) {
                fse.rmSync(testDirPath, { recursive: true });
            } else {
                console.log(`${chalk.bold.red('error:')} ${formattedDirPath} already exists (specify \`--overwrite\` to overwrite)`);
                process.exit(1);
            }
        }

        fse.mkdirSync(testDirSrcPath, { recursive: true });
        fse.copySync(WORKSHOP_DIR, testDirSrcPath);

        let outputFilePath = path.join(testDirPath, OUTPUT_FILE_NAME);
        let outputFileContent = await runTest(testDirSrcPath, testDirPath, testOptions);
        fse.writeFileSync(outputFilePath, outputFileContent);

        setTimeout(() => {
            console.log(`${chalk.bold('generated:')} ${formattedDirPath}`);
        });
    } else if (runAll || commandLineNames.length > 0) {
        let testDirectoryList = fse.readdirSync(TEST_DIR).filter(dirName => fse.statSync(path.join(TEST_DIR, dirName)).isDirectory());
        let commandLineTests = testDirectoryList.filter(dirPath => commandLineNames.includes(path.basename(dirPath)));
        let testsToRun = runAll ? testDirectoryList : commandLineTests;

        process.env.LOTUS_TESTS = testsToRun.join(' ');
        if (overwriteExpectedOutput) {
            process.env.LOTUS_VALIDATE_TESTS = 'true';
        }

        runCommand(`${MOCHA_BINARY_PATH} --experimental-specifier-resolution=node ${__filename}`, true);
    } else {
        await runTest(WORKSHOP_DIR, BUILD_DIR, testOptions);
    }
}

function readDirRecursive(dirPath) {
    let list = [];

    for (let name of fse.readdirSync(dirPath)) {
        let fullPath = path.join(dirPath, name);
        let stats = fse.statSync(fullPath);

        if (stats.isDirectory()) {
            list.push({ type: 'directory', path: fullPath });
            list.push.apply(list, readDirRecursive(fullPath));
        } else if (stats.isFile()) {
            list.push({ type: 'file', path: fullPath });
        }
    }

    return list;
}

function hasOption(longOption, shortOption) {
    return ARGV.includes(longOption) || ARGV.includes(shortOption);
}

function compileCompiler({ mode = 'debug' } = {}) {
    let option = mode === "release" ? '--release' : '';

    return runCommand(`cd ${ROOT_DIR} && cargo build --${option}`, true).success;
}

function compileLotus({ inputPath, outputPath, inheritStdio, showDetails, mode, validate, benchmark }) {
    let compilerPath = path.join(ROOT_DIR, 'target', mode, 'lotus-compiler');
    let silentOption = inheritStdio ? '' : '--silent';
    let detailsOption = showDetails ? '--details' : '';
    let validateOption = validate ? '--validate' : '';
    let benchmarkOption = benchmark ? '--benchmark' : '';
    let command = `${compilerPath} ${inputPath} ${outputPath} ${silentOption} ${detailsOption} ${validateOption} ${benchmarkOption}`;

    // console.log(command);

    return runCommand(command, inheritStdio);
}

function compileWat(inputPath, outputPath, inheritStdio) {
    return runCommand(`${WAT2WASM_BINARY_PATH} ${WAT2WASM_OPTIONS.join(' ')} ${inputPath} -o ${outputPath}`, inheritStdio);
}

async function runWasm(wasmPath, inheritStdio, displayMemory) {
    let lines = [];
    let log = inheritStdio ? console.log : value => lines.push(value.toString());
    let instance = await initializeWasm(fse.readFileSync(wasmPath, null), { log });

    instance.exports.main();

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

async function runTest(sourceDirPath, buildDirectory, { inheritStdio = false, displayMemory = false, onlyCompileWat = false, showDetails = false, mode = 'debug', validate = false, benchmark = false } = {}) {
    let watPath = path.join(buildDirectory, WAT_FILE_NAME);
    let wasmPath = path.join(buildDirectory, WASM_FILE_NAME);
    let commandChain = [
        () => compileLotus({ inputPath: sourceDirPath, outputPath: watPath, inheritStdio, mode, showDetails, validate, benchmark }),
        () => compileWat(watPath, wasmPath, inheritStdio),
        () => runWasm(wasmPath, inheritStdio, displayMemory)
    ];

    if (validate || benchmark) {
        commandChain.pop();
        commandChain.pop();
    } else if (onlyCompileWat) {
        commandChain.pop();
    }

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

function runCommand(command, inheritStdio) {
    let result = '';
    let success = false;
    let options = {};

    if (inheritStdio) {
        options.stdio = 'inherit';
    }

    // console.log(command);

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

main();