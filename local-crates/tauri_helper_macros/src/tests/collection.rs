use crate::collection::normalize_commands;

#[test]
fn strips_prefix_for_calling_crate_only() {
    let raw = vec![
        "tauri_helper::local_cmd".to_string(),
        "other_crate::foreign_cmd".to_string(),
    ];

    let result = normalize_commands(raw, "tauri_helper".into());
    let collected: Vec<_> = result.into_iter().collect();
    println!("hi: {:#?}", collected);
    assert_eq!(collected, vec!["local_cmd", "other_crate::foreign_cmd"]);
}

#[test]
fn preserves_prefix_when_calling_crate_differs() {
    let raw = vec!["tauri_helper::cmd".to_string()];

    let result = normalize_commands(raw, "another_crate".into());
    let collected: Vec<_> = result.into_iter().collect();

    assert_eq!(collected, vec!["tauri_helper::cmd"]);
}

#[test]
fn sorts_and_deduplicates_commands() {
    let raw = vec![
        "b_cmd".to_string(),
        "a_cmd".to_string(),
        "a_cmd".to_string(),
        "c_cmd".to_string(),
    ];

    let result = normalize_commands(raw, "tauri_helper".into());
    let collected: Vec<_> = result.into_iter().collect();

    assert_eq!(collected, vec!["a_cmd", "b_cmd", "c_cmd",]);
}

#[test]
fn accepts_valid_identifiers_and_paths() {
    let raw = vec![
        "valid_cmd".to_string(),
        "crate_name::valid_cmd".to_string(),
        "a1_b2::c3".to_string(),
    ];

    let result = normalize_commands(raw, "tauri_helper".into());

    assert_eq!(result.len(), 3);
}

#[test]
fn panics_on_invalid_command_name() {
    let raw = vec!["valid_cmd".to_string(), "invalid-cmd".to_string()];

    let result = std::panic::catch_unwind(|| {
        normalize_commands(raw, "tauri_helper".into());
    });

    assert!(result.is_err());
}
