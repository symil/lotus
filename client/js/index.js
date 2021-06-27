import { Client } from './client.js';

async function main() {
    let client = new Client();

    await client.start();

    window.client = client;
}

main();