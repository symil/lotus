import path from 'path';
import { CLIENT_ENTRY_PATH, COMPILER_BINARY_PATH, DEFAULT_HTTP_PORT, SERVER_CONFIG_FILE_NAME, SERVER_ENTRY_PATH, WAT2WASM_BINARY_PATH, WAT2WASM_OPTIONS, WRITE_SERVER_CONFIG_SCRIPT_PATH } from './constants';
import { execSync } from 'child_process';

const REQUIRED_NODE_PACKAGES = ['ws', 'express'];
const BUILD_DIRECTORY_NAME = 'build';
const FORWARDED_OPTIONS = ['-b', '--build', '-u', '--upload', '--remote-start', '--remote-stop'];

async function main() {
    let rootPath = process.cwd();
    let gameId = path.basename(rootPath);
    let remoteHost = process.argv.find(arg => arg.includes('@')) || '';
    let port = +process.argv.find(arg => +arg > 0) || DEFAULT_HTTP_PORT;

    if (remoteHost && !remoteHost.includes('/')) {
        remoteHost += `/${gameId}`;
    }

    let inputPath = path.join(rootPath, 'src');
    let buildPath = path.join(rootPath, BUILD_DIRECTORY_NAME);
    let watFilePath = path.join(buildPath, 'client', 'module.wat');
    let wasmFilePath = path.join(buildPath, 'client', 'module.wasm');
    let configFilePath = path.join(buildPath, 'server', SERVER_CONFIG_FILE_NAME);
    let lotusToWatCommand = `'${COMPILER_BINARY_PATH} ${inputPath} ${watFilePath} --silent'`;
    let watToWasmCommand = `'${WAT2WASM_BINARY_PATH} ${WAT2WASM_OPTIONS} ${watFilePath} -o ${wasmFilePath}'`;
    let buildConfigCommand = `'${WRITE_SERVER_CONFIG_SCRIPT_PATH} ${rootPath} ${configFilePath}'`;

    let command = [
        'vanguard', port, remoteHost,
        '-n', ...REQUIRED_NODE_PACKAGES,
        '-z', buildConfigCommand, lotusToWatCommand, watToWasmCommand,
        '-c', CLIENT_ENTRY_PATH,
        '-s', SERVER_ENTRY_PATH,
        '-o', buildPath
    ];

    for (let option of FORWARDED_OPTIONS) {
        if (process.argv.includes(option)) {
            command.push(option);
        }
    }

    execSync(command.join(' '), { stdio: 'inherit' });
}

main();