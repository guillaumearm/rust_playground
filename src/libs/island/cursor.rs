use super::cell::Cell;
use super::map::Map;

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::*;

pub const DIRECTIONS: [Direction; 4] = [Up, Right, Down, Left];

#[derive(Debug, Copy, Clone)]
pub struct Cursor<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(map: &'a Map) -> Cursor<'a> {
        Cursor { map, x: 0, y: 0 }
    }

    pub fn read(&self) -> Cell {
        let Cursor { map, x, y } = *self;
        return map
            .get(x, y)
            .expect(format!("[FATAL] invalid cursor read at x:{} y:{}", x, y).as_str());
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cursor<'a>> {
        let Cursor { map, .. } = self;
        map.get(x, y).and(Some(Cursor { map, x, y }))
    }

    pub fn move_next_line(&self) -> Option<Cursor<'a>> {
        self.get(0, self.y + 1)
    }

    pub fn move_dir(&self, dir: Direction) -> Option<Cursor<'a>> {
        let Cursor { x, y, .. } = *self;
        match dir {
            Up => self.get(x, y - 1),
            Down => self.get(x, y + 1),
            Left => self.get(x - 1, y),
            Right => self.get(x + 1, y),
        }
    }

    pub fn iter(&self) -> CursorIter<'a> {
        CursorIter::new(*self)
    }
}

pub struct CursorIter<'a> {
    cursor: Cursor<'a>,
    nextable: bool,
}

impl<'a> CursorIter<'a> {
    fn new(cursor: Cursor<'a>) -> CursorIter<'a> {
        CursorIter {
            cursor,
            nextable: true,
        }
    }
}

impl<'a> Iterator for CursorIter<'a> {
    type Item = Cursor<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.nextable {
            return None;
        }
        let cursor = self.cursor;

        if let Some(next_cursor) = cursor.move_dir(Right).or(cursor.move_next_line()) {
            self.cursor = next_cursor;
        } else {
            self.nextable = false;
        }

        Some(cursor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Cell::*;

    #[test]
    fn cursor_iterator() {
        let map: Map = "# \n0".parse().unwrap();
        let cells: Vec<Cell> = map.cursor().iter().map(|c| c.read()).collect();

        assert_eq!(cells, vec![Earth, Sea, MarkedEarth('0')]);
    }
}
