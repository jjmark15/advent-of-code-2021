use crate::domain::solution_executor::day_8::PuzzleInput;
use crate::ports::cli::structopt::error::ParseInputError;

#[derive(Debug)]
pub(crate) struct SubmarineDisplaySignal {
    inner: Vec<Vec<String>>,
}

impl TryFrom<String> for SubmarineDisplaySignal {
    type Error = ParseInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner: Vec<Vec<String>> = value
            .split('|')
            .map(|section| {
                section
                    .trim()
                    .split(' ')
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
            })
            .collect();
        Ok(SubmarineDisplaySignal { inner })
    }
}

impl From<SubmarineDisplaySignal> for PuzzleInput {
    fn from(from: SubmarineDisplaySignal) -> Self {
        PuzzleInput::new(
            from.inner
                .first()
                .expect("does not contain signal patterns")
                .clone(),
            from.inner
                .get(1)
                .expect("does not contain digit output values")
                .clone(),
        )
    }
}
