pub(crate) use day_0::Day0SolutionExecutor;
pub(crate) use day_1::Day1SolutionExecutor;

mod day_0;
mod day_1;

pub(crate) trait SolutionExecutor {
    type Input;
    type Part1Output;
    type Part2Output;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output;

    fn part_2(&self, input: Self::Input) -> Self::Part2Output;
}
