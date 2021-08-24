import path from 'path';
import fse from 'fs-extra';
import chalk from 'chalk';
import { compileParser, runTest, ROOT_DIR, TEST_DIR, OUTPUT_FILE_NAME, SRC_DIR_NAME } from './test-utils';

const TEMPLATE_DIR_PATH = path.join(ROOT_DIR, 'workshop');
const BUILD_DIR = path.join(ROOT_DIR, 'build');
const ARGV = process.argv.slice(2);

async function main() {
    if (!compileParser()) {
        process.exit(1);
    }

    let testName = ARGV.find(string => !string.startsWith('-'));
    let inheritStdio = !testName;
    let displayMemory = ARGV.includes('-m');
    let options = { inheritStdio, displayMemory };

    if (testName) {
        let testDirPath = path.join(TEST_DIR, testName);
        let testDirSrcPath = path.join(testDirPath, SRC_DIR_NAME);
        let formattedDirPath = '`' + testDirPath.replace(ROOT_DIR + '/', '') + '`';

        if (fse.existsSync(testDirPath)) {
            console.log(`${chalk.bold.red('error:')} ${formattedDirPath} already exists`);
            process.exit(1);
            // fse.rmSync(testDirPath, { recursive: true });
        }

        fse.mkdirSync(testDirSrcPath, { recursive: true });
        fse.copySync(TEMPLATE_DIR_PATH, testDirSrcPath);

        let outputFilePath = path.join(testDirPath, OUTPUT_FILE_NAME);
        let outputFileContent = await runTest(testDirSrcPath, testDirPath, options);
        fse.writeFileSync(outputFilePath, outputFileContent);

        setTimeout(() => {
            console.log(`${chalk.bold('generated:')} ${formattedDirPath}`);
        });
    } else {
        await runTest(TEMPLATE_DIR_PATH, BUILD_DIR, options);
    }
}

main();