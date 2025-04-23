# `tauri_helper`

![Crates.io License](https://img.shields.io/crates/l/tauri_helper)
[![Crates.io](https://img.shields.io/crates/v/tauri_helper)](https://crates.io/crates/tauri_helper)
[![Docs.rs](https://docs.rs/tauri-helper/badge.svg)](https://docs.rs/tauri_helper)
![Crates.io Size](https://img.shields.io/crates/size/tauri_helper)


`tauri_helper` is a collection of tools and utilities designed to simplify the development of Tauri applications. It provides macros, utilities, and automation to streamline common tasks such as command collection, error handling, and workspace management.

This workspace includes the following crates:
- **`tauri_helper_core`**: Core utilities for workspace management and command collection.
- **`tauri_helper_macros`**: Procedural macros for automating Tauri command registration and error handling.

---

## Features

### Core Features
- **Workspace Management**: Automatically detect and manage workspace members.
- **Command Collection**: Collect Tauri commands from workspace members and generate handler invocations.

### Macros
- **`#[auto_collect_command]`**: Automatically collect Tauri commands annotated with this attribute.
- **`specta_collect_commands!`**: Generate a `tauri_specta::collect_commands!` invocation for all collected commands.
- **`tauri_collect_commands!`**: Generate a `tauri::generate_handler!` invocation for all collected commands.
- **`array_collect_commands!`**: Generate an array of collected command names, optionally printing them.
- **`WithLogging`**: Automatically implement `From` for enum variants and optionally log errors using `tracing` (requires the `tracing` feature), this is extremely unstable.

---

## Installation

Add `tauri-helper` to your `Cargo.toml`:

```toml
[dependencies]
tauri-helper = "0.1.4"
```

If you want to use the `WithLogging` macro with `tracing`, enable the `tracing` feature:

```toml
[dependencies]
tauri-helper = { version = "0.1.4", features = ["tracing"] }
```

Then add it to the `[build-dependencies]` :

```toml
[build-dependencies]
tauri-helper = "0.1.4"
```

## IMPORTANT

Before using any command collection, you have to add this to the `build.rs` file.

```rust
    fn main() {
        tauri_helper::generate_command_file(tauri_helper::TauriHelperOptions::default());
        tauri_build::build();
    }
```
---

## Usage

### Command Collection

Annotate your Tauri command functions with `#[auto_collect_command]` to automatically collect them:

```rust
#[tauri::command]
#[auto_collect_command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```

Generate a `tauri::generate_handler!` invocation:

```rust
tauri_collect_commands!();
```

Generate a `tauri_specta::collect_commands!` invocation:

```rust
specta_collect_commands!();
```

### Note 

If you do not want to have to annotate every command with `#[auto_collect_command]`, you can do this in the `build.rs`.

```rust
    fn main() {
        tauri_helper::generate_command_file(tauri_helper::TauriHelperOptions::new(true));
        tauri_build::build();
    }
```

This will tell the build script to get every tauri_command available in every member of the workspace.

This is not recommended as it can lead to adding functions that are not meant to be exported.

---

If your workspace contains multiple crates, you must export all functions in the root file (`lib.rs`) of each crate.

### Example

In `my_commands.rs`:
```rust
#[tauri::command]
#[auto_collect_command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```

In `lib.rs`:
```rust
pub mod my_commands;
pub use my_commands::*;
```

> **Note:** This is required because the feature that enables full module path retrieval is still only available in the nightly version of Rust.

## Example

Here’s a complete example of using `tauri_helper` in a Tauri application:

```rust
#[tauri::command]
#[auto_collect_command]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
#[auto_collect_command]
fn calculate_sum(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let builder: tauri_specta::Builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(specta_collect_commands!());

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default().bigint(specta_typescript::BigIntExportBehavior::Number),
            "../src/bindings.ts",
        )
        .expect("should work");

    tauri::Builder::default()
        .invoke_handler(tauri_collect_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## Feature Flags

- **`tracing`**: Enables `tracing` support in the `WithLogging` macro. This feature is optional and must be explicitly enabled.

---

## Notes

- **`WithLogging` Stability**: The `WithLogging` macro is experimental and may undergo breaking changes. It is not recommended for production use.
- **Command Collection**: Ensure that all Tauri commands are annotated with `#[auto_collect_command]` to be included in the generated handlers by default.
- **Collection Conflict**: Ensure that you are only using an `#[command]` that comes from tauri.

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or bug fixes.

---

## License

This project is licensed under the [MIT License](LICENSE).

