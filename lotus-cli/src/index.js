import path from 'path';
import fs from 'fs';
import chalk from 'chalk';
import express from 'express';
import esbuild from 'esbuild';
import { wasmLoader } from 'esbuild-plugin-wasm';
import { CLIENT_ENTRY_PATH, COMPILER_BINARY_PATH, COMPILER_DIR, OUTPUT_CLIENT_FILE_NAME, SERVER_ENTRY_PATH, WAT2WASM_BINARY_PATH, WAT2WASM_OPTIONS } from './constants';
import { computeLocations } from './locations';
import { runCommand } from './utils';
import { Command } from './command';

async function main() {
    let root = getOption('--root', '-r') || '.';
    let httpPort = +getOption('--port', '-p') || 8080;
    let locations = computeLocations(root);
    let buildOnly = hasOption('--build', '-b');

    if (buildOnly) {
        await buildProject(locations);
    } else {
        await startHttpServer(locations, httpPort);
    }
}

async function startHttpServer(locations, port) {
    let app = express();
    let serverCommand = new Command('node', [locations.outputServerFilePath]);

    app.get(`/`, (req, res, next) => {
        return serverCommand.stop()
            .then(() => buildProject(locations))
            .then(() => serverCommand.start())
            .then(() => next())
            .catch(e => serveError(res, e));
    });

    app.use(express.static(locations.buildDirPath));
    app.use(express.static(locations.rootDirPath));
    app.listen(port, 'localhost');

    console.log(`${chalk.bold('> info:')} listening on port ${chalk.bold(port)}...`);
}

async function buildProject(locations) {
    if (!compileCompiler()) {
        process.exit(1);
    }

    if (!fs.existsSync(locations.nodeModulesDirPath)) {
        runCommand(`cd ${locations.buildDirPath} && npm install ws`);
    }

    process.stdout.write(chalk.bold.blue('> build bundle...'));


    let ok = await compileLotus({
        inputPath: locations.srcDirPath,
        outputPath: locations.outputWasmFilePath
    });

    if (!ok) {
        if (fs.existsSync(locations.outputIndexHtmlFilePath)) {
            fs.unlinkSync(locations.outputIndexHtmlFilePath);
        }

        return;
    }
    
    await compileJs({
        inputPath: CLIENT_ENTRY_PATH,
        outputPath: locations.outputClientFilePath,
        platform: 'browser'
    });
    await compileJs({
        inputPath: SERVER_ENTRY_PATH,
        outputPath: locations.outputServerFilePath,
        platform: 'node'
    });
    writeIndexHtml({
        title: path.basename(locations.rootDirPath),
        bundleFileName: OUTPUT_CLIENT_FILE_NAME,
        outputPath: locations.outputIndexHtmlFilePath,
    });

    process.stdout.write(chalk.bold.blue(' ok\n'));
}

function compileCompiler() {
    return runCommand(`cd ${COMPILER_DIR} && cargo build`);
}

function compileLotus({ inputPath, outputPath }) {
    let wasmFilePath = outputPath;
    let watFilePath = outputPath.replace('.wasm', '.wat');

    return runCommand(`${COMPILER_BINARY_PATH} ${inputPath} ${watFilePath} --silent`)
        && runCommand(`${WAT2WASM_BINARY_PATH} ${WAT2WASM_OPTIONS} ${watFilePath} -o ${wasmFilePath}`);
}

async function compileJs({ inputPath, outputPath, platform }) {
    return esbuild.build({
        entryPoints: [inputPath],
        bundle: true,
        outfile: outputPath,
        minify: false,
        sourcemap: true,
        platform,
        external: ['ws'],
        plugins: [ wasmLoader({}) ]
    });
}

function writeIndexHtml({ title, bundleFileName, outputPath }) {
    let content = [
        `<title>${title}</title>`,
        `<script src="/${bundleFileName}"></script>`
    ].join('\n');

    fs.writeFileSync(outputPath, content, 'utf8');
}

function serveError(response, error) {
    let content = `<p style="font-family: Helvetica; font-size: large;">${error.toString().replaceAll('\n', '<br>')}</p>`;
    let mimeType = 'text/html';

    response.set('Content-Type', mimeType);
    response.status(500);
    response.send(Buffer.from(content));
}

function getOption(long, short) {
    let options = [long, short];
    let index = process.argv.findIndex(str => options.includes(str));
    let value = process.argv[index + 1];

    if (index == -1 || !value || value.startsWith('-')) {
        return null;
    } else {
        return value;
    }
}

function hasOption(long, short) {
    return !!process.argv.find(str => str === long || str === short);
}

main();