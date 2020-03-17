use std::{error, fmt, str::FromStr};

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub enum Tag {
    Version,
    SortOrder,
    GroupOrder,
    SubsortOrder,
    Other(String),
}

#[derive(Debug)]
pub struct ParseError(String);

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid header tag: '{}'", self.0)
    }
}

impl FromStr for Tag {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "VN" => Ok(Self::Version),
            "SO" => Ok(Self::SortOrder),
            "GO" => Ok(Self::GroupOrder),
            "SS" => Ok(Self::SubsortOrder),
            _ => {
                if s.len() == 2 {
                    Ok(Self::Other(s.into()))
                } else {
                    Err(ParseError(s.into()))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() -> Result<(), ParseError> {
        assert_eq!("VN".parse::<Tag>()?, Tag::Version);
        assert_eq!("SO".parse::<Tag>()?, Tag::SortOrder);
        assert_eq!("GO".parse::<Tag>()?, Tag::GroupOrder);
        assert_eq!("SS".parse::<Tag>()?, Tag::SubsortOrder);

        assert_eq!("ND".parse::<Tag>()?, Tag::Other(String::from("ND")));

        assert!("".parse::<Tag>().is_err());
        assert!("NDL".parse::<Tag>().is_err());

        Ok(())
    }
}