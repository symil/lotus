import path from 'path';
import { CACHE_DIR_NAME, CLIENT_ENTRY_PATH, COMPILER_BINARY_PATH, HTML_ENTRY_PATH, OUTPUT_WASM_FILE_NAME, OUTPUT_WAT_FILE_NAME, PRE_BUILD_SCRIPTS_DIR_PATH, SERVER_ENTRY_PATH, WAT2WASM_BINARY_PATH, WAT2WASM_OPTIONS } from './constants';
import { execSync } from 'child_process';
import { readPackageDetails } from './package-details';
import { runWasmCommandLine } from './command-line-entry-point';

const REQUIRED_NODE_PACKAGES = ['ws', 'express'];
const BUILD_DIRECTORY_NAME = 'build';
const FORWARDED_OPTIONS = ['-b', '--build', '-u', '--upload', '--remote-start', '--remote-stop'];
const DEBUG_COMPILER_OPTION = '-d';
const RUN_WASM_OPTION = '-r';
const FETCH_SPREADSHEET_OPTION = '-l';
const FETCH_SPREADSHEET_SCRIPT_PATH = path.join(PRE_BUILD_SCRIPTS_DIR_PATH, 'fetch-spreadsheet.sh');

async function main() {
    let fetchSpreadsheet = process.argv.includes(FETCH_SPREADSHEET_OPTION);
    let runWasm = process.argv.includes(RUN_WASM_OPTION);
    let useDebugCompiler = process.argv.includes(DEBUG_COMPILER_OPTION);

    let config = readPackageDetails(process.cwd());
    let rootPath = config.packageRootPath;
    let cachePath = path.join(rootPath, CACHE_DIR_NAME);
    let projectId = config.name;
    let remoteHost = config.remote;
    let port = config.port;
    let clientFiles = config.clientFiles.map(name => path.join(rootPath, name));
    let windowTitle = `'${config.title}'`;
    let inputPath = path.join(rootPath, 'src');
    let buildPath = path.join(rootPath, BUILD_DIRECTORY_NAME);
    let clientBuildPath = runWasm ? buildPath : path.join(buildPath, 'client');
    let watFilePath = path.join(clientBuildPath, OUTPUT_WAT_FILE_NAME);
    let wasmFilePath = path.join(clientBuildPath, OUTPUT_WASM_FILE_NAME);
    let appOption = runWasm ? '' : '--app';
    let compilerPath = useDebugCompiler ? COMPILER_BINARY_PATH.replace('release', 'debug') : COMPILER_BINARY_PATH;
    let loadSpreadsheetCommand = '';
    let lotusToWatCommand = `'${compilerPath} ${inputPath} ${watFilePath} ${appOption} --silent'`;
    let watToWasmCommand = `'${WAT2WASM_BINARY_PATH} ${WAT2WASM_OPTIONS.join(' ')} ${watFilePath} -o ${wasmFilePath}'`;

    if (config.spreadsheetUrl && fetchSpreadsheet) {
        loadSpreadsheetCommand = `'${FETCH_SPREADSHEET_SCRIPT_PATH} ${config.spreadsheetUrl} ${cachePath}'`;
    }

    if (runWasm) {
        if (exec([ loadSpreadsheetCommand, lotusToWatCommand, watToWasmCommand ])) {
            runWasmCommandLine(wasmFilePath);
        }
        return;
    }

    let command = [
        'outpost', port, remoteHost,
        '-i', projectId,
        '-t', windowTitle,
        '-n', ...REQUIRED_NODE_PACKAGES,
        '-z', loadSpreadsheetCommand, lotusToWatCommand, watToWasmCommand,
        '-h', HTML_ENTRY_PATH,
        '-c', CLIENT_ENTRY_PATH,
        '-s', SERVER_ENTRY_PATH,
        '-o', buildPath,
        '-cf', ...clientFiles
    ];

    for (let option of FORWARDED_OPTIONS) {
        if (process.argv.includes(option)) {
            command.push(option);
        }
    }

    exec(command.join('\n'));
}

function exec(commands) {
    if (!Array.isArray(commands)) {
        commands = [commands];
    }

    for (let command of commands) {
        if (command.startsWith("'") && command.endsWith("'")) {
            command = command.substring(1, command.length - 1);
        }

        if (!command) {
            continue;
        }

        try {
            execSync(command, { stdio: 'inherit' });
        } catch (e) {
            return false;
        }
    }

    return true;
}

main();