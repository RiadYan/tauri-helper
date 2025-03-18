use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
    #[allow(dead_code)]
    pub package: Package,
    pub workspace: Workspace,
}

#[derive(Debug, Deserialize)]
pub struct Package {
    #[allow(dead_code)]
    pub name: String,
    #[allow(dead_code)]
    pub version: String,
    #[allow(dead_code)]
    pub edition: String,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
    pub members: Vec<String>,
}

/// Configuration options for the `tauri_helper` crate.
///
/// This struct allows you to customize the behavior of the command collection process.
#[derive(Deserialize)]
pub struct TauriHelperOptions {
    /// Determines whether to collect all functions annotated with `#[tauri::command]`
    /// regardless of whether they also have the `#[auto_collect_command]` attribute.
    ///
    /// When set to `true`, the crate will scan for and collect all functions marked
    /// with `#[tauri::command]`, even if they do not have the `#[auto_collect_command]` attribute.
    ///
    /// When set to `false` (the default), only functions with both `#[tauri::command]`
    /// and `#[auto_collect_command]` will be collected.
    ///
    /// # Recommendation
    ///
    /// It is strongly recommended to keep this option set to `false` (the default) and
    /// explicitly annotate the functions you want to collect with `#[auto_collect_command]`.
    /// This ensures better control over which commands are included in your Tauri application
    /// and avoids accidentally collecting unintended commands.
    ///
    /// # Example
    ///
    /// ```rust
    /// #[tauri::command]
    /// #[auto_collect_command] // Explicitly opt-in to command collection
    /// fn my_command() {
    ///     println!("Good Opt-In")
    /// }
    /// ```
    ///
    /// Setting `collect_all` to `true` should only be used in specific cases where you
    /// want to automatically collect all `#[tauri::command]` functions without explicit
    /// opt-in. Use this option with caution.
    pub collect_all: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for TauriHelperOptions {
    /// Provides a default configuration for `TauriHelperOptions`.
    ///
    /// By default, `collect_all` is set to `false`, meaning only functions annotated with
    /// both `#[tauri::command]` and `#[auto_collect_command]` will be collected.
    ///
    /// This default behavior is recommended for most use cases to ensure explicit control
    /// over which commands are included in your Tauri application.
    fn default() -> Self {
        Self { collect_all: false }
    }
}

impl TauriHelperOptions {
    pub fn new(collect_all: bool) -> Self {
        Self { collect_all }
    }
}
