# Lotus

Lotus is specialized programming language to create prototypes of multiplayer, 2D video games playable in the browser.
It compiles a source directory into a WebAssembly module, and generates two javascript bundles (for server & client) that run the WebAssembly module.

You can find an example of a simple game made with it in the `example` folder.

## Installation

- Install [Node.js](https://nodejs.org/en).
- Install [Rust](https://www.rust-lang.org/tools/install).
- Clone the project:

```sh
git clone git@github.com:symil/lotus.git
```

- Install the dependancies:

```sh
cd /path/to/lotus
npm install
```

- Build the compiler:

```sh
cargo build --release
```

- (Optional) Install the compiler globally:

```sh
npm link # Must be run as super user on Linux
```

- (Optional) Install the [VS code extension for Lotus](https://github.com/symil/lotus-vscode) to have syntax highlighting and autocompletion.

## Usage

- Compile the example directory:
```sh
lotus example/
# Or alternatively, if you haven't installed it globally:
# scripts/build-app.js example/
```

- This generates a `example/build` folder. You can then run the game server:
```sh
node example/build/server-bundle.js
```

- Connect to `http://localhost:8000`. To try out the multiplayer, either ask someone else to join you or open multiple tabs.

Note: the generated build folder is standalone. It can be deployed and used on any server with Node.js installed.

## Tests

- Run the tests with:
```sh
npm test
```

## Repository

Here are listed the most important folders in the repository:

- `example/`: a simple multiplayer platform/shooter game made with Lotus.
- `javascript/`: JavaScript code that wraps the generated WebAssembly.
- `prelude/`: base Lotus code that is included in every project. Provides core features such as memory management.
- `scripts/`: utility scripts, including `build-app.js` to build a project.
- `src/`: source code for the compiler (in Rust).
- `test/`: test files.

## Language

### An important note

This language has many unique features, many of then being related to its server/client and game aspects. Among these features, some ended up being great and very useful, some of them ended up being straight up terrible.

All in all this project is only really the first iteration towards a more complete tool that would allow anyone to turn a neat multiplayer game idea into a playable prototype within a few hours.

### Project structure

- The `assets/` directory is always copied into the build folder. Any image/sound/etc used by the game must go there.
- The source code of a project *must* be located in the `src/` directory.

### Syntax

TODO
