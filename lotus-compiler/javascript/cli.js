import path from 'path';
import { CLIENT_ENTRY_PATH, COMPILER_BINARY_PATH, DEFAULT_HTTP_PORT, HTML_ENTRY_PATH, OUTPUT_WASM_FILE_NAME, OUTPUT_WAT_FILE_NAME, SERVER_CONFIG_FILE_NAME, SERVER_ENTRY_PATH, WAT2WASM_BINARY_PATH, WAT2WASM_OPTIONS } from './constants';
import { execSync } from 'child_process';
import { readPackageDetails } from './package-details';

const REQUIRED_NODE_PACKAGES = ['ws', 'express'];
const BUILD_DIRECTORY_NAME = 'build';
const FORWARDED_OPTIONS = ['-b', '--build', '-u', '--upload', '--remote-start', '--remote-stop'];

async function main() {
    let config = readPackageDetails(process.cwd());
    let rootPath = config.packageRootPath;
    let projectId = config.name;
    let remoteHost = process.argv.find(arg => arg.includes('@')) || '';
    let port = +process.argv.find(arg => +arg > 0) || DEFAULT_HTTP_PORT;

    let inputPath = path.join(rootPath, 'src');
    let buildPath = path.join(rootPath, BUILD_DIRECTORY_NAME);
    let watFilePath = path.join(buildPath, 'client', OUTPUT_WAT_FILE_NAME);
    let wasmFilePath = path.join(buildPath, 'client', OUTPUT_WASM_FILE_NAME);
    let lotusToWatCommand = `'${COMPILER_BINARY_PATH} ${inputPath} ${watFilePath} --silent'`;
    let watToWasmCommand = `'${WAT2WASM_BINARY_PATH} ${WAT2WASM_OPTIONS.join(' ')} ${watFilePath} -o ${wasmFilePath}'`;

    let command = [
        'outpost', port, remoteHost,
        '-i', projectId,
        '-n', ...REQUIRED_NODE_PACKAGES,
        '-z', lotusToWatCommand, watToWasmCommand,
        '-h', HTML_ENTRY_PATH,
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