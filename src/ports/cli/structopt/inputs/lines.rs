use std::str::FromStr;

use crate::ports::cli::structopt::error::ParseInputError;

#[derive(derive_new::new)]
pub(crate) struct Lines<T> {
    inner: Vec<T>,
}

impl<T> Lines<T> {
    pub(crate) fn inner(self) -> Vec<T> {
        self.inner
    }
}

impl<T> TryFrom<String> for Lines<T>
where
    T: TryFrom<String>,
{
    type Error = ParseInputError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner: Vec<T> = value
            .lines()
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| T::try_from(line.to_string()))
            .map(|result| result.map_err(|_e| ParseInputError::new()))
            .collect::<Result<Vec<T>, Self::Error>>()?;
        Ok(Lines::new(inner))
    }
}

impl<E, T: FromStr<Err = E>> FromStr for Lines<T> {
    type Err = E;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Vec<T> = s
            .lines()
            .into_iter()
            .filter(|line| !line.is_empty())
            .map(|line| T::from_str(line))
            .collect::<Result<Vec<T>, Self::Err>>()?;
        Ok(Lines::new(inner))
    }
}
