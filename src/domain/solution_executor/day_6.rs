use crate::domain::solution_executor::SolutionExecutor;

#[derive(Debug, derive_new::new)]
struct DayState {
    timers: Vec<u8>,
}

impl DayState {
    fn tick_day(&mut self) {
        let mut new_fish_count = 0;
        self.timers.iter_mut().for_each(|timer| {
            if *timer == 0 {
                new_fish_count += 1;
                *timer = 8;
            } else {
                *timer -= 1;
            }
        });

        self.timers
            .append(&mut (0..new_fish_count).map(|_| 6).collect::<Vec<u8>>());
    }

    fn fish_count(&self) -> usize {
        self.timers.len()
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day6SolutionExecutor;

impl SolutionExecutor for Day6SolutionExecutor {
    type Input = Vec<u8>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let mut state = DayState::new(input);
        (0..80).for_each(|_day| state.tick_day());
        state.fish_count()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<u8> {
        vec![3, 4, 3, 1, 2]
    }

    #[test]
    fn counts_number_of_fish_after_80_days() {
        assert_that(&Day6SolutionExecutor::new().part_1(test_data())).is_equal_to(5934);
    }
}
