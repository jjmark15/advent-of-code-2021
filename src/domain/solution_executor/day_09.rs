use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day9SolutionExecutor;

impl SolutionExecutor for Day9SolutionExecutor {
    type Input = Vec<Vec<u8>>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let height_map = HeightMap::new(to_locations(input));
        height_map
            .low_points()
            .into_iter()
            .map(Location::risk_level)
            .sum()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[derive(derive_new::new)]
struct HeightMap {
    inner: Vec<Vec<Location>>,
}

impl HeightMap {
    fn low_points(&self) -> Vec<&Location> {
        self.inner
            .iter()
            .enumerate()
            .map(|(row_index, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(col_index, _location)| {
                        self.is_low_point(Position::new(row_index, *col_index))
                    })
                    .map(|(_, location)| location)
                    .collect::<Vec<&Location>>()
            })
            .flatten()
            .collect()
    }

    fn is_low_point(&self, position: Position) -> bool {
        let height = self
            .at_position(position)
            .expect("position does not exist")
            .height();

        let neighbour_heights = self.neighbours(position).into_iter().map(Location::height);

        height < neighbour_heights.min().unwrap()
    }

    fn at_position(&self, position: Position) -> Option<&Location> {
        if let Some(row) = self.inner.get(position.row) {
            if let Some(location) = row.get(position.col) {
                return Some(location);
            }
        }
        None
    }

    fn neighbours(&self, position: Position) -> Vec<&Location> {
        position
            .adjacent()
            .into_iter()
            .map(|p| self.at_position(p))
            .flatten()
            .collect()
    }
}

#[derive(derive_new::new, Copy, Clone, Debug)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn adjacent(&self) -> Vec<Position> {
        vec![self.above(), self.below(), self.left(), self.right()]
            .into_iter()
            .flatten()
            .collect()
    }

    fn above(&self) -> Option<Position> {
        if let Some(row) = self.row.checked_add(1) {
            return Some(Position::new(row, self.col));
        }
        None
    }

    fn below(&self) -> Option<Position> {
        if let Some(row) = self.row.checked_sub(1) {
            return Some(Position::new(row, self.col));
        }
        None
    }

    fn left(&self) -> Option<Position> {
        if let Some(col) = self.col.checked_sub(1) {
            return Some(Position::new(self.row, col));
        }
        None
    }

    fn right(&self) -> Option<Position> {
        if let Some(col) = self.col.checked_add(1) {
            return Some(Position::new(self.row, col));
        }
        None
    }
}

#[derive(derive_new::new)]
struct Location {
    height: u8,
}

impl Location {
    fn risk_level(&self) -> usize {
        self.height as usize + 1
    }

    fn height(&self) -> u8 {
        self.height
    }
}

fn to_locations(heights: Vec<Vec<u8>>) -> Vec<Vec<Location>> {
    heights
        .into_iter()
        .map(|line| line.into_iter().map(Location::new).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<Vec<u8>> {
        vec![
            vec![2, 1, 9, 9, 9, 4, 3, 2, 1, 0],
            vec![3, 9, 8, 7, 8, 9, 4, 9, 2, 1],
            vec![9, 8, 5, 6, 7, 8, 9, 8, 9, 2],
            vec![8, 7, 6, 7, 8, 9, 6, 7, 8, 9],
            vec![9, 8, 9, 9, 9, 6, 5, 6, 7, 8],
        ]
    }

    #[test]
    fn calculates_sum_of_low_point_risk_levels() {
        assert_that(&Day9SolutionExecutor::new().part_1(test_data())).is_equal_to(15);
    }
}
