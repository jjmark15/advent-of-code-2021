pub(crate) mod day_00;
pub(crate) mod day_01;
pub(crate) mod day_02;
pub(crate) mod day_03;
pub(crate) mod day_04;
pub(crate) mod day_05;
pub(crate) mod day_06;
pub(crate) mod day_07;
pub(crate) mod day_08;
pub(crate) mod day_09;

pub(crate) trait SolutionExecutor {
    type Input;
    type Part1Output;
    type Part2Output;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output;

    fn part_2(&self, input: Self::Input) -> Self::Part2Output;
}
