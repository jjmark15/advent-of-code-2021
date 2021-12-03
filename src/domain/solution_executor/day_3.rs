use crate::domain::solution_executor::SolutionExecutor;

#[derive(Debug)]
struct BinaryFrequencyCounter {
    state: Vec<usize>,
    list_length: usize,
}

impl BinaryFrequencyCounter {
    fn new(number_length: usize, list_length: usize) -> Self {
        BinaryFrequencyCounter {
            state: vec![0; number_length],
            list_length,
        }
    }

    fn record_number(&mut self, binary_number: &str) {
        binary_number.chars().enumerate().for_each(|(i, digit)| {
            if digit == '1' {
                self.state[i] += 1;
            }
        });
    }

    fn most_frequent(&self) -> String {
        self.state
            .iter()
            .map(|count| {
                if *count > self.list_length / 2 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect()
    }
}

fn gamma_rate(binary_numbers: Vec<String>) -> String {
    let number_length = binary_numbers.first().map_or(0, |number| number.len());
    let mut binary_frequency_counter: BinaryFrequencyCounter =
        BinaryFrequencyCounter::new(number_length, binary_numbers.len());
    binary_numbers
        .into_iter()
        .for_each(|binary_number| binary_frequency_counter.record_number(&binary_number));
    binary_frequency_counter.most_frequent()
}

fn epsilon_rate(gamma_rate: impl AsRef<str>) -> String {
    gamma_rate
        .as_ref()
        .chars()
        .map(|digit| if digit == '1' { '0' } else { '1' })
        .collect()
}

fn to_decimal(binary_number: impl AsRef<str>) -> usize {
    usize::from_str_radix(binary_number.as_ref(), 2).unwrap()
}

#[derive(derive_new::new)]
pub(crate) struct Day3SolutionExecutor;

impl SolutionExecutor for Day3SolutionExecutor {
    type Input = Vec<String>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let gamma_rate = gamma_rate(input);
        let epsilon_rate = epsilon_rate(&gamma_rate);
        to_decimal(gamma_rate) * to_decimal(epsilon_rate)
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn calculates_power_rate() {
        let binary_numbers: Vec<String> = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .into_iter()
        .map(String::from)
        .collect();
        let under_test: Day3SolutionExecutor = Day3SolutionExecutor::new();

        assert_that(&under_test.part_1(binary_numbers)).is_equal_to(198);
    }
}
