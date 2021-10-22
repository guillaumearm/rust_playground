pub mod cell;
pub mod cursor;
pub mod error;
pub mod filler;
pub mod map;

pub type AppResult<T = String> = map::MapResult<T>;

pub fn run() -> AppResult {
    let map: map::Map = "\n\n\n\n".parse()?;
    Ok(map.to_string())
}
