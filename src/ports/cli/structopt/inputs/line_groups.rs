pub(crate) trait LineGroups {
    fn line_groups(&self) -> Vec<Vec<String>>;
}

impl LineGroups for str {
    fn line_groups(&self) -> Vec<Vec<String>> {
        self.lines()
            .fold(vec![vec![]], |mut acc, curr| {
                if curr.is_empty() {
                    acc.push(vec![]);
                } else {
                    acc.last_mut().unwrap().push(curr.to_string());
                }
                acc
            })
            .into_iter()
            .filter(|group| !group.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn groups_lines() {
        assert_that(&"line1\n\nline2".line_groups())
            .is_equal_to(vec![vec!["line1".to_string()], vec!["line2".to_string()]]);
    }
}
