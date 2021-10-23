use std::fmt;

#[derive(PartialEq)]
pub enum Error {
    EmptyMap,
    EmptyLine(usize),
    InvalidChar { char: char, line: usize, col: usize },
}

use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EmptyMap => write!(f, "map is empty"),
            EmptyLine(line) => write!(f, "empty line found at line '{}'", line),
            InvalidChar { char, line, col } => write!(
                f,
                "invalid character '{}' found at position {}:{}",
                char, line, col
            ),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

#[derive(PartialEq)]
pub struct ErrorList(pub Vec<Error>);

impl fmt::Display for ErrorList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ErrorList(errors) = self;

        let errors: Vec<String> = errors.iter().map(|e| format!("{}", e)).collect();

        write!(f, "{}", errors.join("\n"))
    }
}

impl fmt::Debug for ErrorList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<Vec<Error>> for ErrorList {
    fn from(errors: Vec<Error>) -> Self {
        ErrorList(errors)
    }
}

impl std::error::Error for ErrorList {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_trait() {
        let err = EmptyMap;
        assert!(format!("{}", err).contains("empty"));

        let err = EmptyLine(42);
        assert!(format!("{}", err).contains("42"));

        let err = InvalidChar {
            char: 'z',
            line: 21,
            col: 42,
        };
        assert!(format!("{}", err).contains("21:42"));
    }

    #[test]
    fn dyn_error_conversion() {
        let mut _dynerr: Box<dyn std::error::Error>;

        _dynerr = EmptyMap.into();
        _dynerr = EmptyLine(0).into();
        _dynerr = InvalidChar {
            char: 'a',
            line: 1,
            col: 2,
        }
        .into();
    }
}
