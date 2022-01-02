use crate::domain::solution_executor::SolutionExecutor;
use std::collections::HashMap;

#[derive(Debug)]
struct DayState {
    timers: HashMap<u8, u64>,
}

impl DayState {
    fn new(timers: Vec<u8>) -> Self {
        let mut map = HashMap::new();
        timers.into_iter().for_each(|timer| {
            let count = map.entry(timer).or_insert(0);
            *count += 1;
        });
        DayState { timers: map }
    }

    fn tick_day(&mut self) {
        let old_map = self.timers.clone();
        self.timers.insert(8, *old_map.get(&0).unwrap_or(&0));
        self.timers.insert(7, *old_map.get(&8).unwrap_or(&0));
        self.timers.insert(
            6,
            *old_map.get(&7).unwrap_or(&0) + *old_map.get(&0).unwrap_or(&0),
        );
        self.timers.insert(5, *old_map.get(&6).unwrap_or(&0));
        self.timers.insert(4, *old_map.get(&5).unwrap_or(&0));
        self.timers.insert(3, *old_map.get(&4).unwrap_or(&0));
        self.timers.insert(2, *old_map.get(&3).unwrap_or(&0));
        self.timers.insert(1, *old_map.get(&2).unwrap_or(&0));
        self.timers.insert(0, *old_map.get(&1).unwrap_or(&0));
    }

    fn fish_count(&self) -> u64 {
        self.timers.values().sum()
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day6SolutionExecutor;

impl SolutionExecutor for Day6SolutionExecutor {
    type Input = Vec<u8>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let mut state = DayState::new(input);
        (0..80).for_each(|_day| state.tick_day());
        state.fish_count()
    }

    fn part_2(&self, input: Self::Input) -> Self::Part2Output {
        let mut state = DayState::new(input);
        (0..256).for_each(|_day| state.tick_day());
        state.fish_count()
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

    #[test]
    fn counts_number_of_fish_after_256_days() {
        assert_that(&Day6SolutionExecutor::new().part_2(test_data())).is_equal_to(26984457539);
    }
}
