## Installation

- Install Node.js & npm (tested with 17.9.1)
- Install [rust](https://www.rust-lang.org/tools/install)
- Install [outpost](https://github.com/symil/outpost)
- Install the project with:
```
$ git clone git@github.com:symil/lotus.git
$ cd lotus
$ ./install.sh
```

Additional information:

- The install script assumes you are running Ubuntu, you might need to tweak it a little if you're using another distribution.
- It will switch your active Rust toolchain to nightly. This is necessary because this project uses some nightly-only features.

## Usage

Run the tests:

```
$ ./test.sh -a
```

## Real example

You can find a small, test game made with this project [here](https://github.com/symil/mesys).
