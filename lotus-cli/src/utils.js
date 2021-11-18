import { execSync } from 'child_process';

export function runCommand(command) {
    try {
        execSync(command, { stdio: 'inherit' });

        return true;
    } catch (e) {
        return false;
    }
}