import path from 'path';
import fs from 'fs';
import chalk from 'chalk';
import { compileParser, runTest, SOURCE_EXTENSION, ROOT_DIR, TEST_DIR } from './test-utils';

const TEMPLATE_FILE_PATH = path.join(TEST_DIR, 'template.lt');
const BUILD_DIR = path.join(ROOT_DIR, 'build');
const ARGV = process.argv.slice(2);

async function main() {
    if (!compileParser()) {
        process.exit(1);
    }

    let testName = ARGV.find(string => !string.startsWith('-'));
    let inheritStdio = !testName;
    let displayMemory = ARGV.includes('-m');

    if (testName) {
        let testDirPath = path.join(TEST_DIR, testName);

        if (fs.existsSync(testDirPath)) {
            fs.rmSync(testDirPath, { recursive: true });
        }

        fs.mkdirSync(testDirPath);

        let inputFilePath = path.join(testDirPath, `${testName}${SOURCE_EXTENSION}`);
        let inputFileContent = fs.readFileSync(TEMPLATE_FILE_PATH, 'utf8');
        fs.writeFileSync(inputFilePath, inputFileContent);

        let outputFilePath = path.join(testDirPath, `${testName}.txt`);
        let outputFileContent = await runTest(inputFilePath, testDirPath, { inheritStdio, displayMemory });
        fs.writeFileSync(outputFilePath, outputFileContent);

        setTimeout(() => {
            console.log(`${chalk.bold('generated:')} ${testDirPath.replace(ROOT_DIR + '/', '')}`);
        });
    } else {
        await runTest(TEMPLATE_FILE_PATH, BUILD_DIR, { inheritStdio, displayMemory });
    }
}

main();