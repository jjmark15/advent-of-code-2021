use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct PuzzleInput {
    signal_patterns: Vec<String>,
    digit_output_values: Vec<String>,
}

enum DigitValue {
    One,
    _Two,
    _Three,
    Four,
    _Five,
    _Six,
    Seven,
    Eight,
    _Nine,
}

#[derive(derive_new::new)]
struct DigitIdentifier {
    unique_length_identifier: UniqueLengthIdentifier,
}

impl DigitIdentifier {
    fn identify(
        &self,
        _signal_patterns: Vec<String>,
        digit_output_values: Vec<String>,
    ) -> Vec<Option<DigitValue>> {
        self.unique_length_identifier.identify(digit_output_values)
    }
}

#[derive(derive_new::new)]
struct UniqueLengthIdentifier;

impl UniqueLengthIdentifier {
    fn identify(&self, digit_output_values: Vec<String>) -> Vec<Option<DigitValue>> {
        digit_output_values
            .iter()
            .map(|value_signal| match value_signal.len() {
                2 => Some(DigitValue::One),
                4 => Some(DigitValue::Four),
                3 => Some(DigitValue::Seven),
                7 => Some(DigitValue::Eight),
                _ => None,
            })
            .collect()
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day8SolutionExecutor;

impl SolutionExecutor for Day8SolutionExecutor {
    type Input = Vec<PuzzleInput>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let identifier = DigitIdentifier::new(UniqueLengthIdentifier::new());
        input
            .into_iter()
            .map(|puzzle_input| {
                identifier
                    .identify(
                        puzzle_input.signal_patterns,
                        puzzle_input.digit_output_values,
                    )
                    .into_iter()
                    .flatten()
                    .collect::<Vec<DigitValue>>()
            })
            .flatten()
            .count()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<PuzzleInput> {
        vec![
            test_puzzle_input(
                vec![
                    "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                    "fabcd", "edb",
                ],
                vec!["fdgacbe", "cefdb", "cefbgd", "gcbe"],
            ),
            test_puzzle_input(
                vec![
                    "edbfga", "begcd", "cbg", "gc", "gcadebf", "fbgde", "acbgfd", "abcde",
                    "gfcbed", "gfec",
                ],
                vec!["fcgedb", "cgb", "dgebacf", "gc"],
            ),
            test_puzzle_input(
                vec![
                    "fgaebd", "cg", "bdaec", "gdafb", "agbcfd", "gdcbef", "bgcad", "gfac", "gcb",
                    "cdgabef",
                ],
                vec!["cg", "cg", "fdcagb", "cbg"],
            ),
            test_puzzle_input(
                vec![
                    "fbegcd", "cbd", "adcefb", "dageb", "afcb", "bc", "aefdc", "ecdab", "fgdeca",
                    "fcdbega",
                ],
                vec!["efabcd", "cedba", "gadfec", "cb"],
            ),
            test_puzzle_input(
                vec![
                    "aecbfdg", "fbg", "gf", "bafeg", "dbefa", "fcge", "gcbea", "fcaegb", "dgceab",
                    "fcbdga",
                ],
                vec!["gecf", "egdcabf", "bgf", "bfgea"],
            ),
            test_puzzle_input(
                vec![
                    "fgeab", "ca", "afcebg", "bdacfeg", "cfaedg", "gcfdb", "baec", "bfadeg",
                    "bafgc", "acf",
                ],
                vec!["gebdcfa", "ecba", "ca", "fadegcb"],
            ),
            test_puzzle_input(
                vec![
                    "dbcfg", "fgd", "bdegcaf", "fgec", "aegbdf", "ecdfab", "fbedc", "dacgb",
                    "gdcebf", "gf",
                ],
                vec!["cefg", "dcbef", "fcge", "gbcadfe"],
            ),
            test_puzzle_input(
                vec![
                    "bdfegc", "cbegaf", "gecbf", "dfcage", "bdacg", "ed", "bedf", "ced", "adcbefg",
                    "gebcd",
                ],
                vec!["ed", "bcgafe", "cdgba", "cbgef"],
            ),
            test_puzzle_input(
                vec![
                    "egadfb", "cdbfeg", "cegd", "fecab", "cgb", "gbdefca", "cg", "fgcdab", "egfdb",
                    "bfceg",
                ],
                vec!["gbdfcae", "bgc", "cg", "cgb"],
            ),
            test_puzzle_input(
                vec![
                    "gcafb", "gcf", "dcaebfg", "ecagb", "gf", "abcdeg", "gaef", "cafbge", "fdbac",
                    "fegbdc",
                ],
                vec!["fgae", "cfgab", "fg", "bagce"],
            ),
        ]
    }

    fn test_puzzle_input(
        signal_patterns: Vec<&str>,
        digit_output_values: Vec<&str>,
    ) -> PuzzleInput {
        PuzzleInput::new(
            signal_patterns.iter().map(ToString::to_string).collect(),
            digit_output_values
                .iter()
                .map(ToString::to_string)
                .collect(),
        )
    }

    #[test]
    fn counts_easy_digits_in_output_values() {
        assert_that(&Day8SolutionExecutor::new().part_1(test_data())).is_equal_to(26);
    }
}
