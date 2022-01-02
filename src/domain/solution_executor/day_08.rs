use crate::domain::solution_executor::SolutionExecutor;
use std::collections::{HashMap, HashSet};

#[derive(derive_new::new)]
pub(crate) struct PuzzleInput {
    signal_patterns: Vec<String>,
    digit_output_values: Vec<String>,
}

impl PuzzleInput {
    fn signal_patterns(&self) -> Vec<HashSet<char>> {
        self.signal_patterns
            .iter()
            .map(|string| string.chars().collect::<HashSet<char>>())
            .collect()
    }

    fn digit_segments(&self) -> Vec<HashSet<char>> {
        self.digit_output_values
            .iter()
            .map(|string| string.chars().collect::<HashSet<char>>())
            .collect()
    }
}

#[derive(Debug)]
struct IdentificationContext {
    map: HashMap<Digit, HashSet<char>>,
    _signal_patterns: Vec<HashSet<char>>,
    digit_output_values: Vec<HashSet<char>>,
}

impl IdentificationContext {
    fn new(signal_patterns: Vec<HashSet<char>>, digit_output_values: Vec<HashSet<char>>) -> Self {
        IdentificationContext {
            map: HashMap::new(),
            _signal_patterns: signal_patterns,
            digit_output_values,
        }
    }

    fn set_digit_segments(&mut self, digit: Digit, segments: HashSet<char>) {
        self.map.insert(digit, segments);
    }

    fn count_occurrences_of_digit_segments(&self, digit: &Digit) -> usize {
        if let Some(segments) = self.map.get(digit) {
            self.digit_output_values
                .iter()
                .filter(|&output_segments| output_segments == segments)
                .count()
        } else {
            0
        }
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Digit {
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
struct IdentifierChain {
    context: IdentificationContext,
}

impl IdentifierChain {
    fn context(self) -> IdentificationContext {
        self.context
    }

    fn chain<I: DigitIdentifier>(mut self, identifier: I) -> Self {
        identifier.identify(&mut self.context);
        self
    }
}

trait DigitIdentifier {
    fn identify(&self, context: &mut IdentificationContext);
}

#[derive(derive_new::new)]
struct UniqueLengthIdentifier;

impl DigitIdentifier for UniqueLengthIdentifier {
    fn identify(&self, context: &mut IdentificationContext) {
        context
            .digit_output_values
            .clone()
            .into_iter()
            .map(|segments| match segments.len() {
                2 => Some((Digit::One, segments)),
                4 => Some((Digit::Four, segments)),
                3 => Some((Digit::Seven, segments)),
                7 => Some((Digit::Eight, segments)),
                _ => None,
            })
            .flatten()
            .for_each(|(digit, segments)| {
                context.set_digit_segments(digit, segments);
            });
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day8SolutionExecutor;

impl SolutionExecutor for Day8SolutionExecutor {
    type Input = Vec<PuzzleInput>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        input
            .into_iter()
            .map(|puzzle_input| {
                let context = IdentifierChain::new(IdentificationContext::new(
                    puzzle_input.signal_patterns(),
                    puzzle_input.digit_segments(),
                ))
                .chain(UniqueLengthIdentifier::new())
                .context();

                vec![Digit::One, Digit::Four, Digit::Seven, Digit::Eight]
                    .into_iter()
                    .map(|digit| context.count_occurrences_of_digit_segments(&digit))
                    .sum::<usize>()
            })
            .sum()
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
