use std::{collections::BTreeSet, env, fs, path::Path};
use tauri_helper_core::{find_workspace_dir, get_workspace_pkg_name};

pub(crate) fn discover_commands() -> Vec<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let workspace_root = find_workspace_dir(Path::new(&manifest_dir));
    let commands_dir = workspace_root.join("target").join("tauri_commands_list");

    let mut commands = Vec::new();

    let entries = match fs::read_dir(&commands_dir) {
        Ok(e) => e,
        Err(_) => {
            eprintln!(
                "Warning: No commands directory found at {}",
                commands_dir.display()
            );
            return commands;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_file()
            && path.extension().and_then(|e| e.to_str()) == Some("txt")
            && let Ok(content) = fs::read_to_string(&path)
        {
            commands.extend(
                content
                    .lines()
                    .map(str::trim)
                    .filter(|l| !l.is_empty())
                    .map(String::from),
            );
        }
    }

    commands
}

pub(crate) fn normalize_commands(
    raw_commands: Vec<String>,
    calling_crate: String,
) -> BTreeSet<String> {
    let crate_name = get_workspace_pkg_name().replace("-", "_");
    let calling_crate = calling_crate.replace("-", "_");

    let mut commands = BTreeSet::new();

    for mut fn_name in raw_commands {
        if crate_name == calling_crate
            && let Some(stripped) = fn_name.strip_prefix(&format!("{crate_name}::"))
        {
            fn_name = stripped.to_string();
        }

        if fn_name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == ':')
        {
            commands.insert(fn_name);
        } else {
            panic!("Invalid function name `{}` in command file", fn_name);
        }
    }

    commands
}

/// Collects all Tauri commands from the workspace's command files
pub(crate) fn collect_commands(calling_crate: String) -> BTreeSet<String> {
    let raw = discover_commands();
    normalize_commands(raw, calling_crate)
}
