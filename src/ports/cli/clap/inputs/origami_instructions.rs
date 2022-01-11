use std::num::ParseIntError;

use regex::Regex;

use crate::domain::solution_executor::day_13 as domain;
use crate::domain::solution_executor::day_13::{Coordinate, Fold};
use crate::ports::cli::clap::inputs::line_groups::LineGroups;

pub(crate) struct OrigamiInstructions {
    points: Vec<domain::Coordinate>,
    folds: Vec<domain::Fold>,
}

impl TryFrom<String> for OrigamiInstructions {
    type Error = ParseIntError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut groups = value.line_groups();
        let folds = parse_folds(groups.pop().expect("points not in input"))?;
        let points = parse_coordinates(groups.pop().unwrap())?;
        Ok(OrigamiInstructions { points, folds })
    }
}

fn parse_coordinates(lines: Vec<String>) -> Result<Vec<domain::Coordinate>, ParseIntError> {
    lines
        .into_iter()
        .map(|line| parse_coordinate(line.split_once(",").expect("could not split by comma")))
        .collect()
}

fn parse_coordinate((x, y): (&str, &str)) -> Result<domain::Coordinate, ParseIntError> {
    Ok(domain::Coordinate::new(x.parse()?, y.parse()?))
}

fn parse_folds(lines: Vec<String>) -> Result<Vec<domain::Fold>, ParseIntError> {
    let re = Regex::new(r"fold along (?P<axis>[xy])=(?P<value>\d+)").unwrap();

    lines
        .into_iter()
        .map(|line| {
            let caps = re.captures(line.as_str()).unwrap();
            let value: usize = (&caps["value"]).parse()?;
            let axis = match &caps["axis"] {
                "x" => domain::FoldAxis::X,
                "y" => domain::FoldAxis::Y,
                _ => unimplemented!(),
            };
            Ok(domain::Fold::new(axis, value))
        })
        .collect()
}

impl From<OrigamiInstructions> for (Vec<Coordinate>, Vec<Fold>) {
    fn from(from: OrigamiInstructions) -> Self {
        (from.points, from.folds)
    }
}
