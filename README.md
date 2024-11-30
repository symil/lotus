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
- The source code of a project _must_ be located in the `src/` directory.

### General

Lotus is an object oriented language whose syntax is a mix between TypeScript and Rust.

### Example

```
// Declare an enum. The `pub` qualifier makes it usable from anywhere in the project.
pub enum DamageKind {
    Physical,
    Magical,
}

// Declare a class. The `pub` qualifier makes it usable from anywhere in the project.
pub class Character {
    // Class fields. Fields are always public.
    // Is init value is unspecified, it is set to the default value for the type (0 for numbers, empty string for strings, false for booleans).
    name: string = "",
    health: float = 0,
    damages: float = 0,
    armor: float = 0,
    damage_kind: DamageKind = DamageKind::Physical,

    // Class methods. Methods are always public.
    deal_damages(target: Character) -> bool {
        // Any value can always be `none`.
        if (!target) {
            return false;
        }

        // You can use `match` to account for the different possible values of an enum variable.
        let damage_reduction = match target.damage_kind {
            DamageKind::Physical => target.armor,
            DamageKind::Magical => 0,
        };

        target.health -= (self.damages - damage_reduction);

        // The last statement of a function or method is its return value.
        // Here it indicates if the character kills its target.
        target.health <= 0
    }

    // A child class can override a parent class' method if both prefix it with "dyn".
    dyn special_ability(target: Character) {

    }
}

pub class Magician extends Character {
    // A child class can override the parent's fields at initialization and declare new ones.
    self.name = get_magician_name()
    self.health = 10
    self.damages = 5
    self.damage_kind = DamageKind::Magical

    power: float = 10,

    // Example of a static method
    static make_super_magician() -> Magician {
        // Instanciate an object. You can specify values to overrides some fields.
        Magician {
            health: 50,
            damages: 20,
            power: 20,
        }
    }

    // Override a parent "dyn" method.
    dyn special_ability(target: Character){
        target.health += self.power;
    }
}

// Declare a global constant. Prefix with `pub` to make it accessible from anywhere in the project.
const MAGICIAN_NAMES = ["Anior", "Shuxius", "Ibeus", "Otior"];

// Declare a function usable only in the file. Prefix with `pub` to make it accessible from anywhere in the project.
fn get_magician_name() -> string {
    MAGICIAN_NAMES.get_random()
}
```

### Builtin types

- `bool`: 32 bits value representing a boolean

```
let b: bool = false;
```

- `int`: 32 bits integer

```
let n: int = 5;
```

- `float`: 32 bits floating number

```
let f: float = 5;
```

- `char`: 32 bits integer that represents a character

```
let c: char = 'A';
```

- `ptr`: 32 bits integer representing a pointer
```
// TODO
```

- `string`: immutable string of characters

```
let greeting: string = "Hello";
// TODO: list builtin methods
```

- `array`: array of items

```
let numbers: Array<int> = [1, 2, 3];
// TODO: list builtin methods
```

- `set`: set of items

```
let set: Set<int> = Set::new();
// TODO: list builtin methods
```

- `map`: associate items with a value

```
let map: Map<string, int> = Map::new();
// TODO: list builtin methods
```

- `Color`: represents a color with 4 channels r, g, b, a

```
let red = Color::new(255, 0, 0, 255);
let red_hex = #FF0000; // Supports the hex notation out of the box
// TODO: list builtin methods
```

- `DisplaySize`: TODO

- `Rect`: TODO

- `Buffer`: TODO

### System functions

System functions are prefixes with "@".

```
let a: int = @todo(); // Marks the code as not done yet, making it panic when it is reached
@panic("Fatal error that should not happen"); // Make the program stop with the given error message

@log(456); // Calls `.to_string()` on the value and logs the result.
@dbg([1, 2, 3]); // Prints the structure of the given value, more suitable for debugging objects.
@trace("Test"); // Log the given string and print the stacktrace

// Returns the number of milliseconds elapsed since the start of the program (wrapper around performance.now()).
let current_time = @get_current_time();

// Wrapper around `console.time()`.
@time_start("debug");

// Wrapper around `console.timeEnd()`.
@time_end("debug");

// Serialize the specified value and returns the corresponding buffer.
let buffer = @serialize([2, 4, 8]);

// Attempts to deserialize the given buffer into `T`. Returns an instance of T if it was successful, and `none` otherwise.
let object = @deserialize<Array<int>>(buffer);

// Emit an event on a list of targets. More on that later.
@emit(event, [target1, target2]);
```

### Events

TODO

### `none`

TODO

### Program entry point

TODO