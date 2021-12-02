use crate::domain::solution_executor::SolutionExecutor;

#[derive(derive_new::new)]
pub(crate) struct DirectionAndSize {
    direction: Direction,
    size: u64,
}

pub(crate) enum Direction {
    Forward,
    Down,
    Up,
}

#[derive(Debug)]
struct Position {
    depth: u64,
    distance: u64,
}

#[derive(Debug)]
struct Submarine {
    position: Position,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            position: Position {
                depth: 0,
                distance: 0,
            },
        }
    }

    fn travel(&mut self, movement: DirectionAndSize) {
        match movement.direction {
            Direction::Forward => self.position.distance += movement.size,
            Direction::Down => self.position.depth += movement.size,
            Direction::Up => self.position.depth -= movement.size,
        }
    }

    fn position(&self) -> &Position {
        &self.position
    }
}

#[derive(derive_new::new)]
pub(crate) struct Day2SolutionExecutor;

impl SolutionExecutor for Day2SolutionExecutor {
    type Input = Vec<DirectionAndSize>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn part_1(&self, input: Self::Input) -> Self::Part1Output {
        let mut submarine = Submarine::new();
        input
            .into_iter()
            .for_each(|movement| submarine.travel(movement));

        submarine.position().depth * submarine.position().distance
    }

    fn part_2(&self, _input: Self::Input) -> Self::Part2Output {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    #[test]
    fn calculates_product_of_depth_and_distance_after_movement() {
        let executor: Day2SolutionExecutor = Day2SolutionExecutor::new();
        let movement: Vec<DirectionAndSize> = vec![
            DirectionAndSize::new(Direction::Forward, 5),
            DirectionAndSize::new(Direction::Down, 5),
            DirectionAndSize::new(Direction::Forward, 8),
            DirectionAndSize::new(Direction::Up, 3),
            DirectionAndSize::new(Direction::Down, 8),
            DirectionAndSize::new(Direction::Forward, 2),
        ];
        assert_that(&executor.part_1(movement)).is_equal_to(150);
    }
}
