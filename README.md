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
cd lotus
```

- Run the `fullinstall` command:

```sh
npm run fullinstall
```

### Troubleshooting

If this command fails, you can run each step separately:

- Install the dependencies:

```sh
npm install
```

- Build the compiler:

```sh
cargo build --release
```

- (Optional) Install the compiler globally:

```sh
npm link
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

The generated build folder is standalone. It can be deployed and used on any server with Node.js installed.

## Tests

- Run the tests with:

```sh
npm test
```

## Repository

Here are listed the most important folders in the repository:

- `example/`: A simple multiplayer platform/shooter game made with Lotus.
- `javascript/`: JavaScript code that wraps the generated WebAssembly.
- `prelude/`: Base Lotus code that is included in every project. Provides core features such as memory management.
- `scripts/`: Utility scripts, including `build-app.js` to build a project.
- `src/`: Source code for the compiler (in Rust).
- `test/`: Test files.

## Language

### An important note

This language has some unique features, many of them being related to its server/client and game aspects. Among these features, some ended up being great and very useful, some of them ended up being straight up terrible.

All in all this project is only really the first iteration towards a more complete tool that would allow anyone to turn a neat multiplayer game idea into a playable prototype within a few hours.

### Project structure

- The `assets/` directory is always copied into the build folder. Any image/sound/etc used by the game must go there.
- The source code of a project must be located in the `src/` directory.
- Optionally, a `lotus.toml` file can be placed at the root of the project (next to `src/`). It marks the project root and can specify a few options: `name` (package name), `framework = true` (include the prelude's UI framework, excluded by default).

### General

Lotus is an object oriented language whose syntax is a mix between TypeScript and Rust.

A few general points not covered elsewhere:

- The last statement of a function, method or block is its return value.
- Blocks are expressions.
- Anonymous functions and closures are supported: `array.filter(item => item.health > 0)`. Function types are written `fn(int)(string)` (a function taking an `int` and returning a `string`).
- `for` loops iterate over arrays and ranges: `for item in items { ... }`, `for i in 0..10 { ... }`. Pairs can be destructured in the loop variable: `for (key, value) in map.entries() { ... }`.
- `match` works on enums, integers, booleans and object types.
- Fields, methods and types are documented via autocompletion if you use the VS code extension; this README only covers the most useful parts of the API.

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
    // If its init value is unspecified, a field is set to the default value for the type (0 for numbers, empty string for strings, false for booleans).
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

### Literals

- Integers: `42`, `-3`, hexadecimal `0xFF`.
- Floats: `3.14`, or the `f` suffix to force a float without a decimal point: `5f`. A bare number literal also becomes a float when the expected type is `float`.
- Characters: `'A'`.
- Strings: `"hello"`. Template strings interpolate expressions: `` `health: ${self.health}` ``.
- Colors: `#F00`, `#FF0000`, `#FF0000AA` (RGB, RRGGBB, RRGGBBAA).
- Arrays: `[1, 2, 3]`.
- Display sizes (see `DisplaySize` below): `10r`, `10v`, `0.5w`, `0.5h`, `0.5m`.

### Builtin types

#### `bool`

32 bits value representing a boolean.

```
let b: bool = false;
```

#### `int`

32 bits signed integer.

```
let n: int = 5;
```

Main methods: `min(other)`, `max(other)`, `clamp(min, max)`, `pow(exponent)`, `log2()`, `to_string()`, `to_hexa_string()`, `to_float()`, `to_char()`, `as_float()` (bitwise reinterpretation).

#### `float`

32 bits floating point number.

```
let f: float = 5;
```

- Static methods: `float::infinity()`, `float::epsilon()`.
- Rounding & arithmetic: `abs()`, `neg()`, `ceil()`, `floor()`, `round()`, `trunc()`, `sqrt()`, `sign()`, `pow` via `int`, `min(other)`, `max(other)`, `clamp(min, max)`.
- Utilities: `mix(other, t)` (linear interpolation), `in_range(min, max)`, `is_zero()`, `is_nan()`, `to_int()`, `to_string()`.

#### `char`

32 bits integer that represents a character.

```
let c: char = 'A';
```

Methods: `as_int()` (code point), `to_string()`.

#### `ptr`

Raw typed pointer to linear memory (`ptr` is an alias for `Pointer<int>`). Mostly used internally by the memory allocator and serialization; regular game code should never need it.

```
let p: ptr = @alloc(4);
```

Methods: `is_null()`, `addr()`, `add(offset)`, `copy_to(target, size)`.

#### `string`

Immutable string of characters. Supports template strings: `` `name: ${player.name}` ``.

```
let greeting: string = "Hello";
```

- Queries: `len()`, `is_empty()`, `contains(str)`, `find(str, index)` (first index of substring, or `none`), `starts_with(str)`, `ends_with(str)`.
- Transformations: `substring(start, end)` (negative and `none` bounds allowed), `split(separator)` (an empty separator splits into individual characters), `pad_start(char, target_length)`, `indent(tab_length)`.
- Misc: `to_string()`, `hash()`.

Note: the empty string `""` is `none` (see the `none` section).

#### `Array<T>`

Array of items. `T[]` is a shorthand for `Array<T>` (e.g `int[]`).

```
let numbers: int[] = [1, 2, 3];
```

- Static methods: `Array<T>::new()`, `with_capacity(capacity)`, `from_fill_function(size, index => ...)`.
- Access: `get(index)` (bounds-checked, negative indices count from the end), `first()`, `last()`, `len()`, `is_empty()`, `includes(item)`, `index_of(item)`.
- Mutation: `push(item)`, `push_unique(item)`, `push_if_not_none(item)`, `pop()`, `shift()`, `insert_at(index, item)`, `remove(item)`, `clear()`, `extend(other)`, `append(other)`.
- Functional (return a new array): `map(callback)`, `filter(predicate)`, `filter_map(callback)`, `reduce(init, callback)`, `find(predicate)`, `find_index(predicate)`, `all(predicate)`, `count(predicate)`, `slice(start, end)`, `concat(other)`, `zip(other)`, `clone()`.
- Ordering: `sort(compare)`, `reverse()`, `shuffle()` — these return a sorted/reversed/shuffled copy; use the `*_in_place()` variants to mutate the array instead.
- Misc: `join(separator)`, `get_random()`.

#### `Set<V>`

Set of items.

```
let set: Set<int> = Set::new();
```

Methods: `add(value)` (returns `true` if newly added), `has(value)`, `delete(value)`, `clear()`, `size()`, `values()`.

#### `Map<K, V>`

Associates keys with values.

```
let map: Map<string, int> = Map::new();
```

Methods: `get(key)`, `set(key, value)` (returns the previous value if any), `delete(key)`, `get_or_insert_with(key, () => ...)`, `has` via `get`, `keys()`, `values()`, `entries()` (array of `(K, V)` pairs), `size()`, `is_empty()`, `clear()`, `clone()`, `sort(compare)`.

#### `Pair<T, U>`

A simple two-element tuple, written `(T, U)` in type position. Returned by APIs such as `Map.entries()`, `Array.zip()` or `Rect.split_horizontally()`, and destructurable in `for` loops: `for (key, value) in map.entries() { ... }`.

#### `Color`

Represents a color with 4 integer channels `r`, `g`, `b`, `a` (0-255).

```
let red = Color::new(255, 0, 0, 255);
let red_hex = #FF0000; // Supports the hex notation out of the box
let salmon = Color::salmon(); // Every CSS color name exists as a static method
```

- Static methods: `new(r, g, b, a)`, `from_hash(int)` / `from_string_hash(str)` (derive a stable color from a value), `transparent()`, and one constructor per CSS color name (`red()`, `steel_blue()`, `dark_orange()`, ...).
- Methods: `mult_alpha(ratio)`, `luminance()`, `to_int()`, `get_components()`.

#### `DisplaySize`

A resolution-independent size, used everywhere in the rendering API (text size, border width, margins...). A display size carries a unit that determines how it resolves to actual pixels, expressed with a literal suffix:

| Literal | Meaning |
|---|---|
| `10r` | 10 real pixels |
| `10v` | 10 virtual pixels (scaled with the window's virtual resolution) |
| `0.5w` | half of the container's width |
| `0.5h` | half of the container's height |
| `0.5m` | half of the container's smallest dimension |

A bare number in a `DisplaySize` context is interpreted as a virtual size. Methods: `scale(ratio)`, `div(ratio)`, `resolve(rect, virtual_to_real_ratio)`.

#### `Rect`

A rectangle, with optional depth and rotation. Note that `x`/`y` are the **center** of the rectangle, not a corner. Fields: `x`, `y`, `width`, `height`, `z` (stacking order), `angle` (rotation).

```
let rect = Rect::new(x, y, width, height);
let (left, right) = rect.split_horizontally(200);
```

- Static methods: `new(x, y, width, height)`, `from_top_left(x1, y1, width, height)`, `from_size(width, height)`, `from_corners(x1, y1, x2, y2)`, `from_rect_list(rects)` (bounding box), `empty()`.
- Corners & metrics: `x1()`, `y1()`, `x2()`, `y2()`, `half_width()`, `half_height()`, `aspect_ratio()`, `contains(x, y)`.
- Position: `translate(tx, ty)`, `recenter(x, y)`, `mirror(cx, cy)`, `rotate(angle)`, `elevate(z)`.
- Size: `resize(width, height)`, `scale(ratio)`, `pad(width, height)` (grow), `strip(width, height)` (shrink), `strip_from_sides(top, right, bottom, left)`, `pad_to_match_aspect_ratio(ratio)`, `strip_to_match_aspect_ratio(ratio)`, `round()`.
- Composition: `split_horizontally(left_width)`, `split_vertically(top_height)`, `left_neighbor(width, margin)` / `right_neighbor(...)` / `top_neighbor(...)` / `bottom_neighbor(...)`, `mix(other, t)` (interpolation).

Most of these return a modified copy, making them chainable.

#### `Buffer`

A growable binary buffer, used by the serialization system (`@serialize` / `@deserialize`) and the network layer. You usually only create one to pass it to `@serialize`.

```
let buffer = @serialize([2, 4, 8], Buffer::new());
let numbers = @deserialize<int[]>(buffer);
```

Methods: `write(value)`, `read()` (returns `none` when exhausted), `read_string()`, `reset()`, `consume()` (extract the content as an `int[]`), `is_finished()`, `has_read_overflow()`.

#### `Rng`

A seedable pseudo-random number generator (xoshiro128++).

```
let dice = Rng::int(1, 7); // random int in [1, 7) using the default generator
let ratio = Rng::float(0, 1);
let rng = Rng::with_seed(123); // deterministic generator
let n = rng.next(0, 10);
```

`Array.get_random()` and `Array.shuffle()` use the default generator unless one is passed explicitly.

### `none`

In Lotus there is no `null` and no option type: **any value of any type can be `none`**. `none` is a literal whose type is inferred from context.

Each type has its own `none` value, which is not always the same as its default value:

| Type | `none` value | Note |
|---|---|---|
| objects, arrays, strings... | null pointer | |
| `int` | `i32::MIN` | `0` is a regular value, distinct from `none` |
| `float` | a specific `NaN` | `0.0` is a regular value, distinct from `none` |
| `bool` | `false` | `false` **is** `none` |
| `string` | `""` | the empty string **is** `none` |

The rules that come with it:

- **Truthiness**: a value is truthy if and only if it is not `none`. `if (target)` reads as "if `target` is not none". The `!` operator returns `true` if its operand is `none` (so `!!x` converts anything to an actual `bool`).
- **`&&` and `||` are select operators**, not boolean-only: `a || b` evaluates to `a` if `a` is not `none`, and to `b` otherwise — the idiomatic way to provide a default value. `a && b` evaluates to `none` if `a` is `none`, and to `b` otherwise.

```
let name = local_storage_name || client.prompt("Enter your name:");
```

- Every value also has the methods `or(other)`, `or_else(callback)`, `and(other)` and `and_then(callback)` for the same purpose in method-chaining style.
- **`check <expression>;`** is a guard statement: if the expression is falsy, the enclosing function immediately returns `none`. Very common at the top of event callbacks:

```
@OnKeyDown {
    check MOVEMENT_KEYS.includes(evt.key) && !evt.repeat;
    // ...
}
```

- **`is`** tests and narrows the type of an object, optionally binding a variable: `if object is Character(character) { character.health -= 1; }`. It returns `false` for `none` values.
- **`as`** is an unchecked cast: `event as ClientEvent`.

By default, accessing a field or calling a method on a `none` object is not checked (it reads through a null pointer). Adding the root tag `#enable_check_field_access` to a file makes the program panic with a proper `file:line: cannot access field 'x' of none` message instead, at the cost of a check on every access.

### System functions

System functions are prefixed with `@`.

Debugging:

```
let a: int = @todo(); // Marks the code as not done yet, making it panic when it is reached
@panic("Fatal error that should not happen"); // Make the program stop with the given error message
@assert(#LINE, value == 42); // Runtime assertion; reports the given line number on failure

@log(456); // Calls `.to_string()` on the value and logs the result.
@dbg([1, 2, 3]); // Prints the structure of the given value, more suitable for debugging objects.
@trace("Test"); // Log the given string and print the stacktrace
let s = @get_debug_string(object); // Returns the string that `@dbg` would print

// Returns the number of milliseconds elapsed since the start of the program (wrapper around performance.now()).
let current_time = @get_current_time();

// Wrapper around `console.time()` / `console.timeEnd()`.
@time_start("debug");
@time_end("debug");
```

Serialization:

```
// Serialize the specified value into the given buffer.
let buffer = @serialize([2, 4, 8], Buffer::new());

// Attempts to deserialize the given buffer into `T`. Returns an instance of T if it was successful, and `none` otherwise.
let object = @deserialize<int[]>(buffer);

// Variants that embed a schema, making the result robust to class layout changes (used for persistent storage).
let data = @serialize_with_schema(object);
let object = @deserialize_with_schema<MyClass>(data);
```

Events:

```
// Emit an event on a list of targets. See the "Events" section.
@emit(event, [target1, target2]);
```

Memory management (handled automatically; mostly useful for debugging the prelude itself): `@alloc(size)`, `@retain(value)`, `@trigger_garbage_collection()`, `@get_allocated_block_count()`, `@get_memory_footprint()`.

There are also a few compile-time macros prefixed with `#` (e.g `#LINE`, `#TYPE_ID`, `#TYPE_NAME`), plus the main type macros described in the "Program entry point" section.

### Events

Events are the backbone of a Lotus program: all game logic lives in event callbacks. An event is a plain object; emitting it on a list of target objects runs the matching callbacks declared on each target's class.

Any class can declare a callback for any event type with an `@` block:

```
pub class World {
    @OnWorldUpdate {
        // `self` is the target the event was emitted on (here, the world)
        // `evt` is the event instance
        let current_time = evt.server.get_current_time();
    }
}
```

- **Emission**: `@emit(event, targets)` or equivalently `event.emit(targets)`. Callbacks declared on the event's own class also run, with the event itself as `self`.
- **Priority**: `@OnKeyDown[-1] { ... }` — callbacks with a lower priority run first (default is 0). This allows pre-processing or intercepting an event before regular handlers.
- **`intercept;`** stops the propagation of the current event: no further callback runs for it.
- **`@Self`** declares a callback for the enclosing class itself, on the event's own instance.

A callback can be split into up to three *steps*, turning an event into an animation spread over time when it is emitted through an event chain. Setting `opt.duration` (in seconds) in the `start` step makes the `progress` step run every frame (with `opt.elapsed` and `opt.ratio` filled in) until the `end` step fires:

```
class OnDealDamages {
    source: Character,
    target: Character,
    amount: float,

    @Self {
        opt.duration = 1.8;
    }
    @Self:progress {
        self.target.health -= self.amount * opt.elapsed;
    }
    @Self:end {
        @log(`finished dealing ${self.amount} damages`);
    }
}
```

#### Standard server events

Emitted automatically by the engine on the world object. They all expose `evt.server` and `evt.world`; user-related ones also expose `evt.user`.

| Event | Emitted | Extra fields |
|---|---|---|
| `OnWorldOpen` | once, when the server starts | |
| `OnWorldUpdate` | every server tick (~100/s) | `elapsed` |
| `OnWorldClose` | when the server stops | |
| `OnUserConnect` | when a client connects | |
| `OnUserDisconnect` | when a client disconnects | |
| `OnUserRequest` | when a client sends a request | `request`, `mark_as_success()` |

#### Standard client events

Emitted automatically by the engine on the window object (and on the views concerned, for input events). They all expose `evt.client`, `evt.user`, `evt.local_data` and `evt.view`.

| Event | Emitted | Extra fields |
|---|---|---|
| `OnClientStart` | once, when the page loads | |
| `OnClientConnect` | when the connection with the server is established | |
| `OnClientUpdate` | every frame (~60/s) | |
| `OnRender` | every frame, on each rendered object | see the "Rendering" section |
| `OnMouseDown`, `OnMouseUp`, `OnClick` | mouse input | `button`, `x`, `y` |
| `OnDragStart`, `OnDragProgress`, `OnDragEnd` | mouse drags | `dx`, `dy` |
| `OnScroll` | mouse wheel | `delta_x`, `delta_y` |
| `OnKeyDown`, `OnKeyUp` | keyboard input | `key`, `text`, `ctrl_key`, `shift_key`, `alt_key`, `repeat` |
| `OnFocus` | a view gains focus | |

### Program entry point

There is no `main` function. Instead, a project declares which of its classes play the special roles of the engine, with top-level main type declarations (see `example/src/main.lt`):

```
#USER_TYPE = Player
#WINDOW_TYPE = Window
#WORLD_TYPE = World
```

| Declaration | Role |
|---|---|
| `#WORLD_TYPE` | **Server-side** global game state. A single instance for the whole server. |
| `#USER_TYPE` | One **connected player**. One instance per connection, server-side; it is streamed to that player's client every tick. |
| `#WINDOW_TYPE` | **Client-side** root object of the rendering tree. |
| `#LOCAL_DATA_TYPE` | **Client-side** data object passed to client events (optional). |

Any role not declared defaults to `Object`. The engine exposes these types back to the code as macros: `evt.world` is a `#WORLD_TYPE`, `evt.user` a `#USER_TYPE`, etc.

#### Lifecycle & state synchronization

On the server (one tick every 10 ms):

1. On startup, the world is instantiated and `OnWorldOpen` is emitted.
2. Network events are processed: connections create a `#USER_TYPE` instance and emit `OnUserConnect`, disconnections emit `OnUserDisconnect`, and incoming messages are deserialized into requests and emitted as `OnUserRequest`.
3. `OnWorldUpdate` is emitted on the world.
4. For each connected user, the engine **serializes the whole `#USER_TYPE` object and sends it to that client**. Anything reachable from the user object (typically the game state it references) is included, so each client automatically sees the latest state without any explicit synchronization code.

On the client (one update per animation frame):

1. On page load, `OnClientStart` is emitted.
2. Each frame, the latest user snapshot received from the server is deserialized, `OnClientUpdate` is emitted, then the window is rendered (see the "Rendering" section) and input events are dispatched.

In other words: the server is authoritative, and clients are thin — they send requests, receive their user object back, and render it.

#### Requests (client → server communication)

A request is a regular class with the fields you want to transmit and an `@OnUserRequest` callback containing the server-side logic:

```
pub class FireRequest {
    target_x: float,
    target_y: float,

    @OnUserRequest {
        check evt.user.character;
        evt.world.game.fire(evt.user.character, self.target_x, self.target_y);
    }
}
```

The engine emits `OnUserRequest` on the world; the idiomatic way to route it to the request's own callback is to re-emit it on the request object:

```
pub class World {
    @OnUserRequest {
        evt.emit([evt.request]);
    }
}
```

The client sends a request with:

```
evt.client.send_request(FireRequest { target_x: evt.x, target_y: evt.y });
// Or, to be notified when the server has processed it:
evt.client.send_request_with_callback(request, (success, client) => { ... });
```

#### `Server` API

Available in server-side callbacks via `evt.server`:

- `users()`: array of all connected users.
- `get_current_time()`: seconds elapsed since the server started.
- `store(path, value)` / `load<T>(path)`: serialize a value to / from a file on the server (uses schema serialization, so it survives class changes).
- `emulate_request(user, request)`: process a request as if the given user had sent it.
- `connect_bot(bot)` / `disconnect_bot(bot)`: attach a server-side bot as a virtual user.
- `add_object_to_update(object)` / `remove_object_to_update(object)`: make an extra object receive `OnWorldUpdate` events.

#### `Client` API

Available in client-side callbacks via `evt.client`:

- `user()`: the latest synchronized user object.
- `send_request(request)` / `send_request_with_callback(request, callback)`.
- `render(object, rect)`: render an object as a view (see the "Rendering" section).
- View queries: `get_view(object)`, `hovered_views()`, `focused_view()`, `pressed_views()`, `dragged_views()`.
- Focus management: `focus(view)`, `clear_focus()`, `focus_next()`, `focus_prev()`, `set_focus_chain(views)`...
- Local storage (persisted in the browser): `get_local_storage_item<T>(key)`, `set_local_storage_item(key, value)`, `remove_local_storage_item(key)`, `clear_local_storage()`.
- Browser utilities: `prompt(message)`, `set_window_title(title)`, `get_href()`, `get_hostname()`, `cursor()` (current cursor position), `get_current_time()`.

### Rendering

Rendering is declarative and immediate-mode: every frame, the engine creates a fresh view for the window object and emits `OnRender` on it. Any object can be rendered — a class becomes renderable simply by declaring an `@OnRender` callback. Inside it, `evt.view` is the `View` associated with `self`, and rendering consists in configuring this view and rendering child objects:

```
pub class Window {
    @OnRender {
        evt.view
            .background_color(BACKGROUND_COLOR)
            .render_children(evt.user.game.objects)
    }
}
```

#### Graphics properties

A view exposes chainable setters for all its visual properties:

- **Shape & position**: `set_rect(rect)`, `shape(Shape::Rectangle|Circle|Line)`, `z_index(int)`, `offset_x/offset_y(DisplaySize)`, anchors.
- **Background & border**: `background_color(color)`, `background_alpha`, `overlay_color`, `border_color`, `border_width`, `border_radius`, `border_dash_length`.
- **Image**: `image_url(string)` (e.g `"assets/explosion.png"`), `image_scale`, `image_layout(per_row, per_column)` + `image_sprite_index` (spritesheets), `animation_start_time` / `animation_duration`.
- **Text**: `text(string)`, `text_size(DisplaySize)`, `text_color`, `text_font(Font::SansSerif|Serif|Monospace|...)`, `text_bold`, `text_italic`, `text_padding`, `text_horizontal_align`, `text_vertical_align`.
- **Cursor**: `cursor(Cursor::Pointer|Text|Grab|...)`.

Every property also has `hover_*`, `focus_*` and `disabled_*` variants, automatically applied when the view is in the corresponding state — which makes basic interactive widgets almost free:

```
evt.view
    .background_color(Color::white())
    .border_radius(10v)
    .hover_overlay_color(Color::black().mult_alpha(0.1))
    .hover_cursor(Cursor::Pointer)
```

Views are interactive by default: input events (`OnClick`, `OnMouseDown`, `OnKeyDown`, ...) are emitted on the objects whose views are hovered or focused. `is_hovered()`, `is_focused()`, `is_pressed()`, `focus()`, `set_disabled(bool)` and `set_pointer_behavior(...)` give manual control when needed.

#### Child views

- `render_child(object, rect?)` renders another object as a child view (recursively emitting `OnRender` on it).
- `render_children(objects)` does it for a whole list.

#### Layouts

For UI screens, `evt.view.layout()` starts a flexbox-like layout builder: it splits the view's rectangle among children, which are then rendered as child views. Example from the framework's lobby screen:

```
@OnRender {
    evt.view.layout()
        .top_to_bottom()
        .push(LobbyHeader)
        .height(HEADER_SIZE)
        .push(middle_panel)
        .force(1)
        .push(LobbyFooter)
        .height(FOOTER_SIZE)
}
```

- **Direction**: `top_to_bottom()`, `left_to_right()`, `right_to_left()`, `bottom_to_top()`, and centered variants (`center_to_right()`, ...).
- **Items**: `push(object_or_view)` adds an item, `push_list(items)` adds several, `push_separator()` adds an empty slot. Calling a direction method on an item turns it into a nested container; `back()` returns to the parent container.
- **Sizing**: `width(DisplaySize)` / `height(DisplaySize)` / `aspect_ratio(ratio)` give an item a fixed size; `force(weight)` distributes the remaining space proportionally (like CSS `flex-grow`).
- **Spacing**: `inner_margin(size)` (between children), `outer_margin(size)`, `margin(size)` (both).

The layout builder also re-exposes all the graphics setters, applying them to the last pushed item.
