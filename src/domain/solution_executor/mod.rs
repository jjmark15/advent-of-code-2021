pub(crate) mod day_0;
pub(crate) mod day_1;
pub(crate) mod day_2;
pub(crate) mod day_3;
pub(crate) mod day_4;
pub(crate) mod day_5;
pub(crate) mod day_6;
pub(crate) mod day_7;
pub(crate) mod day_8;

pub(crate) trait SolutionExecutor {
    type Input;
    type Part1Output;
    type Part2Output;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output;

    fn part_2(&self, input: Self::Input) -> Self::Part2Output;
}
