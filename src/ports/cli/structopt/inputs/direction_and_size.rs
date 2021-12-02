use std::num::ParseIntError;

use regex::Regex;

#[derive(derive_getters::Getters)]
pub(crate) struct DirectionAndSize {
    direction: Direction,
    size: u64,
}

pub(crate) enum Direction {
    Forward,
    Down,
    Up,
}

impl TryFrom<String> for Direction {
    type Error = ParseDirectionAndSizeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            "forward" => Ok(Direction::Forward),
            _ => Err(ParseDirectionAndSizeError::new(value)),
        }
    }
}

impl TryFrom<String> for DirectionAndSize {
    type Error = ParseDirectionAndSizeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?P<direction>\S+) (?P<size>\d+)").unwrap();
        let caps = re.captures(value.as_str()).unwrap();

        let direction: Result<Direction, ParseDirectionAndSizeError> =
            (&caps["direction"]).to_string().try_into();

        let size: Result<u64, ParseIntError> = (&caps["size"]).parse();

        Ok(DirectionAndSize {
            direction: direction?,
            size: size.map_err(|_e| ParseDirectionAndSizeError::new(value))?,
        })
    }
}

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("could not parse direction and size from {0}")]
pub(crate) struct ParseDirectionAndSizeError(String);

impl From<DirectionAndSize> for crate::domain::solution_executor::day_2::DirectionAndSize {
    fn from(from: DirectionAndSize) -> Self {
        crate::domain::solution_executor::day_2::DirectionAndSize::new(
            from.direction.into(),
            from.size,
        )
    }
}

impl From<Direction> for crate::domain::solution_executor::day_2::Direction {
    fn from(from: Direction) -> Self {
        match from {
            Direction::Forward => crate::domain::solution_executor::day_2::Direction::Forward,
            Direction::Up => crate::domain::solution_executor::day_2::Direction::Up,
            Direction::Down => crate::domain::solution_executor::day_2::Direction::Down,
        }
    }
}
