use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day0SolutionExecutor;

impl SolutionExecutor for Day0SolutionExecutor {
    type Input = Vec<String>;
    type Part1Output = Vec<String>;
    type Part2Output = ();

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        input
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}
