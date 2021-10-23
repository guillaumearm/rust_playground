use garm_playground::libs::island;
use std::env;
use std::fs;

/// # Usage: `cargo run <filepath>`
///
/// You may find usable maps in the `misc/count_island` directory of the project.
///
/// usage: `cargo run misc/count_island/complex_map.txt`
fn main() -> Result<(), Box<dyn std::error::Error>> {
    const USAGE: &str = "Usage: cargo run <filepath>";

    let args: Vec<String> = env::args().skip(1).collect();
    let filepath = args.get(0).ok_or(USAGE)?;

    let raw_map = fs::read_to_string(filepath)?;

    let updated_raw_map = island::run(&raw_map)?;
    print!("{}", updated_raw_map);

    Ok(())
}
