import path from 'path';
import url from 'url';

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

export const ROOT_DIR = path.join(__dirname, '..');
export const COMPILER_DIR = path.join(ROOT_DIR, '..', 'lotus-compiler');
export const COMPILER_BINARY_PATH = path.join(COMPILER_DIR, 'target', 'debug', 'lotus-compiler');
export const CLIENT_ENTRY_PATH = path.join(COMPILER_DIR, 'js', 'client.js');
export const SERVER_ENTRY_PATH = path.join(COMPILER_DIR, 'js', 'server.js');
export const WAT2WASM_BINARY_PATH = 'wat2wasm';
export const WAT2WASM_OPTIONS = ['--enable-bulk-memory'];

export const PROJECT_SRC_DIR_NAME = 'src';
export const PROJECT_BUILD_DIR_NAME = 'build';
export const OUTPUT_WASM_FILE_NAME = 'module.wasm';
export const OUTPUT_CLIENT_FILE_NAME = 'client.js';
export const OUTPUT_SERVER_FILE_NAME = 'server.js';