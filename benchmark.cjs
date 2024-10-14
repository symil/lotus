#!/usr/bin/env node

const { execSync, spawn } = require('child_process');
const { readFileSync, fstat, writeFileSync } = require('fs');
const { createConnection } = require('net');
const { join } = require('path');

const COMPILER_PATH = join(__dirname, 'target', 'release', 'lotus-compiler');
const FILE_TO_VALIDATE_PATH = join(__dirname, 'prelude', '_core', 'object.lt');
const FILE_TO_VALIDATE_CONTENT = readFileSync(FILE_TO_VALIDATE_PATH, 'utf8').toString();
const COMMAND = `1##validate##${FILE_TO_VALIDATE_PATH}##-1`;

async function main() {
    execSync(`cd ${__dirname} && cargo build --release`, { stdio: 'inherit' });
    let serverProcess = await startServer();
    let connection = await connectToServer();

    serverProcess.stdout.on('data', data => console.log(data.toString().trim()));
    serverProcess.stderr.on('data', data => console.log(data.toString().trim()));

    for (let i = 0; i < 200; ++i) {
        writeFileSync(FILE_TO_VALIDATE_PATH, FILE_TO_VALIDATE_CONTENT, 'utf8');
        connection.write(COMMAND);
        await wait(1000);
        displayMemoryUsage();
    }
}

async function startServer() {
    return new Promise(resolve => {
        let process = spawn(COMPILER_PATH, ['--server']);
        process.stdout.once('data', () => resolve(process));
    });
}

async function connectToServer() {
    return new Promise(resolve => {
        let connection = createConnection(9609);
        connection.once('connect', () => resolve(connection));
    });
}

function displayMemoryUsage() {
	let output = execSync(`ps aux | grep lotus-compiler`).toString();
	let memoryUsage = output.split('\n')[0].split(' ').filter(x => x)[3];

	console.log(`MEMORY USAGE: ${memoryUsage}%`);
}

async function wait(duration) {
    return new Promise(resolve => setTimeout(resolve, duration));
}

main();