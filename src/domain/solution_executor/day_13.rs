use std::cmp::Ordering;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

use itertools::Itertools;

use crate::domain::solution_executor::SolutionExecutor;

#[derive(Debug)]
pub(crate) enum FoldAxis {
    X,
    Y,
}

#[derive(derive_new::new, Debug)]
pub(crate) struct Fold {
    axis: FoldAxis,
    value: usize,
}

#[derive(derive_new::new, Debug, Hash, Eq, PartialEq)]
pub(crate) struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct OrigamiPaper {
    visible_dots: HashSet<Coordinate>,
    dimensions: Coordinate,
}

impl OrigamiPaper {
    fn new(visible_dots: HashSet<Coordinate>) -> Self {
        let dimensions = Self::dimensions(&visible_dots);
        OrigamiPaper {
            visible_dots,
            dimensions,
        }
    }

    fn dimensions(visible_dots: &HashSet<Coordinate>) -> Coordinate {
        visible_dots
            .iter()
            .fold(Coordinate::new(0, 0), |acc, point| {
                Coordinate::new(acc.x.max(point.x), acc.y.max(point.y))
            })
    }

    fn fold(&mut self, fold: &Fold) {
        self.visible_dots = self
            .visible_dots
            .drain()
            .into_iter()
            .filter_map(|point| match fold.axis {
                FoldAxis::X => match point.x.cmp(&fold.value) {
                    Ordering::Greater => {
                        Some(Coordinate::new(self.dimensions.x - point.x, point.y))
                    }
                    Ordering::Less => Some(point),
                    Ordering::Equal => None,
                },
                FoldAxis::Y => match point.y.cmp(&fold.value) {
                    Ordering::Greater => {
                        Some(Coordinate::new(point.x, self.dimensions.y - point.y))
                    }
                    Ordering::Less => Some(point),
                    Ordering::Equal => None,
                },
            })
            .collect();

        match fold.axis {
            FoldAxis::X => {
                self.dimensions = Coordinate::new((self.dimensions.x - 1) / 2, self.dimensions.y);
            }
            FoldAxis::Y => {
                self.dimensions = Coordinate::new(self.dimensions.x, (self.dimensions.y - 1) / 2);
            }
        }
    }

    fn visible_dots(&self) -> usize {
        self.visible_dots.len()
    }
}

impl Display for OrigamiPaper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.dimensions.y + 1)
                .into_iter()
                .map(|y| {
                    (0..self.dimensions.x + 1)
                        .into_iter()
                        .map(|x| {
                            let c = Coordinate::new(x, y);
                            if self.visible_dots.contains(&c) {
                                "#".to_string()
                            } else {
                                ".".to_string()
                            }
                        })
                        .join("")
                })
                .join("\n")
        )
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day13SolutionExecutor;

impl SolutionExecutor for Day13SolutionExecutor {
    type Input = (Vec<Coordinate>, Vec<Fold>);
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, (points, folds): Self::Input) -> Self::Part1Output {
        let mut paper = OrigamiPaper::new(HashSet::from_iter(points));
        folds.iter().take(1).for_each(|fold| paper.fold(fold));
        paper.visible_dots()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> (Vec<Coordinate>, Vec<Fold>) {
        let coordinates = vec![
            Coordinate::new(6, 10),
            Coordinate::new(0, 14),
            Coordinate::new(9, 10),
            Coordinate::new(0, 3),
            Coordinate::new(10, 4),
            Coordinate::new(4, 11),
            Coordinate::new(6, 0),
            Coordinate::new(6, 12),
            Coordinate::new(4, 1),
            Coordinate::new(0, 13),
            Coordinate::new(10, 12),
            Coordinate::new(3, 4),
            Coordinate::new(3, 0),
            Coordinate::new(8, 4),
            Coordinate::new(1, 10),
            Coordinate::new(2, 14),
            Coordinate::new(8, 10),
            Coordinate::new(9, 0),
        ];
        let folds = vec![Fold::new(FoldAxis::Y, 7), Fold::new(FoldAxis::X, 5)];
        (coordinates, folds)
    }

    #[test]
    fn counts_visible_dots_after_first_fold() {
        assert_that(&Day13SolutionExecutor::new().part_1(test_data())).is_equal_to(17);
    }
}
