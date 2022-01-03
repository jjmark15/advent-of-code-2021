use std::fmt::{Display, Formatter};

#[derive(derive_new::new)]
pub(crate) struct List<T: Display> {
    elements: Vec<T>,
}

impl<T: Display> Display for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.elements
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
