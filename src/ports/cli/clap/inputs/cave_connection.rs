use regex::Regex;

pub(crate) struct CaveConnection {
    start: String,
    end: String,
}

impl TryFrom<String> for CaveConnection {
    type Error = ParseCaveConnectionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?P<start>\S+)-(?P<end>\S+)").unwrap();
        let caps = re
            .captures(value.as_str())
            .ok_or_else(|| ParseCaveConnectionError::new(value.clone()))?;

        let start: String = (&caps["start"]).to_string();
        let end: String = (&caps["end"]).to_string();
        Ok(CaveConnection { start, end })
    }
}

impl From<CaveConnection> for (String, String) {
    fn from(connection: CaveConnection) -> Self {
        (connection.start, connection.end)
    }
}

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("could not parse cave connection from '{0}'")]
pub(crate) struct ParseCaveConnectionError(String);
