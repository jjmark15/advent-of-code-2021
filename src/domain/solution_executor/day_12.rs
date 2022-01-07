use std::collections::{HashMap, HashSet};

use rayon::prelude::*;

use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day12SolutionExecutor;

impl SolutionExecutor for Day12SolutionExecutor {
    type Input = Vec<(String, String)>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let cave_map = CaveMap::new(to_cave_connections(input));
        let starting_path = CavePath::new(vec![Cave::new("start".to_owned())]);
        PathFinder::new().count_possible_paths(&cave_map, starting_path, false)
    }

    fn part_2(&self, input: Self::Input) -> Self::Part2Output {
        let cave_map = CaveMap::new(to_cave_connections(input));
        let starting_path = CavePath::new(vec![Cave::new("start".to_owned())]);
        PathFinder::new().count_possible_paths_v2(&cave_map, starting_path, true)
    }
}

#[derive(Debug, derive_new::new)]
struct PathFinder;

impl PathFinder {
    fn count_possible_paths(
        &self,
        cave_map: &CaveMap,
        starting_path: CavePath,
        allow_single_small_twice: bool,
    ) -> usize {
        let mut exploration_stack: Vec<CavePath> = vec![starting_path];
        let mut path_count: usize = 0;

        while let Some(path) = exploration_stack.pop() {
            exploration_stack.extend(
                cave_map
                    .adjacent(path.last().unwrap())
                    .into_iter()
                    .filter(|&cave| path.can_go_to(cave, allow_single_small_twice))
                    .filter(|&cave| {
                        if cave.is_end() {
                            path_count += 1;
                            false
                        } else {
                            true
                        }
                    })
                    .map(|cave| path.extended_with(cave.clone())),
            );
        }

        path_count
    }

    #[allow(dead_code)]
    fn count_possible_paths_v2(
        &self,
        cave_map: &CaveMap,
        path: CavePath,
        allow_single_small_twice: bool,
    ) -> usize {
        if path.last().unwrap().is_end() {
            return 1;
        }

        cave_map
            .adjacent(path.last().unwrap())
            .into_par_iter()
            .map(|cave| {
                if path.can_go_to(cave, allow_single_small_twice) {
                    self.count_possible_paths_v2(
                        cave_map,
                        path.extended_with(cave.clone()),
                        allow_single_small_twice,
                    )
                } else {
                    0
                }
            })
            .sum()
    }
}

struct CaveMap {
    adjacency_lists: HashMap<Cave, HashSet<Cave>>,
}

impl CaveMap {
    fn new(connections: Vec<CaveConnection>) -> Self {
        let adjacency_lists: HashMap<Cave, HashSet<Cave>> =
            connections
                .into_iter()
                .fold(HashMap::new(), |mut map, connection| {
                    Self::insert_or_initialise_with_value(
                        &mut map,
                        connection.start(),
                        connection.end(),
                    );
                    Self::insert_or_initialise_with_value(
                        &mut map,
                        connection.end(),
                        connection.start(),
                    );

                    map
                });
        CaveMap { adjacency_lists }
    }

    fn insert_or_initialise_with_value(
        map: &mut HashMap<Cave, HashSet<Cave>>,
        key: &Cave,
        value: &Cave,
    ) {
        if let Some(adjacent_caves) = map.get_mut(key) {
            adjacent_caves.insert(value.clone());
        } else {
            let mut adjacent_caves = HashSet::new();
            adjacent_caves.insert(value.clone());
            map.insert(key.clone(), adjacent_caves);
        }
    }

    fn adjacent(&self, cave: &Cave) -> HashSet<&Cave> {
        self.adjacency_lists
            .get(cave)
            .map(|list| list.iter().collect())
            .unwrap_or_default()
    }
}

#[derive(derive_new::new, Eq, PartialEq, Debug)]
struct CavePath {
    steps: Vec<Cave>,
    #[new(value = "false")]
    has_used_small_cave_twice: bool,
}

impl CavePath {
    fn can_go_to(&self, cave: &Cave, allow_single_small_twice: bool) -> bool {
        if cave.name() == "start" {
            false
        } else if cave.is_small() && self.steps.contains(cave) {
            allow_single_small_twice && !self.has_used_small_cave_twice
        } else {
            true
        }
    }

    fn last(&self) -> Option<&Cave> {
        self.steps.last()
    }

    fn extended_with(&self, cave: Cave) -> Self {
        let has_used_small_cave_twice =
            self.has_used_small_cave_twice || (self.steps.contains(&cave) && cave.is_small());
        let mut steps = self.steps.clone();
        steps.push(cave);
        let mut new_path = CavePath::new(steps);
        new_path.has_used_small_cave_twice = has_used_small_cave_twice;
        new_path
    }
}

#[derive(Debug, derive_getters::Getters, derive_new::new)]
struct CaveConnection {
    start: Cave,
    end: Cave,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
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

    fn name(&self) -> &String {
        &self.name
    }

    fn is_end(&self) -> bool {
        self.name == "end"
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

    #[test]
    fn counts_paths_visiting_small_caves_at_most_once_with_one_small_twice() {
        assert_that(&Day12SolutionExecutor::new().part_2(test_data())).is_equal_to(3509);
    }
}
