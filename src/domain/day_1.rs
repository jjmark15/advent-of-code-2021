#[derive(derive_new::new)]
pub(crate) struct Day1SolutionExecutor;

impl Day1SolutionExecutor {
    pub(crate) fn part_1(&self, input: Vec<u64>) -> u64 {
        count_increases(input)
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

#[cfg(test)]
mod tests {
    use super::*;
    use speculoos::prelude::*;

    #[test]
    fn records_increases() {
        let under_test: Day1SolutionExecutor = Day1SolutionExecutor::new();
        let numbers = vec![1, 2, 3, 4];

        assert_that(&under_test.part_1(numbers)).is_equal_to(3);
    }
}
