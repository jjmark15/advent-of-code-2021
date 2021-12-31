use std::collections::HashMap;

use crate::domain::solution_executor::SolutionExecutor;

#[derive(Debug, derive_new::new, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, derive_new::new)]
pub(crate) struct StraightLine {
    start: Position,
    end: Position,
}

impl StraightLine {
    fn points(&self) -> Vec<Position> {
        if self.start.x == self.end.x {
            potentially_descending_end_inclusive_range(self.start.y, self.end.y)
                .map(|y| Position::new(self.start.x, y))
                .collect()
        } else if self.start.y == self.end.y {
            potentially_descending_end_inclusive_range(self.start.x, self.end.x)
                .map(|x| Position::new(x, self.start.y))
                .collect()
        } else {
            potentially_descending_end_inclusive_range(self.start.x, self.end.x)
                .zip(potentially_descending_end_inclusive_range(
                    self.start.y,
                    self.end.y,
                ))
                .map(|(x, y)| Position::new(x, y))
                .collect()
        }
    }

    fn is_diagonal(&self) -> bool {
        !(self.start.x == self.end.x || self.start.y == self.end.y)
    }
}

fn potentially_descending_end_inclusive_range(
    start: usize,
    end: usize,
) -> impl Iterator<Item = usize> {
    if start > end {
        (end..start + 1).rev().collect::<Vec<usize>>().into_iter()
    } else {
        (start..end + 1).collect::<Vec<usize>>().into_iter()
    }
}

#[derive(Debug)]
struct VentMap {
    map: HashMap<Position, usize>,
}

impl VentMap {
    fn new() -> Self {
        VentMap {
            map: HashMap::new(),
        }
    }

    fn point_vent_counts(&self) -> Vec<usize> {
        self.map.values().into_iter().copied().collect()
    }

    fn record_line(&mut self, line: &StraightLine) {
        line.points().into_iter().for_each(|point| {
            let count = self.map.entry(point).or_insert(0);
            *count += 1;
        });
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day5SolutionExecutor;

impl SolutionExecutor for Day5SolutionExecutor {
    type Input = Vec<StraightLine>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let mut vent_map = VentMap::new();
        input
            .iter()
            .filter(|line| !line.is_diagonal())
            .for_each(|line| vent_map.record_line(line));
        vent_map
            .point_vent_counts()
            .into_iter()
            .filter(|&count| count >= 2)
            .count()
    }

    fn part_2(&self, input: Self::Input) -> Self::Part2Output {
        let mut vent_map = VentMap::new();
        input.iter().for_each(|line| vent_map.record_line(line));
        vent_map
            .point_vent_counts()
            .into_iter()
            .filter(|&count| count >= 2)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<StraightLine> {
        vec![
            StraightLine::new(Position::new(0, 9), Position::new(5, 9)),
            StraightLine::new(Position::new(8, 0), Position::new(0, 8)),
            StraightLine::new(Position::new(9, 4), Position::new(3, 4)),
            StraightLine::new(Position::new(2, 2), Position::new(2, 1)),
            StraightLine::new(Position::new(7, 0), Position::new(7, 4)),
            StraightLine::new(Position::new(6, 4), Position::new(2, 0)),
            StraightLine::new(Position::new(0, 9), Position::new(2, 9)),
            StraightLine::new(Position::new(3, 4), Position::new(1, 4)),
            StraightLine::new(Position::new(0, 0), Position::new(8, 8)),
            StraightLine::new(Position::new(5, 5), Position::new(8, 2)),
        ]
    }

    #[test]
    fn counts_points_where_at_least_two_non_diagonal_lines_intersect() {
        assert_that(&Day5SolutionExecutor::new().part_1(test_data())).is_equal_to(5)
    }

    #[test]
    fn counts_points_where_at_least_two_lines_intersect() {
        assert_that(&Day5SolutionExecutor::new().part_2(test_data())).is_equal_to(12)
    }
}
