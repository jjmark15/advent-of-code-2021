use std::num::ParseIntError;

use regex::Regex;

use crate::domain::solution_executor::day_05 as domain;

#[derive(derive_new::new)]
pub(crate) struct Position {
    x: u64,
    y: u64,
}

pub(crate) struct StraightLine {
    start: Position,
    end: Position,
}

impl TryFrom<String> for StraightLine {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)").unwrap();
        let caps = re.captures(value.as_str()).unwrap();

        let x1: u64 = (&caps["x1"]).parse()?;
        let y1: u64 = (&caps["y1"]).parse()?;
        let x2: u64 = (&caps["x2"]).parse()?;
        let y2: u64 = (&caps["y2"]).parse()?;

        let start = Position::new(x1, y1);
        let end = Position::new(x2, y2);

        Ok(StraightLine { start, end })
    }
}

impl From<StraightLine> for domain::StraightLine {
    fn from(from: StraightLine) -> Self {
        domain::StraightLine::new(from.start.into(), from.end.into())
    }
}

impl From<Position> for domain::Position {
    fn from(from: Position) -> Self {
        domain::Position::new(from.x as usize, from.y as usize)
    }
}
