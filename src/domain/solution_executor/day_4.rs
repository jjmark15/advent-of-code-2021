use itertools::Itertools;

use crate::domain::solution_executor::SolutionExecutor;

#[derive(Debug)]
pub(crate) struct BingoGame {
    draw_order: Vec<u8>,
    boards: Vec<BingoBoard>,
}

impl BingoGame {
    pub(crate) fn new(draw_order: Vec<u8>, boards: Vec<Vec<Vec<u8>>>) -> Self {
        BingoGame {
            draw_order,
            boards: boards.into_iter().map(BingoBoard::new).collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct BingoBoard {
    rows: Vec<Vec<BoardCell>>,
}

impl BingoBoard {
    pub(crate) fn new(rows: Vec<Vec<u8>>) -> Self {
        BingoBoard {
            rows: rows
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|value| BoardCell::new(value, MarkedOrUnmarked::Unmarked))
                        .collect()
                })
                .collect(),
        }
    }

    fn unmarked_numbers(&self) -> Vec<u8> {
        self.rows
            .iter()
            .flatten()
            .filter(|cell| !cell.is_marked())
            .map(|cell| cell.value)
            .collect()
    }

    fn rows(&self) -> &Vec<Vec<BoardCell>> {
        &self.rows
    }

    fn cell_at_position(&self, row: usize, column: usize) -> &BoardCell {
        self.rows.get(row).unwrap().get(column).unwrap()
    }

    fn columns(&self) -> Vec<Vec<BoardCell>> {
        self.rows
            .first()
            .unwrap()
            .iter()
            .enumerate()
            .map(|(col_index, _first_cell)| {
                (0..5)
                    .map(|row_index| *self.cell_at_position(row_index, col_index))
                    .collect::<Vec<BoardCell>>()
            })
            .collect::<Vec<Vec<BoardCell>>>()
    }
}

#[derive(derive_new::new, Copy, Clone, Debug)]
struct BoardCell {
    value: u8,
    state: MarkedOrUnmarked,
}

impl BoardCell {
    fn is_marked(&self) -> bool {
        match self.state {
            MarkedOrUnmarked::Marked => true,
            MarkedOrUnmarked::Unmarked => false,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum MarkedOrUnmarked {
    Marked,
    Unmarked,
}

fn mark_drawn_number(board: &mut BingoBoard, drawn_number: u8) {
    board.rows.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            if cell.value == drawn_number {
                cell.state = MarkedOrUnmarked::Marked;
            }
        })
    })
}

#[derive(derive_new::new)]
struct CompletionDetector;

impl CompletionDetector {
    fn is_complete(&self, board: &BingoBoard) -> bool {
        has_a_fully_marked_list(board.rows()) || has_a_fully_marked_list(&board.columns())
    }
}

fn has_a_fully_marked_list(list: &Vec<Vec<BoardCell>>) -> bool {
    list.iter()
        .any(|inner| inner.iter().all(|cell| cell.is_marked()))
}

#[derive(derive_new::new)]
pub(crate) struct Day4SolutionExecutor;

impl SolutionExecutor for Day4SolutionExecutor {
    type Input = BingoGame;
    type Part1Output = usize;
    type Part2Output = usize;

    fn part_1(&self, mut input: Self::Input) -> Self::Part1Output {
        let completion_detector: CompletionDetector = CompletionDetector::new();
        let mut completed_board: Option<BingoBoard> = None;
        let mut final_drawn_number: Option<u8> = None;

        for drawn_number in input.draw_order {
            input
                .boards
                .iter_mut()
                .for_each(|board| mark_drawn_number(board, drawn_number));

            for board in &input.boards {
                if completion_detector.is_complete(board) {
                    completed_board = Some(board.clone());
                    final_drawn_number = Some(drawn_number);
                    break;
                }
            }
            if final_drawn_number.is_some() {
                break;
            }
        }

        completed_board
            .unwrap()
            .unmarked_numbers()
            .into_iter()
            .map(|value| value as usize)
            .sum::<usize>()
            * final_drawn_number.unwrap() as usize
    }

    fn part_2(&self, mut input: Self::Input) -> Self::Part2Output {
        let completion_detector: CompletionDetector = CompletionDetector::new();
        let mut last_completed_board: Option<BingoBoard> = None;
        let mut final_drawn_number: Option<u8> = None;

        for drawn_number in input.draw_order {
            input
                .boards
                .iter_mut()
                .for_each(|board| mark_drawn_number(board, drawn_number));

            if input.boards.len() > 1 {
                input.boards = input
                    .boards
                    .into_iter()
                    .filter(|board| !completion_detector.is_complete(board))
                    .collect();
            }

            if let Ok(Some(remaining_board)) = input
                .boards
                .iter()
                .filter(|board| completion_detector.is_complete(board))
                .at_most_one()
            {
                last_completed_board = Some(remaining_board.clone());
                final_drawn_number = Some(drawn_number);
                break;
            }
        }

        let unmarked_numbers_sum = last_completed_board
            .unwrap()
            .unmarked_numbers()
            .into_iter()
            .map(|value| value as usize)
            .sum::<usize>();

        unmarked_numbers_sum * final_drawn_number.unwrap() as usize
    }
}

#[cfg(test)]
mod tests {
    use speculoos::prelude::*;

    use super::*;

    fn test_game() -> BingoGame {
        let draw_order = vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let boards: Vec<Vec<Vec<u8>>> = vec![
            vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ],
            vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ],
            vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ],
        ];
        BingoGame::new(draw_order, boards)
    }

    #[test]
    fn calculates_score_of_first_winning_board() {
        assert_that(&Day4SolutionExecutor::new().part_1(test_game())).is_equal_to(4512);
    }

    #[test]
    fn calculates_score_of_last_winning_board() {
        assert_that(&Day4SolutionExecutor::new().part_2(test_game())).is_equal_to(1924);
    }
}
