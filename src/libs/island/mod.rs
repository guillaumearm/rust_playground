use self::filler::Filler;

mod cell;
mod cursor;
mod error;
mod filler;
mod map;

pub use error::*;

/// The result returned by [`run`]
pub type AppResult<T = String> = map::MapResult<T>;

/// Apply the count_island logic on a raw map and get a new raw map wrapped in a [`AppResult`]
pub fn run(raw_map: &str) -> AppResult {
    let filler = Filler::new(raw_map.parse()?);
    let map = &filler.map;
    let cursor = map.cursor();

    let marked_cursor = cursor.iter().filter(|c| c.read().is_marked());
    let markable_cursors = cursor.iter().filter(|c| c.read().is_markable());

    for cursor in marked_cursor {
        map.write(cursor.coord(), cell::Cell::Earth);
    }

    for cursor in markable_cursors {
        if filler.fill(cursor).is_none() {
            break;
        }
    }

    Ok(map.to_string())
}
