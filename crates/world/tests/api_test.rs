#[cfg(feature = "default-impl")]
#[test]
fn world_test() {
    let wld_path = match std::fs::read_to_string("world_path.txt") {
        Ok(val) => Some(val),
        Err(_) => None,
    };
    if wld_path.is_none() {
        println!(
            "Skipping Full Test, Add \"world_path.txt\" next to the Cargo.toml with a world path!"
        );
        return;
    }
}
