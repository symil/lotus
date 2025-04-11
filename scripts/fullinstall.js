import { execSync } from 'child_process';
import { cpSync, existsSync } from 'fs';
import { basename, join } from 'path';
import { homedir } from 'os';
import { COMPILER_BINARY_PATH, ROOT_DIR_PATH } from '../javascript/paths.js';

const VS_CODE_EXTENSION_GIT_URL = 'git@github.com:symil/lotus-vscode.git';

// TODO: handle Windows
function main() {
    let vsCodeExtensionsDir = getVsCodeExtensionsDir();
    process.chdir(ROOT_DIR_PATH);

    console.log('> Installing dependencies...');
    runCommand('npm install');

    console.log('> Building compiler...');
    runCommand('cargo build --release');

    console.log('> Installing compiler globally...');
    runCommand('npm link');

    if (vsCodeExtensionsDir) {
        let vsCodeExtensionsPath = join(vsCodeExtensionsDir, 'lotus-vscode');
        let compilerPath = COMPILER_BINARY_PATH;

        if (!existsSync(vsCodeExtensionsPath)) {
            console.log('> Installing VS code extension...');
            runCommand(`git clone ${VS_CODE_EXTENSION_GIT_URL} ${vsCodeExtensionsPath}`);
        }

        process.chdir(vsCodeExtensionsPath);
        runCommand('npm install');
        runCommand('npm run compile');
        
        cpSync(compilerPath, join(vsCodeExtensionsPath, 'server', basename(compilerPath)));
    }

    console.log('> Done!');
}

function runCommand(command) {
    try {
        execSync(command, { stdio: 'inherit' });
    } catch (e) {
        process.exit(1);
    }
}

function getVsCodeExtensionsDir() {
    let linuxPath = join(homedir(), '.vscode', 'extensions');

    if (existsSync(linuxPath)) {
        return linuxPath;
    }

    return null;
}

main();