import path from 'path';
import { PROJECT_BUILD_DIR_NAME, PROJECT_SRC_DIR_NAME, OUTPUT_WASM_FILE_NAME, OUTPUT_CLIENT_FILE_NAME, OUTPUT_SERVER_FILE_NAME } from './constants';

function findRootDirectory() {
    return path.resolve('.');
}

export function computeLocations(root) {
    let rootDirPath = path.resolve(root || findRootDirectory());
    let srcDirPath = path.join(rootDirPath, PROJECT_SRC_DIR_NAME);
    let buildDirPath = path.join(rootDirPath, PROJECT_BUILD_DIR_NAME);
    let outputWasmFilePath = path.join(buildDirPath, OUTPUT_WASM_FILE_NAME);
    let outputClientFilePath = path.join(buildDirPath, OUTPUT_CLIENT_FILE_NAME);
    let outputServerFilePath = path.join(buildDirPath, OUTPUT_SERVER_FILE_NAME);
    let outputIndexHtmlFilePath = path.join(buildDirPath, 'index.html');

    return { rootDirPath, srcDirPath, buildDirPath, outputWasmFilePath, outputClientFilePath, outputServerFilePath, outputIndexHtmlFilePath };
}