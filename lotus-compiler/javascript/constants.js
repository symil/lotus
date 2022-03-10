import path from 'path';
import url from 'url';

const __dirname = path.dirname(url.fileURLToPath(import.meta.url));

export const ROOT_DIR_PATH = path.join(__dirname, '..');
export const COMPILER_DIR_PATH = path.join(ROOT_DIR_PATH, '..', 'lotus-compiler');
export const COMPILER_BINARY_PATH = path.join(COMPILER_DIR_PATH, 'target', 'release', 'lotus-compiler');
export const HTML_SRC_DIR_PATH = path.join(COMPILER_DIR_PATH, 'html');
export const JAVASCRIPT_SRC_DIR_PATH = path.join(COMPILER_DIR_PATH, 'javascript');
export const HTML_ENTRY_PATH = path.join(HTML_SRC_DIR_PATH, 'index.html');
export const CLIENT_ENTRY_PATH = path.join(JAVASCRIPT_SRC_DIR_PATH, 'client-entry-point.js');
export const SERVER_ENTRY_PATH = path.join(JAVASCRIPT_SRC_DIR_PATH, 'server-entry-point.js');

export const WAT2WASM_BINARY_PATH = 'wat2wasm';
export const WAT2WASM_OPTIONS = ['--enable-bulk-memory', '--debug-names'];
export const OUTPUT_WAT_FILE_NAME = 'module.wat';
export const OUTPUT_WASM_FILE_NAME = 'module.wasm';

export const DEFAULT_HTTP_PORT = 8000;
export const PACKAGE_CONFIG_FILE_NAME = 'lotus.toml';
export const SERVER_CONFIG_FILE_NAME = 'lotus.json';

export const SERVER_REFRESH_RATE = 20;