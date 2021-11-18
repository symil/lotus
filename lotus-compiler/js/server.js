function main() {
    let now = Date.now();
    console.log('SERVER START');
    setInterval(() => console.log('update: ' + now), 500);
}

main();