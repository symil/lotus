#!/usr/bin/env node

const { join } = require('path');
const { execSync } = require('child_process');

const CLI_ENTRY_PATH = join(__dirname, '..', 'lotus-compiler', 'javascript', 'cli.js');
const NODE_OPTIONS = [
    '--enable-source-maps',
    '--experimental-specifier-resolution=node',
];
const USER_OPTIONS = process.argv.slice(2);

execSync(`node ${NODE_OPTIONS.join(' ')} ${CLI_ENTRY_PATH} ${USER_OPTIONS.join(' ')}`, { stdio: 'inherit' });