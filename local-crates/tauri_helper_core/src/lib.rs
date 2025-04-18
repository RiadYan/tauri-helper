pub mod types;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use types::CargoToml;

pub fn find_workspace_dir(start_dir: &Path) -> PathBuf {
    let mut current_dir = start_dir.to_path_buf();
    loop {
        if current_dir.join("Cargo.toml").exists() {
            if let Ok(contents) = fs::read_to_string(current_dir.join("Cargo.toml")) {
                if contents.contains("[workspace]") {
                    return current_dir;
                }
            }
        }
        if !current_dir.pop() {
            panic!("Workspace root not found from {}", start_dir.display());
        }
    }
}

pub fn get_workspace_members(workspace_root: &Path) -> Vec<String> {
    let cargo_toml = workspace_root.join("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml).unwrap_or_else(|_| {
        panic!(
            "Failed to read workspace Cargo.toml at {}",
            cargo_toml.display()
        );
    });

    let toml_content: CargoToml = toml::from_str(&contents).unwrap();

    toml_content.workspace.members
}

pub fn get_workspace() -> CargoToml {
    let workspace_root = find_workspace_dir(Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()));

    let cargo_toml = workspace_root.join("Cargo.toml");
    let toml_contents = fs::read_to_string(&cargo_toml).unwrap_or_else(|_| {
        panic!(
            "Failed to read workspace Cargo.toml at {}",
            cargo_toml.display()
        );
    });

    let toml_content: CargoToml = toml::from_str(&toml_contents).unwrap();
    toml_content
}

pub fn get_workspace_pkg_name() -> String {
    let cont = get_workspace();
    cont.package.name
}
