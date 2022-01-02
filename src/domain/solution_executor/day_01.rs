use std::collections::VecDeque;

use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day1SolutionExecutor;

impl SolutionExecutor for Day1SolutionExecutor {
    type Input = Vec<u64>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        count_increases(input)
    }

    fn part_2(&self, input: Self::Input) -> Self::Part2Output {
        count_sliding_window_sum_increases(input)
    }
}

fn count_increases(numbers: Vec<u64>) -> u64 {
    let mut increases = 0;
    let mut previous: Option<u64> = None;

    for number in numbers {
        if let Some(previous_number) = previous {
            if number > previous_number {
                increases += 1;
            }
        }
        previous = Some(number);
    }

    increases
}

fn count_sliding_window_sum_increases(numbers: Vec<u64>) -> u64 {
    let mut queue = VecDeque::from(numbers);
    let mut increases = 0;
    let mut previous: Option<u64> = None;

    while queue.len() >= 3 {
        let current: u64 = queue.range(..3).sum();
        queue.pop_front();
        if let Some(previous_sum) = previous {
            if current > previous_sum {
                increases += 1;
            }
        }
        previous = Some(current);
    }

    increases
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn counts_increases() {
        let under_test: Day1SolutionExecutor = Day1SolutionExecutor::new();
        let numbers = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_that(&under_test.part_1(numbers)).is_equal_to(7);
    }

    #[test]
    fn counts_sliding_window_sum_increases() {
        let under_test: Day1SolutionExecutor = Day1SolutionExecutor::new();
        let numbers = vec![607, 618, 618, 617, 647, 716, 769, 792];

        assert_that(&under_test.part_2(numbers)).is_equal_to(5);
    }
}