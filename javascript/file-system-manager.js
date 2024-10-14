export class FileSystemManager {
    constructor({ getPathModule, getFileSystemModule, getFileSystemRootPath }) {
        this._getPathModule = getPathModule;
        this._getFileSystemModule = getFileSystemModule;
        this._getFileSystemRootPath = getFileSystemRootPath;
    }

    writeFile(path, bytes) {
        let { writeFileSync, existsSync, mkdirSync } = this._getFileSystemModule();
        let { dirname } = this._getPathModule();
        let filePath = this._formatPath(path);
        let dirPath = dirname(filePath);

        if (!existsSync(dirPath)) {
            mkdirSync(dirPath, { recursive: true });
        }

        writeFileSync(filePath, bytes, null);
    }

    readFile(path) {
        let { readFileSync, existsSync } = this._getFileSystemModule();
        let filePath = this._formatPath(path);

        if (!existsSync(filePath)) {
            return new Uint8Array();
        }

        return readFileSync(filePath, null);
    }

    _formatPath(path) {
        let { join, sep } = this._getPathModule();

        return join(this._getFileSystemRootPath(), path.split('/').join(sep));
    }
}