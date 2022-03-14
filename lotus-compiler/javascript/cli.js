import path from 'path';
import { CACHE_DIR_NAME, CLIENT_ENTRY_PATH, COMPILER_BINARY_PATH, HTML_ENTRY_PATH, OUTPUT_WASM_FILE_NAME, OUTPUT_WAT_FILE_NAME, PRE_BUILD_SCRIPTS_DIR_PATH, SERVER_ENTRY_PATH, WAT2WASM_BINARY_PATH, WAT2WASM_OPTIONS } from './constants';
import { execSync } from 'child_process';
import { readPackageDetails } from './package-details';

const REQUIRED_NODE_PACKAGES = ['ws', 'express'];
const BUILD_DIRECTORY_NAME = 'build';
const FORWARDED_OPTIONS = ['-b', '--build', '-u', '--upload', '--remote-start', '--remote-stop'];
const FETCH_SPREADSHEET_OPTION = '-l';
const FETCH_SPREADSHEET_SCRIPT_PATH = path.join(PRE_BUILD_SCRIPTS_DIR_PATH, 'fetch-spreadsheet.sh');

async function main() {
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
    let watFilePath = path.join(buildPath, 'client', OUTPUT_WAT_FILE_NAME);
    let wasmFilePath = path.join(buildPath, 'client', OUTPUT_WASM_FILE_NAME);
    let loadSpreadsheetCommand = '';
    let lotusToWatCommand = `'${COMPILER_BINARY_PATH} ${inputPath} ${watFilePath} --silent'`;
    let watToWasmCommand = `'${WAT2WASM_BINARY_PATH} ${WAT2WASM_OPTIONS.join(' ')} ${watFilePath} -o ${wasmFilePath}'`;

    if (config.spreadsheetUrl && process.argv.includes(FETCH_SPREADSHEET_OPTION)) {
        loadSpreadsheetCommand = `'${FETCH_SPREADSHEET_SCRIPT_PATH} ${config.spreadsheetUrl} ${cachePath}'`;
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

    execSync(command.join(' '), { stdio: 'inherit' });
}

main();