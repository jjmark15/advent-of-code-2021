use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day11SolutionExecutor;

impl SolutionExecutor for Day11SolutionExecutor {
    type Input = Vec<Vec<u8>>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let mut energy_map = to_octopus_map(input);
        let mut total_flashes = 0;
        (0..100).for_each(|_i| {
            energy_map.increment_energy_levels();
            total_flashes += energy_map.count_flashing();
            energy_map.reset_flashing();
        });
        total_flashes
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[derive(derive_new::new, derive_getters::Getters, Copy, Clone, Debug)]
struct MapPosition {
    row: usize,
    col: usize,
}

impl MapPosition {
    pub(crate) fn adjacent(&self) -> Vec<MapPosition> {
        vec![
            self.above(),
            self.above().and_then(|pos| pos.left()),
            self.above().and_then(|pos| pos.right()),
            self.left(),
            self.right(),
            self.below(),
            self.below().and_then(|pos| pos.left()),
            self.below().and_then(|pos| pos.right()),
        ]
        .into_iter()
        .flatten()
        .filter(MapPosition::is_valid)
        .collect()
    }

    fn above(&self) -> Option<MapPosition> {
        self.row
            .checked_add(1)
            .map(|row| MapPosition::new(row, self.col))
    }

    fn below(&self) -> Option<MapPosition> {
        self.row
            .checked_sub(1)
            .map(|row| MapPosition::new(row, self.col))
    }

    fn left(&self) -> Option<MapPosition> {
        self.col
            .checked_sub(1)
            .map(|col| MapPosition::new(self.row, col))
    }

    fn right(&self) -> Option<MapPosition> {
        self.col
            .checked_add(1)
            .map(|col| MapPosition::new(self.row, col))
    }

    fn is_valid(&self) -> bool {
        self.row < 10 && self.col < 10
    }
}

#[derive(Debug, derive_new::new)]
struct OctopusEnergyMap {
    inner: Vec<Vec<Octopus>>,
}

impl OctopusEnergyMap {
    fn at_position(&mut self, position: MapPosition) -> &mut Octopus {
        self.inner
            .get_mut(position.row)
            .expect("row does not exist")
            .get_mut(position.col)
            .expect("col does not exist")
    }

    fn all_positions() -> Vec<MapPosition> {
        (0..10)
            .map(|row| {
                (0..10)
                    .map(|col| MapPosition::new(row, col))
                    .collect::<Vec<MapPosition>>()
            })
            .flatten()
            .collect()
    }

    fn increment_energy_levels(&mut self) {
        let mut to_be_raised: Vec<MapPosition> = Self::all_positions();

        while let Some(position) = to_be_raised.pop() {
            let octopus = self.at_position(position);
            let previously_flashing = octopus.flashing();
            octopus.raise_energy();

            if !previously_flashing && octopus.flashing() {
                to_be_raised.append(&mut position.adjacent());
            }
        }
    }

    fn reset_flashing(&mut self) {
        self.inner.iter_mut().for_each(|row| {
            row.iter_mut()
                .for_each(|octopus| octopus.reset_if_flashing())
        })
    }

    fn count_flashing(&self) -> usize {
        self.inner
            .iter()
            .map(|row| row.iter().filter(|octopus| octopus.flashing()))
            .flatten()
            .count()
    }
}

#[derive(Debug, derive_new::new)]
struct Octopus {
    energy_level: u8,
}

impl Octopus {
    fn flashing(&self) -> bool {
        self.energy_level > 9
    }

    fn raise_energy(&mut self) {
        if self.energy_level <= 9 {
            self.energy_level += 1;
        }
    }

    fn reset_if_flashing(&mut self) {
        if self.flashing() {
            self.energy_level = 0;
        }
    }
}

fn to_octopus_map(map: Vec<Vec<u8>>) -> OctopusEnergyMap {
    OctopusEnergyMap::new(
        map.into_iter()
            .map(|row| row.into_iter().map(Octopus::new).collect())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<Vec<u8>> {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    }

    #[test]
    fn counts_flashes_in_100_steps() {
        assert_that(&Day11SolutionExecutor::new().part_1(test_data())).is_equal_to(1656);
    }
}
