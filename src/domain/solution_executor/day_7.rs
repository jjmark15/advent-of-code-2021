use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day7SolutionExecutor;

fn required_fuel(start: &u64, end: &u64) -> u64 {
    start.max(end) - start.min(end)
}

#[derive(derive_new::new)]
struct FuelCalculator;

impl FuelCalculator {
    fn find_lowest(&self, starting_positions: Vec<u64>) -> u64 {
        let lower_bound_position = starting_positions.iter().min().unwrap();
        let upper_bound_position = starting_positions.iter().max().unwrap();
        let mut lowest_fuel = upper_bound_position * starting_positions.len() as u64;

        (*lower_bound_position..*upper_bound_position + 1).for_each(|target_position| {
            let total_fuel = starting_positions
                .iter()
                .map(|starting_position| required_fuel(starting_position, &target_position))
                .sum();
            if total_fuel < lowest_fuel {
                lowest_fuel = total_fuel;
            }
        });

        lowest_fuel
    }
}

impl SolutionExecutor for Day7SolutionExecutor {
    type Input = Vec<u64>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        FuelCalculator::new().find_lowest(input)
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<u64> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn calculates_least_amount_of_fuel_to_align_at_common_position() {
        assert_that(&Day7SolutionExecutor::new().part_1(test_data())).is_equal_to(37);
    }
}
