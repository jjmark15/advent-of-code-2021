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
            .check_lines(to_syntax_lines(input))
            .into_iter()
            .filter(|res| res.is_err())
            .map(|res| error_scorer.score(&res.err().unwrap()))
            .sum()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

fn to_syntax_lines(lines: Vec<String>) -> Vec<Vec<SyntaxCharacter>> {
    lines
        .into_iter()
        .map(|line| line.chars().map(SyntaxCharacter::new).collect())
        .collect()
}

#[derive(derive_new::new)]
struct SyntaxChecker;

impl SyntaxChecker {
    fn check_lines(
        &self,
        lines: Vec<Vec<SyntaxCharacter>>,
    ) -> Vec<Result<Vec<SyntaxCharacter>, SyntaxError>> {
        lines
            .into_iter()
            .map(|line| self.check_line(line))
            .collect()
    }

    fn check_line(&self, line: Vec<SyntaxCharacter>) -> Result<Vec<SyntaxCharacter>, SyntaxError> {
        SyntaxWalker::new().walk(line)
    }
}

#[derive(derive_new::new, Copy, Clone, Eq, PartialEq, Debug)]
struct SyntaxCharacter(char);

impl SyntaxCharacter {
    fn is_closed_by(&self, other: Self) -> bool {
        other == self.closing_character()
    }

    fn closing_character(&self) -> Self {
        let character = match self.value() {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => unimplemented!(),
        };
        SyntaxCharacter::new(character)
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
    fn walk(&mut self, syntax: Vec<SyntaxCharacter>) -> Result<Vec<SyntaxCharacter>, SyntaxError> {
        syntax
            .into_iter()
            .fold_while(
                Ok(Vec::new()),
                |acc: Result<Vec<SyntaxCharacter>, SyntaxError>, character: SyntaxCharacter| {
                    let mut bracket_stack = acc.unwrap();
                    if character.closes() {
                        if let Some(previous) = bracket_stack.last() {
                            if previous.is_closed_by(character) {
                                bracket_stack.pop();
                                return Continue(Ok(bracket_stack));
                            }
                        }
                        Done(Err(SyntaxError::new(character)))
                    } else {
                        bracket_stack.push(character);
                        Continue(Ok(bracket_stack))
                    }
                },
            )
            .into_inner()
    }
}

#[derive(Debug, derive_new::new)]
struct SyntaxError(SyntaxCharacter);

#[derive(derive_new::new)]
struct SyntaxErrorScorer;

impl SyntaxErrorScorer {
    fn score(&self, error: &SyntaxError) -> u64 {
        match error.0.value() {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => unimplemented!(),
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
