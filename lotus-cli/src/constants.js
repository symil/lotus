import path from 'path';
import url from 'url';

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

export const ROOT_DIR_PATH = path.join(__dirname, '..');
export const COMPILER_DIR_PATH = path.join(ROOT_DIR_PATH, '..', 'lotus-compiler');
export const COMPILER_BINARY_PATH = path.join(COMPILER_DIR_PATH, 'target', 'debug', 'lotus-compiler');
export const JAVASCRIPT_SRC_DIR_PATH = path.join(COMPILER_DIR_PATH, 'js');
export const CLIENT_ENTRY_PATH = path.join(JAVASCRIPT_SRC_DIR_PATH, 'client.js');
export const SERVER_ENTRY_PATH = path.join(JAVASCRIPT_SRC_DIR_PATH, 'server.js');
export const SCRIPTS_DIR_PATH = path.join(ROOT_DIR_PATH, 'scripts');
export const WRITE_SERVER_CONFIG_SCRIPT_PATH = path.join(SCRIPTS_DIR_PATH, 'write-server-config.js');

export const WAT2WASM_BINARY_PATH = 'wat2wasm';
export const WAT2WASM_OPTIONS = ['--enable-bulk-memory'];
export const OUTPUT_WAT_FILE_NAME = 'module.wat';
export const OUTPUT_WASM_FILE_NAME = 'module.wasm';

export const DEFAULT_HTTP_PORT = 8000;
export const SERVER_CONFIG_FILE_NAME = 'lotus.json';