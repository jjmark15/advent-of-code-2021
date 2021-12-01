use std::str::FromStr;

#[derive(Debug, Clone)]
pub(crate) enum DayPart {
    One,
    Two,
}

impl FromStr for DayPart {
    type Err = ParseDayPartError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .parse::<u8>()
            .map_err(|_e| ParseDayPartError::new(s.to_string()))?
        {
            1 => Ok(DayPart::One),
            2 => Ok(DayPart::Two),
            _ => Err(ParseDayPartError::new(s.to_string())),
        }
    }
}

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("could not parse part {0}")]
pub(crate) struct ParseDayPartError(String);
