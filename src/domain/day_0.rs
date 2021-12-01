use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct Day0SolutionExecutor;

impl SolutionExecutor for Day0SolutionExecutor {
    type Part1Input = Vec<String>;
    type Part1Output = Vec<String>;
    type Part2Input = ();
    type Part2Output = ();

    fn part_1(&self, input: Self::Part1Input) -> Self::Part1Output {
        input
    }

    fn part_2(&self, _input: Self::Part2Input) -> Self::Part2Output {
        unimplemented!()
    }
}
