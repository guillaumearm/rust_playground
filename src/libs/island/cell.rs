use std::convert::{From, TryFrom};
use std::fmt;

/*
 * Constants
 */
mod constants {
    pub const SEA_CHAR: char = ' ';
    pub const EARTH_CHAR: char = '#';
    pub const RADIX_BASE: u32 = 36;
}

pub use constants::*;

/*
 * Errors
 */
pub mod error {
    use super::*;

    // Parsing error Display
    #[derive(PartialEq)]
    pub struct Parse(pub char);

    impl fmt::Display for Parse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let Parse(c) = *self;
            write!(f, "cell parse error for character '{}'", c)
        }
    }

    // Increment error Display
    #[derive(PartialEq)]
    pub enum Increment {
        Overflow { cell: Cell },
        NotValidCell { cell: Cell },
    }

    impl fmt::Display for Increment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match *self {
            Increment::Overflow { cell } => write!(f, "cannot increment the cell '{}' because this is the maximum allowed character for marked earth", char::from(cell)),
            Increment::NotValidCell {cell} => write!(f, "cannot increment the cell '{}' because only Cell::MarkedEarth can be incremented", char::from(cell))
        }
        }
    }

    // Debug + Error impl
    impl fmt::Debug for Increment {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    impl fmt::Debug for Parse {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }
    impl std::error::Error for Parse {}
    impl std::error::Error for Increment {}
}

/*
 * Utils
 */
mod utils {
    use super::*;

    pub fn is_valid_cell_char(c: char) -> bool {
        match c {
            _ if c.is_digit(RADIX_BASE) => true,
            SEA_CHAR => true,
            EARTH_CHAR => true,
            _ => false,
        }
    }

    pub fn increment_char(c: char) -> Option<char> {
        let n = c.to_digit(RADIX_BASE)?;
        char::from_digit(n + 1, RADIX_BASE)
    }
}

/*
 * Cell
 */
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Sea,
    Earth,
    MarkedEarth(char),
}

impl Cell {
    pub fn is_markable(&self) -> bool {
        match self {
            Cell::Earth => true,
            _ => false,
        }
    }

    pub fn is_marked(&self) -> bool {
        match self {
            Cell::MarkedEarth(_) => true,
            _ => false,
        }
    }

    pub fn increment(&mut self) -> Result<(), error::Increment> {
        let cell = *self;
        match *self {
            Cell::MarkedEarth(c) => {
                let next_char =
                    utils::increment_char(c).ok_or(error::Increment::Overflow { cell })?;

                *self = Cell::MarkedEarth(next_char);
                Ok(())
            }
            _ => Err(error::Increment::NotValidCell { cell }),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = error::Parse;

    fn try_from(c: char) -> Result<Cell, error::Parse> {
        match c {
            SEA_CHAR => Ok(Cell::Sea),
            EARTH_CHAR => Ok(Cell::Earth),
            _ if utils::is_valid_cell_char(c) => Ok(Cell::MarkedEarth(c)),
            _ => Err(error::Parse(c)),
        }
    }
}

impl From<Cell> for char {
    fn from(cell: Cell) -> Self {
        match cell {
            Cell::Sea => SEA_CHAR,
            Cell::Earth => EARTH_CHAR,
            Cell::MarkedEarth(c) => c,
        }
    }
}

/*
 * Unit tests
 */
#[cfg(test)]
mod tests {
    use super::*;

    mod constants {
        use super::*;

        #[test]
        fn radix_base() {
            assert!(RADIX_BASE > 0 && RADIX_BASE < 37);
        }

        #[test]
        fn sea_constant_is_not_a_digit() {
            assert!(!SEA_CHAR.is_digit(RADIX_BASE));
        }

        #[test]
        fn earth_constant_is_not_a_digit() {
            assert!(!EARTH_CHAR.is_digit(RADIX_BASE));
        }
    }

    mod conversion {
        use super::*;

        // Conversion
        #[test]
        fn sea_cell_to_char() -> Result<(), error::Parse> {
            assert_eq!(Cell::try_from(SEA_CHAR)?, Cell::Sea);
            Ok(())
        }

        #[test]
        fn earth_cell_to_char() -> Result<(), error::Parse> {
            assert_eq!(Cell::try_from(EARTH_CHAR)?, Cell::Earth);
            Ok(())
        }

        #[test]
        fn marked_earth_cell_to_char() -> Result<(), error::Parse> {
            assert_eq!(Cell::try_from('1')?, Cell::MarkedEarth('1'));
            Ok(())
        }

        #[test]
        fn sea_char_to_cell() {
            assert_eq!(char::from(Cell::Sea), SEA_CHAR);
        }

        #[test]
        fn earth_char_to_cell() {
            assert_eq!(char::from(Cell::Earth), EARTH_CHAR);
        }

        #[test]
        fn marked_earth_char_to_cell() {
            assert_eq!(char::from(Cell::MarkedEarth('0')), '0');
        }

        // Conversion error
        #[test]
        fn invalid_char_to_cell() {
            let invalid_char = '%';
            let res = Cell::try_from(invalid_char);
            assert_eq!(res, Err(error::Parse(invalid_char)));
        }
    }

    mod methods {
        use super::*;

        // Cell::is_markable method
        #[test]
        fn is_markable() {
            assert_eq!(true, Cell::Earth.is_markable());
            assert_eq!(false, Cell::Sea.is_markable());
            assert_eq!(false, Cell::MarkedEarth('0').is_markable());
        }

        mod increment {
            use super::*;

            #[test]
            fn zero_cell() {
                let mut cell = Cell::MarkedEarth('0');
                let result = cell.increment();
                assert_eq!(result, Ok(()));
                assert_eq!(cell, Cell::MarkedEarth('1'));
            }

            #[test]
            fn all() {
                let mut cell = Cell::MarkedEarth('0');
                let mut counter = 1;

                while let Ok(()) = cell.increment() {
                    counter += 1;
                }

                assert_eq!(counter, RADIX_BASE);
            }

            // Increment errors
            #[test]
            fn overflow() {
                let last_char = char::from_digit(RADIX_BASE - 1, RADIX_BASE).unwrap();
                let mut cell = Cell::MarkedEarth(last_char);

                assert_eq!(cell.increment(), Err(error::Increment::Overflow { cell }));
            }

            #[test]
            fn sea() {
                let mut cell = Cell::Sea;

                assert_eq!(
                    cell.increment(),
                    Err(error::Increment::NotValidCell { cell })
                )
            }

            #[test]
            fn earth() {
                let mut cell = Cell::Earth;

                assert_eq!(
                    cell.increment(),
                    Err(error::Increment::NotValidCell { cell })
                )
            }
        }
    }
}
