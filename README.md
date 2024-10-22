# Lotus

Lotus is specialized programming to create prototypes of multiplayer, 2D video games playable in the browser.
It compiles a source directory into a WebAssembly module, and generates two javascript bundles (for server & client) that run the WebAssembly module.

You can find an example of a game made with it in the `example` folder.

## Installation

- Install [Node.js](https://nodejs.org/en).
- Install [Rust](https://www.rust-lang.org/tools/install).
- Clone the project.

```sh
git clone git@github.com:symil/lotus.git
```

- Install the dependancies.

```sh
cd /path/to/lotus
npm install
```

- Build the compiler.

```sh
cargo build --release
```

- (Optional) Install the compiler globally.

```sh
sudo npm link
```

- (Optional) Install the (VS code extension for Lotus)[https://github.com/symil/lotus-vscode].

## Usage

- Compile the example directory.
```sh
lotus example/
# Or alternatively, if you haven't installed it globally:
# scripts/build-app.js example/
```

- This generates a standalone `example/build` folder. You can then run the game server:
```sh
node example/build/server-bundle.js
```

- Connect to `http://localhost:8000`. To try out the multipler, either ask someone else to join you or open multiple tabs.

## Language specifications

Coming one day.
