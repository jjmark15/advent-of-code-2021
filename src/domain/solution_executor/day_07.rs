use crate::domain::solution_executor::SolutionExecutor;

trait FuelCalculator {
    fn calculate(&self, current_position: &u64, target_position: &u64) -> u64;
}

#[derive(derive_new::new)]
struct SimpleFuelCalculator;

impl FuelCalculator for SimpleFuelCalculator {
    fn calculate(&self, current_position: &u64, target_position: &u64) -> u64 {
        current_position.max(target_position) - current_position.min(target_position)
    }
}

#[derive(derive_new::new)]
struct AccurateFuelCalculator;

impl FuelCalculator for AccurateFuelCalculator {
    fn calculate(&self, current_position: &u64, target_position: &u64) -> u64 {
        let distance =
            current_position.max(target_position) - current_position.min(target_position);
        (1..distance + 1).sum()
    }
}

#[derive(derive_new::new)]
struct OptimumUsageFinder<FC: FuelCalculator> {
    fuel_calculator: FC,
}

impl<FC: FuelCalculator> OptimumUsageFinder<FC> {
    fn find_lowest(&self, starting_positions: Vec<u64>) -> u64 {
        let lower_bound_position = starting_positions.iter().min().unwrap();
        let upper_bound_position = starting_positions.iter().max().unwrap();

        (*lower_bound_position..*upper_bound_position + 1)
            .fold(None, |lowest_fuel, target_position| {
                let total_fuel: u64 = starting_positions
                    .iter()
                    .map(|starting_position| {
                        self.fuel_calculator
                            .calculate(starting_position, &target_position)
                    })
                    .sum();
                Some(lowest_fuel.unwrap_or(total_fuel).min(total_fuel))
            })
            .unwrap()
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day7SolutionExecutor;

impl SolutionExecutor for Day7SolutionExecutor {
    type Input = Vec<u64>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        OptimumUsageFinder::new(SimpleFuelCalculator::new()).find_lowest(input)
    }

    fn part_2(&self, input: Self::Input) -> Self::Part2Output {
        OptimumUsageFinder::new(AccurateFuelCalculator::new()).find_lowest(input)
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

    #[test]
    fn calculates_least_amount_of_fuel_to_align_at_common_position_with_increasing_fuel_cost() {
        assert_that(&Day7SolutionExecutor::new().part_2(test_data())).is_equal_to(168);
    }
}
