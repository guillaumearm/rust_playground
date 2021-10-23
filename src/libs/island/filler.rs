use crate::libs::island::{
    // cell::constants::RADIX_BASE,
    cell::*,
    cursor::Cursor,
    cursor::DIRECTIONS,
    map::Map,
};

use Cell::*;

// TODO errors: replace Option type with Result
pub struct Filler {
    pub map: Map,
    cell: std::cell::Cell<Option<Cell>>,
}

impl Filler {
    pub fn new(map: Map) -> Self {
        let cell = Some(MarkedEarth('0'));
        Filler {
            map,
            cell: std::cell::Cell::new(cell),
        }
    }

    pub fn fill(&self, cursor: Cursor) -> Option<()> {
        let cell = self.cell.get();
        cell.and_then(|_| self.fill_cells(Some(cursor)))?;

        let mut cell = cell?;
        let cell = cell.increment().and(Ok(cell)).ok();

        self.cell.set(cell);

        Some(())
    }

    fn fill_cells(&self, cursor: Option<Cursor>) -> Option<()> {
        let cursor = cursor?;
        let cell = self.cell.get()?;

        if !self.map.get(cursor.coord())?.is_markable() {
            return None;
        }

        self.map.write(cursor.coord(), cell);

        for dir in DIRECTIONS {
            let next_coord = self.map.cursor().get(cursor.coord())?.move_dir(dir);
            self.fill_cells(next_coord);
        }

        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_MAP: &str = "\
###
# #
###
";

    const RESOLVED_SIMPLE_MAP: &str = "\
000
0 0
000
";

    const COMPLEX_MAP: &str = "\
######
#    #
# ## #
# ## #
#    #
######
";

    const RESOLVED_COMPLEX_MAP: &str = "\
000000
0    0
0 11 0
0 11 0
0    0
000000
";

    const ALL_ISLAND: &str = "\
# # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
";

    const RESOLVED_ALL_ISLAND: &str = "\
0 1 2 3 4 5 6 7 8 9 a b c d e f g h i j k l m n o p q r s t u v w x y z #
";

    #[test]
    fn fill_simple_one() {
        let map: Map = "#\n".parse().unwrap();
        let filler = Filler::new(map);
        let cursor = filler.map.cursor();

        assert_eq!("#\n", filler.map.to_string());
        assert_eq!(Some(()), filler.fill(cursor.get((0, 0)).unwrap()));
        assert_eq!("0\n", filler.map.to_string());
    }

    #[test]
    fn fill_simple() {
        let map: Map = SIMPLE_MAP.parse().unwrap();
        let filler = Filler::new(map);
        let cursor = filler.map.cursor();

        assert_eq!(SIMPLE_MAP, filler.map.to_string());
        assert_eq!(Some(()), filler.fill(cursor.get((0, 0)).unwrap()));
        assert_eq!(RESOLVED_SIMPLE_MAP, filler.map.to_string());
    }

    #[test]
    fn fill_multiple() {
        let map: Map = COMPLEX_MAP.parse().unwrap();
        let filler = Filler::new(map);
        let cursor = filler.map.cursor();

        assert_eq!(COMPLEX_MAP, filler.map.to_string());
        assert_eq!(Some(()), filler.fill(cursor.get((0, 0)).unwrap()));
        assert_eq!(Some(()), filler.fill(cursor.get((2, 2)).unwrap()));
        assert_eq!(RESOLVED_COMPLEX_MAP, filler.map.to_string());
    }

    #[test]
    fn fill_sea() {
        let map: Map = COMPLEX_MAP.parse().unwrap();
        let filler = Filler::new(map);
        let cursor = filler.map.cursor();

        assert_eq!(COMPLEX_MAP, filler.map.to_string());
        assert_eq!(None, filler.fill(cursor.get((1, 1)).unwrap()));
        assert_eq!(Some(()), filler.fill(cursor.get((0, 0)).unwrap()));
        assert_eq!(None, filler.fill(cursor.get((1, 1)).unwrap()));
        assert_eq!(Some(()), filler.fill(cursor.get((2, 2)).unwrap()));
        assert_eq!(RESOLVED_COMPLEX_MAP, filler.map.to_string());
    }

    #[test]
    fn fill_marked_earth() {
        let map: Map = COMPLEX_MAP.parse().unwrap();
        let filler = Filler::new(map);
        let cursor = filler.map.cursor();

        assert_eq!(COMPLEX_MAP, filler.map.to_string());
        assert_eq!(Some(()), filler.fill(cursor.get((0, 0)).unwrap()));
        assert_eq!(None, filler.fill(cursor.get((0, 0)).unwrap()));
        assert_eq!(Some(()), filler.fill(cursor.get((2, 2)).unwrap()));
        assert_eq!(None, filler.fill(cursor.get((2, 2)).unwrap()));
        assert_eq!(RESOLVED_COMPLEX_MAP, filler.map.to_string());
    }

    #[test]
    fn fill_overflow() {
        let map: Map = ALL_ISLAND.parse().unwrap();
        let filler = Filler::new(map);
        let cursor = filler.map.cursor();

        assert_eq!(ALL_ISLAND, filler.map.to_string());
        for x in (0..RADIX_BASE * 2).step_by(2) {
            let x = x.try_into().unwrap();
            assert_eq!(Some(()), filler.fill(cursor.get((x, 0)).unwrap()));
        }

        let x = ((RADIX_BASE) * 2).try_into().unwrap();
        assert_eq!(None, filler.fill(cursor.get((x, 0)).unwrap()));
        assert_eq!(RESOLVED_ALL_ISLAND, filler.map.to_string());
    }
}
