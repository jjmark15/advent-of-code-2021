use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day12SolutionExecutor;

impl SolutionExecutor for Day12SolutionExecutor {
    type Input = Vec<(String, String)>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        PathFinder::new()
            .find_all_possible_paths(to_cave_connections(input))
            .len()
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[derive(Debug, derive_new::new)]
struct PathFinder;

impl PathFinder {
    fn find_all_possible_paths(&self, cave_connections: Vec<CaveConnection>) -> Vec<CavePath> {
        let cave_map = CaveMap::new(cave_connections);
        let mut exploration_stack: Vec<CavePath> =
            vec![CavePath::new(vec![Cave::new("start".to_string())])];
        let mut paths = Vec::new();

        while let Some(path) = exploration_stack.pop() {
            if path.is_complete() {
                paths.push(path);
            } else {
                let adjacent: Vec<Cave> = cave_map
                    .adjacent(path.last().unwrap())
                    .into_iter()
                    .filter(|cave| path.can_go_to(cave))
                    .collect();

                if adjacent.is_empty() {
                    paths.push(path);
                } else {
                    let mut new_paths: Vec<CavePath> = adjacent
                        .into_iter()
                        .map(|cave| path.extended_with(cave))
                        .filter(|new_path| !paths.contains(new_path))
                        .filter(|new_path| !exploration_stack.contains(new_path))
                        .collect();
                    exploration_stack.append(&mut new_paths);
                }
            }
        }

        paths.into_iter().filter(CavePath::is_complete).collect()
    }
}

#[derive(derive_new::new)]
struct CaveMap {
    connections: Vec<CaveConnection>,
}

impl CaveMap {
    fn adjacent(&self, cave: &Cave) -> Vec<Cave> {
        self.connections
            .iter()
            .filter(|connection| connection.contains(cave))
            .map(|connection| {
                if connection.start() == cave {
                    connection.end()
                } else {
                    connection.start()
                }
            })
            .cloned()
            .collect()
    }
}

#[derive(derive_new::new, Eq, PartialEq)]
struct CavePath {
    steps: Vec<Cave>,
}

impl CavePath {
    fn is_complete(&self) -> bool {
        self.steps
            .last()
            .map(|cave| cave.name == "end")
            .unwrap_or(false)
    }

    fn includes(&self, cave: &Cave) -> bool {
        self.steps.contains(cave)
    }

    fn can_go_to(&self, cave: &Cave) -> bool {
        !(self.includes(cave) && cave.is_small())
    }

    fn last(&self) -> Option<&Cave> {
        self.steps.last()
    }

    fn extended_with(&self, cave: Cave) -> Self {
        let mut steps = self.steps.clone();
        steps.push(cave);
        CavePath::new(steps)
    }
}

#[derive(Debug, derive_getters::Getters, derive_new::new)]
struct CaveConnection {
    start: Cave,
    end: Cave,
}

impl CaveConnection {
    fn contains(&self, cave: &Cave) -> bool {
        &self.start == cave || &self.end == cave
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Cave {
    name: String,
    is_small: bool,
}

impl Cave {
    fn new(name: String) -> Self {
        let is_small = name.to_lowercase() == name;
        Cave { name, is_small }
    }

    fn is_small(&self) -> bool {
        self.is_small
    }
}

fn to_cave_connections(input: Vec<(String, String)>) -> Vec<CaveConnection> {
    input
        .into_iter()
        .map(|(start_name, end_name)| {
            CaveConnection::new(Cave::new(start_name), Cave::new(end_name))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_data() -> Vec<(String, String)> {
        vec![
            ("fs", "end"),
            ("he", "DX"),
            ("fs", "he"),
            ("start", "DX"),
            ("pj", "DX"),
            ("end", "zg"),
            ("zg", "sl"),
            ("zg", "pj"),
            ("pj", "he"),
            ("RW", "he"),
            ("fs", "DX"),
            ("pj", "RW"),
            ("zg", "RW"),
            ("start", "pj"),
            ("he", "WI"),
            ("zg", "he"),
            ("pj", "fs"),
            ("start", "RW"),
        ]
        .into_iter()
        .map(|(start, end)| (start.to_string(), end.to_string()))
        .collect()
    }

    #[test]
    fn counts_paths_through_cave_using_small_caves_at_most_once() {
        assert_that(&Day12SolutionExecutor::new().part_1(test_data())).is_equal_to(226);
    }
}
