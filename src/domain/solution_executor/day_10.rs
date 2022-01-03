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

        to_syntax_lines(input)
            .into_iter()
            .map(|line| syntax_checker.check_line(line))
            .map(|res| res.err())
            .flatten()
            .map(|error| error_scorer.score(&error))
            .sum()
    }

    fn part_2(&self, input: Self::Input) -> Self::Part2Output {
        let syntax_checker = SyntaxChecker::new();
        let completion_scorer = SyntaxCompletionScorer::new();

        let completion_scores: Vec<u64> = to_syntax_lines(input)
            .into_iter()
            .filter(|line| syntax_checker.check_line(line.clone()).is_ok())
            .map(|line| syntax_checker.complete_incomplete_line(line))
            .map(|completion| completion_scorer.score(completion))
            .sorted()
            .collect();

        *completion_scores.get(completion_scores.len() / 2).unwrap() as u64
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
    fn check_line(&self, line: Vec<SyntaxCharacter>) -> Result<(), SyntaxError> {
        SyntaxWalker::new().walk(line)
    }

    fn complete_incomplete_line(&self, line: Vec<SyntaxCharacter>) -> Vec<SyntaxCharacter> {
        let mut walker = SyntaxWalker::new();
        walker.walk(line).unwrap();
        walker.suggest_completion()
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

struct SyntaxWalker {
    syntax_stack: Vec<SyntaxCharacter>,
}

impl SyntaxWalker {
    fn new() -> Self {
        SyntaxWalker {
            syntax_stack: Vec::new(),
        }
    }

    fn walk(&mut self, syntax: Vec<SyntaxCharacter>) -> Result<(), SyntaxError> {
        let syntax_error = syntax
            .into_iter()
            .fold_while(
                None,
                |_syntax_error: Option<SyntaxError>, character: SyntaxCharacter| {
                    if character.closes() {
                        if let Some(previous) = self.syntax_stack.last() {
                            if previous.is_closed_by(character) {
                                self.syntax_stack.pop();
                                return Continue(None);
                            }
                        }
                        Done(Some(SyntaxError::new(character)))
                    } else {
                        self.syntax_stack.push(character);
                        Continue(None)
                    }
                },
            )
            .into_inner();

        match syntax_error {
            None => Ok(()),
            Some(error) => Err(error),
        }
    }

    fn suggest_completion(&self) -> Vec<SyntaxCharacter> {
        self.syntax_stack
            .iter()
            .rev()
            .map(|closer| closer.closing_character())
            .collect()
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

#[derive(derive_new::new)]
struct SyntaxCompletionScorer;

impl SyntaxCompletionScorer {
    fn score(&self, characters: Vec<SyntaxCharacter>) -> u64 {
        characters.into_iter().fold(0_u64, |score, character| {
            score * 5 + Self::character_score(character)
        })
    }

    fn character_score(character: SyntaxCharacter) -> u64 {
        match character.value() {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
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

    #[test]
    fn calculates_completion_score() {
        assert_that(&Day10SolutionExecutor::new().part_2(test_data())).is_equal_to(288957)
    }
}
