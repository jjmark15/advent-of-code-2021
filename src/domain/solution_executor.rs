pub(crate) trait SolutionExecutor {
    type Part1Input;
    type Part1Output;

    type Part2Input;
    type Part2Output;

    fn part_1(&self, input: Self::Part1Input) -> Self::Part1Output;

    fn part_2(&self, input: Self::Part2Input) -> Self::Part2Output;
}
