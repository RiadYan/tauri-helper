use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use syn::parse_file;
use tauri_helper_core::{find_workspace_dir, get_workspace, get_workspace_members};
use walkdir::WalkDir;

pub use tauri_helper_core::types::TauriHelperOptions;
pub use tauri_helper_macros::*;

#[allow(clippy::needless_doctest_main)]
/// Scans the crate for functions annotated with `#[tauri::command]` and optionally `#[auto_collect_command]`,
/// then generates a file containing a list of these functions in the `tauri_commands_list` folder.
///
/// This function is intended to be used in a `build.rs` script to automate the process of
/// collecting Tauri commands during the build process. It should be called before invoking
/// `tauri_build::build()` to ensure the command list is available for the Tauri application.
///
/// # Usage
///
/// Add the following to your `build.rs` file:
///
/// ```rust
/// fn main() {
///     // Generate the command file for Tauri
///     tauri_helper::generate_command_file(tauri_helper::TauriHelperOptions::default());
///
///     // Build the Tauri application
///     tauri_build::build();
/// }
/// ```
///
/// # Annotations
///
/// By default, this function looks for functions annotated with both `#[tauri::command]` and
/// `#[auto_collect_command]`. For example:
///
/// ```rust
/// #[tauri::command]
/// #[auto_collect_command]
/// fn my_command() {
///     println!("Some Command")
/// }
/// ```
///
/// These functions will be automatically collected and written to the `tauri_commands_list` folder.
///
/// If the `collect_all` option is set to `true`, the function will collect all `#[tauri::command]`
/// functions, regardless of whether they have the `#[auto_collect_command]` attribute. However,
/// this behavior is not recommended unless explicitly needed.
///
/// # Output
///
/// The generated file will be placed in the `tauri_commands_list` folder (relative to the crate root) inside of the target folder.
/// The file will contain a list of all collected commands, which can be used by the Tauri application
/// to register commands.
///
/// # Options
///
/// The behavior of this function can be customized using the `TauriHelperOptions` struct:
///
/// - **`collect_all`**: When `true`, collects all `#[tauri::command]` functions, even if they lack
///   the `#[auto_collect_command]` attribute. When `false` (default), only functions with both
///   `#[tauri::command]` and `#[auto_collect_command]` are collected.
///
///   **Recommendation**: Keep this option set to `false` to ensure explicit control over which
///   commands are included in your Tauri application.
///
/// # Notes
///
/// - This function should only be called once per build, typically in the `build.rs` script.
/// - More options are coming such as a list of explicit files that need to be scanned only, if you have any more ideas, please open an issue on Github.
///
/// # Example
///
/// ```rust
/// #[tauri::command]
/// #[auto_collect_command]
/// fn greet(name: String) -> String {
///     format!("Hello, {}!", name)
/// }
///
/// #[tauri::command]
/// fn calculate_sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// ```
///
/// With `collect_all` set to `false` (default), only `greet` will be collected. With `collect_all`
/// set to `true`, both `greet` and `calculate_sum` will be collected.
///
/// # Panics
///
/// This function will panic if:
/// - The `tauri_commands_list` folder cannot be created or written to.
/// - No functions matching the criteria are found.
///
/// # Errors
///
/// If the function encounters an error during file generation, it will log the error and exit the
/// build process with a non-zero status code.
pub fn generate_command_file() {
    let workspace_root = find_workspace_dir(Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()));
    let commands_dir = workspace_root.join("target").join("commands");
    fs::create_dir_all(&commands_dir).unwrap();

    // Read the workspace members from `Cargo.toml`
    let workspace_members = get_workspace_members(&workspace_root);
    for member in &workspace_members {
        println!("cargo:rerun-if-changed={}", member);
    }

    for member in workspace_members {
        let manifest_dir = workspace_root.join(&member);
        let crate_name = manifest_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();

        let mut functions = Vec::new();

        // Scan all Rust files in the crate's src directory
        let src_dir = manifest_dir.join("src");
        for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(path) {
                    if let Ok(ast) = parse_file(&content) {
                        for item in ast.items {
                            if let syn::Item::Fn(func) = item {
                                for attr in &func.attrs {
                                    if attr.path().is_ident("auto_collect_command") {
                                        functions.push(func.sig.ident.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let package_name = get_workspace().package.name.replace("-", "_");
        // Write to the crate's command file
        let command_file = commands_dir.join(format!("{}.txt", crate_name));
        let mut file = File::create(&command_file).unwrap();

        for func in functions {
            if crate_name.replace("-", "_") == "src_tauri" {
                let full_name = format!("{}::{}", package_name, func);
                println!("found: {:#?}", &full_name);
                writeln!(file, "{}", full_name).unwrap();
            } else {
                let full_name = format!("{}::{}", crate_name.replace("-", "_"), func);
                println!("found: {:#?}", &full_name);
                writeln!(file, "{}", full_name).unwrap();
            }
        }
    }
}
