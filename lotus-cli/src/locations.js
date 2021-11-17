import path from 'path';

const SRC_DIR_NAME = 'src';
const BUILD_DIR_NAME = 'build';

function findRootDirectory() {
    return path.resolve('');
}

export function computeLocations() {
    let rootDirPath = findRootDirectory();
    let srcDirPath = path.join(rootDirPath, SRC_DIR_NAME);
    let buildDirPath = path.join(rootDirPath, BUILD_DIR_NAME);

    return { rootDirPath, srcDirPath, buildDirPath };
}