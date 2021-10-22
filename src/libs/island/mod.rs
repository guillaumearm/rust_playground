pub mod cell;
pub mod cursor;
pub mod error;
pub mod map;

pub type AppResult = Result<(), Box<dyn std::error::Error>>;

pub fn run() -> AppResult {
    Ok(())
}
