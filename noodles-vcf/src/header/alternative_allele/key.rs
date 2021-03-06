use std::{error, fmt, str::FromStr};

/// A VCF header symbolic alternate allele record key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Key {
    /// (`ID`).
    Id,
    /// (`Description`).
    Description,
}

impl AsRef<str> for Key {
    fn as_ref(&self) -> &str {
        match self {
            Self::Id => "ID",
            Self::Description => "Description",
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_ref())
    }
}

/// An error returned when a raw VCF header symbolic alternate allele record key fails to parse.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseError(String);

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid filter key: expected {{ID, Description}}, got {}",
            self.0
        )
    }
}

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ID" => Ok(Self::Id),
            "Description" => Ok(Self::Description),
            _ => Err(ParseError(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        assert_eq!(Key::Id.to_string(), "ID");
        assert_eq!(Key::Description.to_string(), "Description");
    }

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        assert_eq!("ID".parse::<Key>()?, Key::Id);
        assert_eq!("Description".parse::<Key>()?, Key::Description);

        assert!("".parse::<Key>().is_err());
        assert!("Noodles".parse::<Key>().is_err());

        Ok(())
    }
}
