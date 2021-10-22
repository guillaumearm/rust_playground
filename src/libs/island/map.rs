use super::cell::Cell;
use super::cursor::Cursor;
use super::error::Error;
use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

type MapData = Vec<Vec<Cell>>;

#[derive(Debug)]
pub struct Map(MapData);

type MapResult<T = Map> = Result<T, Vec<Error>>;

impl Map {
    pub fn get(self: &Self, x: usize, y: usize) -> Option<Cell> {
        let Map(data) = self;
        return data.get(y)?.get(x).map(|cell| *cell);
    }

    pub fn cursor(self: &Self) -> Cursor {
        Cursor::new(self)
    }
}

impl FromStr for Map {
    type Err = Vec<Error>;

    fn from_str(s: &str) -> MapResult {
        if s.len() == 0 {
            return Err(vec![Error::EmptyMap]);
        }

        let mut errors: Vec<Error> = vec![];

        let map_data: MapData = s
            .lines()
            .enumerate()
            .map(|(index, line)| {
                let line_number = index + 1;

                if line.len() == 0 {
                    errors.push(Error::EmptyLine(line_number))
                }

                line.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        // on parse error: notify InvalidChar errors and replace map cell by Sea
                        Cell::try_from(c).unwrap_or_else(|_err| {
                            errors.push(Error::InvalidChar {
                                char: c,
                                line: line_number,
                                col: i + 1,
                            });
                            return Cell::Sea;
                        })
                    })
                    .collect()
            })
            .collect();

        if errors.len() == 0 {
            Ok(Map(map_data))
        } else {
            Err(errors)
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Map(data) = self;
        let raw_map: String = data
            .iter()
            .map(|line| {
                line.iter()
                    .map(|cell| char::from(*cell))
                    .collect::<String>()
                    + "\n"
            })
            .collect();

        write!(f, "{}", raw_map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RAW_VALID_MAP: &str = "\
#  
###
 # 
# 0
";

    mod parse {
        use super::*;
        use Cell::*;

        #[test]
        fn valid_map() -> MapResult<()> {
            let Map(map_data) = RAW_VALID_MAP.parse()?;
            assert_eq!(
                map_data,
                vec![
                    vec![Earth, Sea, Sea],
                    vec![Earth, Earth, Earth],
                    vec![Sea, Earth, Sea],
                    vec![Earth, Sea, MarkedEarth('0')],
                ]
            );
            Ok(())
        }

        #[test]
        fn map_empty() {
            let result: MapResult = "".parse();
            let errors = result.unwrap_err();

            assert_eq!(errors, vec![Error::EmptyMap]);
        }

        #[test]
        fn map_empty_lines() {
            let raw_map = "\n123\n\n456\n\n\n789";

            let result: MapResult = raw_map.parse();

            assert_eq!(
                result.unwrap_err(),
                vec![
                    Error::EmptyLine(1),
                    Error::EmptyLine(3),
                    Error::EmptyLine(5),
                    Error::EmptyLine(6)
                ]
            );
        }

        #[test]
        fn map_invalid_character() {
            let raw_map = "@23\n45!\n7[9";

            let result: MapResult = raw_map.parse();

            assert_eq!(
                result.unwrap_err(),
                vec![
                    Error::InvalidChar {
                        char: '@',
                        col: 1,
                        line: 1
                    },
                    Error::InvalidChar {
                        char: '!',
                        col: 3,
                        line: 2
                    },
                    Error::InvalidChar {
                        char: '[',
                        col: 2,
                        line: 3
                    },
                ]
            );
        }

        #[test]
        fn map_mixed_errors() {
            let raw_map = "\n@\n\n";

            let result: MapResult = raw_map.parse();

            assert_eq!(
                result.unwrap_err(),
                vec![
                    Error::EmptyLine(1),
                    Error::InvalidChar {
                        char: '@',
                        line: 2,
                        col: 1
                    },
                    Error::EmptyLine(3),
                ]
            )
        }
    }

    mod display {
        use super::*;

        #[test]
        fn format_with_endline() {
            let raw_map = "123\n";
            let map: Map = raw_map.parse().unwrap();
            assert_eq!(raw_map, format!("{}", map));
        }

        #[test]
        fn format_without_endline() {
            let raw_map = "123".to_string();
            let map: Map = raw_map.parse().unwrap();

            // Note: \n is added
            assert_eq!(raw_map + "\n", format!("{}", map));
        }

        #[test]
        fn format_with_several_lines() {
            let map: Map = RAW_VALID_MAP.parse().unwrap();
            assert_eq!(RAW_VALID_MAP, format!("{}", map));
        }
    }

    mod get {
        use super::*;
        use Cell::*;

        #[test]
        fn valid_coord() {
            let map: Map = RAW_VALID_MAP.parse().unwrap();

            // entire line 1
            assert_eq!(map.get(0, 0), Some(Earth));
            assert_eq!(map.get(1, 0), Some(Sea));
            assert_eq!(map.get(2, 0), Some(Sea));

            // line 2 and 3
            assert_eq!(map.get(0, 1), Some(Earth));
            assert_eq!(map.get(0, 2), Some(Sea));

            // entire line 3
            assert_eq!(map.get(0, 3), Some(Earth));
            assert_eq!(map.get(1, 3), Some(Sea));
            assert_eq!(map.get(2, 3), Some(MarkedEarth('0')));
        }

        #[test]
        fn invalid_coord() {
            let map: Map = RAW_VALID_MAP.parse().unwrap();

            assert_eq!(map.get(3, 0), None);
            assert_eq!(map.get(0, 4), None);
            assert_eq!(map.get(3, 3), None);
            assert_eq!(map.get(42, 42), None);
        }
    }
}
