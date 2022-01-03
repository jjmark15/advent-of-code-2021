use std::str::FromStr;

#[derive(Debug)]
pub(crate) struct CommaSeparatedList<T> {
    inner: Vec<T>,
}

impl<T> CommaSeparatedList<T> {
    pub(crate) fn inner(self) -> Vec<T> {
        self.inner
    }
}

impl<E, T: TryFrom<String, Error = E>> TryFrom<String> for CommaSeparatedList<T> {
    type Error = E;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner = value
            .split(',')
            .map(|val| val.to_string().try_into())
            .collect::<Result<Vec<T>, Self::Error>>();

        Ok(CommaSeparatedList { inner: inner? })
    }
}

impl<E, T: FromStr<Err = E>> FromStr for CommaSeparatedList<T> {
    type Err = E;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner: Result<Vec<T>, Self::Err> = s.split(',').map(FromStr::from_str).collect();
        Ok(CommaSeparatedList { inner: inner? })
    }
}
