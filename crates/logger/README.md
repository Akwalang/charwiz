# zero_cost_logger

A **zero-cost, compile-time gated logger** for Rust with optional colors, markup, and aliases.

Designed for **development ergonomics** while guaranteeing that **no logger code, formatting, or log strings end up in production binaries**.


## Features

- **Zero-cost in production** — logging code is fully removed at compile time
- Colored output (ANSI)
- Log levels: `debug`, `log`, `warn`, `error`
- UTC timestamp without external dependencies
- Inline markup: `<red,i>text</>`
- Compile-time markup aliases: `<$>`, `<!>`, `<i+>`, etc.
- No `unsafe`
- No heap allocations when `logger-markup` is disabled


## Installation

### Local / workspace dependency

```toml
[dependencies]
zero_cost_logger = { path = "../../crates/zero_cost_logger", default-features = false }
````


## Feature model (important)

The logger is **enabled only via Cargo features**.

### Core features

| Feature     | Description                |
| ----------- | -------------------------- |
| `logger`    | Enables the logger backend |
| `log-debug` | Enables `debug!()`         |
| `log-info`  | Enables `log!()`           |
| `log-warn`  | Enables `warn!()`          |
| `log-error` | Enables `error!()`         |

### Optional features

| Feature            | Description                                     |
| ------------------ | ----------------------------------------------- |
| `logger-color`     | ANSI colors                                     |
| `logger-timestamp` | Timestamp                                       |
| `logger-markup`    | `<red,i>text</>` markup                         |
| `logger-aliases`   | Compile-time aliases (requires `logger-markup`) |


## Recommended setup (dev / prod)

### Application `Cargo.toml`

```toml
[features]
default = ["dev"]

dev = [
  "zero_cost_logger/logger",
  "zero_cost_logger/log-debug",
  "zero_cost_logger/log-info",
  "zero_cost_logger/log-warn",
  "zero_cost_logger/log-error",
  "zero_cost_logger/logger-color",
  "zero_cost_logger/logger-timestamp",
  "zero_cost_logger/logger-markup",
  "zero_cost_logger/logger-aliases",
]

prod = []
```


### Build commands

```bash
# DEV build (logging enabled)
cargo run --features dev

# PROD build (logger fully removed)
cargo build --release --no-default-features --features prod
```


### Usage

Logging macros work exactly like `format!()` / `println!()`
and fully support **format placeholders**.

```rust
use zero_cost_logger::*;

fn main() {
    let user = "Alice";
    let retries = 3;
    let error = "connection timeout";

    debug!("Debug value: retries = {}", retries);

    log!("User {} has logged in", user);

    warn!(
        "Retry attempt {}/{} for user {}",
        retries,
        5,
        user
    );

    error!(
        "Operation failed for user {}: {}",
        user,
        error
    );
}
```

You can freely mix **format arguments** with **markup**:

```rust
let plugin = "Transformer";
let reason = "Lua scripts could not be loaded";

error!(
    "<$>{}</>: <i->{}</>",
    plugin,
    reason
);
```

And with aliases:

```rust
warn!(
    "<!>Retry {}</>: <i+>{}</>",
    attempt,
    description
);
```

If the corresponding feature is **disabled**, the macro call is compiled to **nothing**:

* no formatting
* no allocations
* no strings in the final binary


## 🎨 Markup

Available with `logger-markup`.

```rust
log!("This is <red>red</> and <green,i>green italic</>");
```

### Supported tokens

* Colors: `red`, `green`, `yellow`, `blue`, `purple`, `cyan`, `white`, `gray`
* Styles:

  * `b` / `bold`
  * `i` / `italic`
  * `u` / `underline`
  * `d` / `dim`
  * `s` / `strike`
  * `r` / `reverse`

---

## 🔤 Compile-time aliases

Aliases are declared **once** in the binary crate.

```rust
use zero_cost_logger::*;

aliases! {
    "$"  => "purple,i",
    "!"  => "yellow",
    "i!" => "yellow,i",
    "+"  => "green",
    "i+" => "green,i",
    "-"  => "red",
    "i-" => "red,i",
    "&"  => "cyan",
    "i&" => "cyan,i",
}
```

Usage:

```rust
error!("<$>Plugin Transformer</>: Plugin scripts could not be loaded");
```


## Verifying zero-cost

### Inspecting the release binary

```bash
strings target/release/your_binary | rg "Your log message"
```

* Strings created **only inside logging macros** → **not present**
* Strings used elsewhere (`Error`, `expect`, etc.) → **will remain**


## Important notes

* The logger does **not** remove strings used outside logging macros
* For true zero-cost:

  * keep log strings **inside `log!/warn!/error!`**
  * avoid reusing them in `Result`, `Error`, `panic!`, `expect()`
* `logger-markup` uses `String` allocation — intended for dev only


## Use cases

✔ CLI tools
✔ System utilities
✔ Low-memory applications
✔ Security-sensitive software
✔ Projects requiring strict binary control


## License

MIT


## Philosophy

> Logs are a developer tool,
> not a runtime requirement.
