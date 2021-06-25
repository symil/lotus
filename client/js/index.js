import { Client } from './client.js';

async function main() {
    let { start, update } = await import('../pkg/lotus_client.js');
    let client = new Client();

    setupGlobalApi(client);

    await client.start();

    start();

    setInterval(update, 200);
}

function toSnakeCase(string) {
    return string.replace(/[a-z][A-Z]/g, str => str[0] + '_' + str[1]).toLowerCase();
}

function setupGlobalApi(client) {
    for (let name of Object.getOwnPropertyNames(Client.prototype)) {
        if (name.startsWith('$')) {
            let rustName = toSnakeCase(name.substring(1));

            window[rustName] = function() {
                return client[name](...arguments);
            };
        }
    }
}

main();