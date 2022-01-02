use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day10SolutionExecutor;

impl SolutionExecutor for Day10SolutionExecutor {
    type Input = Vec<String>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let syntax_checker = SyntaxChecker::new();
        let error_scorer = SyntaxErrorScorer::new();

        syntax_checker
            .find_errors(input)
            .iter()
            .map(|error| error_scorer.score(error))
            .sum()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[derive(derive_new::new)]
struct SyntaxChecker;

impl SyntaxChecker {
    fn find_errors(&self, lines: Vec<String>) -> Vec<SyntaxError> {
        lines
            .into_iter()
            .map(|line| self.find_first_error(line))
            .flatten()
            .collect()
    }

    fn find_first_error(&self, line: String) -> Option<SyntaxError> {
        SyntaxWalker::new().walk(line).err()
    }
}

#[derive(derive_new::new, Copy, Clone)]
struct SyntaxCharacter(char);

impl SyntaxCharacter {
    fn is_closed_by(&self, other: Self) -> bool {
        match self.value() {
            '(' => other.value() == ')',
            '[' => other.value() == ']',
            '{' => other.value() == '}',
            '<' => other.value() == '>',
            _ => unimplemented!(),
        }
    }

    fn closes(&self) -> bool {
        [')', ']', '}', '>'].contains(&self.0)
    }

    fn value(&self) -> char {
        self.0
    }
}

#[derive(derive_new::new)]
struct SyntaxWalker;

impl SyntaxWalker {
    fn walk(&mut self, syntax: String) -> Result<(), SyntaxError> {
        syntax
            .chars()
            .map(SyntaxCharacter::new)
            .fold_while(
                Ok(Vec::new()),
                |acc: Result<Vec<SyntaxCharacter>, SyntaxError>, character: SyntaxCharacter| {
                    let mut bracket_stack = acc.unwrap_or_default();
                    if character.closes() {
                        match bracket_stack.last() {
                            None => Done(Err(SyntaxError::Illegal(character.value()))),
                            Some(previous) if !previous.is_closed_by(character) => {
                                Done(Err(SyntaxError::Illegal(character.value())))
                            }
                            Some(_previous) => {
                                bracket_stack.pop();
                                Continue(Ok(bracket_stack))
                            }
                        }
                    } else {
                        bracket_stack.push(character);
                        Continue(Ok(bracket_stack))
                    }
                },
            )
            .into_inner()
            .map(|_| ())
    }
}

enum SyntaxError {
    _Incomplete,
    Illegal(char),
}

#[derive(derive_new::new)]
struct SyntaxErrorScorer;

impl SyntaxErrorScorer {
    fn score(&self, error: &SyntaxError) -> u64 {
        match error {
            SyntaxError::_Incomplete => 0,
            SyntaxError::Illegal(character) => match character {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unimplemented!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .into_iter()
        .map(ToString::to_string)
        .collect()
    }

    #[test]
    fn calculates_total_illegal_syntax_error_score() {
        assert_that(&Day10SolutionExecutor::new().part_1(test_data())).is_equal_to(26397)
    }
}
