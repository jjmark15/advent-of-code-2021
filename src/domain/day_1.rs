use std::collections::VecDeque;

#[derive(derive_new::new)]
pub(crate) struct Day1SolutionExecutor;

impl Day1SolutionExecutor {
    pub(crate) fn part_1(&self, input: Vec<u64>) -> u64 {
        count_increases(input)
    }

    pub(crate) fn part_2(&self, input: Vec<u64>) -> u64 {
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
    use super::*;
    use speculoos::prelude::*;

    #[test]
    fn counts_increases() {
        let under_test: Day1SolutionExecutor = Day1SolutionExecutor::new();
        let numbers = vec![1, 2, 3, 4];

        assert_that(&under_test.part_1(numbers)).is_equal_to(3);
    }

    #[test]
    fn counts_sliding_window_sum_increases() {
        let under_test: Day1SolutionExecutor = Day1SolutionExecutor::new();
        let numbers = vec![607, 618, 618, 617, 647, 716, 769, 792];

        assert_that(&under_test.part_2(numbers)).is_equal_to(5);
    }
}
