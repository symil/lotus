import { spawn } from 'child_process';

export class Command {
    constructor(name, args = []) {
        this._name = name;
        this._args = args;
        this._process = null;
        this._running = false;
    }

    async start() {
        if (this._running) {
            return;
        }

        this._process = spawn(this._name, this._args, { stdio: 'inherit' });
        this._process.on('close', (code) => {
            this._running = false;
            this._process.removeAllListeners();
            this._process = null;
        });
        this._running = true;

        return new Promise(resolve => setTimeout(resolve, 200));
    }

    async stop() {
        if (!this._running) {
            return;
        }

        return new Promise(resolve => {
            this._process.on('exit', () => {
                this._process.removeAllListeners();
                this._process = null;
                resolve();
            });
            this._process.kill('SIGKILL');
            this._running = false;
        });
    }
}