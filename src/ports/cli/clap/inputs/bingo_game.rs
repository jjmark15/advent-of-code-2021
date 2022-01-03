use std::num::ParseIntError;
use std::str::FromStr;

use crate::ports::cli::clap::inputs::line_groups::LineGroups;

pub(crate) struct BingoGame {
    draw_order: Vec<u8>,
    boards: Vec<BingoBoard>,
}

pub(crate) struct BingoBoard {
    rows: Vec<Vec<u8>>,
}

impl BingoBoard {
    pub(crate) fn inner(self) -> Vec<Vec<u8>> {
        self.rows
    }
}

impl TryFrom<String> for BingoGame {
    type Error = ParseBingoGameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let line_groups = value.line_groups();
        let draw_order = parse_draw_order(line_groups.first().unwrap().first().unwrap())?;

        Ok(BingoGame {
            draw_order,
            boards: line_groups[1..]
                .iter()
                .map(|group| parse_board(group))
                .collect::<Result<Vec<BingoBoard>, ParseBingoGameError>>()?,
        })
    }
}

fn parse_draw_order(s: &str) -> Result<Vec<u8>, ParseBingoGameError> {
    s.trim()
        .split(',')
        .map(u8::from_str)
        .collect::<Result<Vec<u8>, ParseIntError>>()
        .map_err(ParseBingoGameError::from)
}

fn parse_board(line_group: &[String]) -> Result<BingoBoard, ParseBingoGameError> {
    Ok(BingoBoard {
        rows: line_group
            .iter()
            .map(|line| {
                line.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(u8::from_str)
                    .collect::<Result<Vec<u8>, ParseIntError>>()
                    .map_err(ParseBingoGameError::from)
            })
            .collect::<Result<Vec<Vec<u8>>, ParseBingoGameError>>()?,
    })
}

#[derive(Debug, thiserror::Error, derive_new::new)]
#[error("could not parse bingo game")]
pub(crate) struct ParseBingoGameError;

impl From<ParseIntError> for ParseBingoGameError {
    fn from(_: ParseIntError) -> Self {
        ParseBingoGameError::new()
    }
}

impl From<BingoGame> for crate::domain::solution_executor::day_04::BingoGame {
    fn from(game: BingoGame) -> Self {
        crate::domain::solution_executor::day_04::BingoGame::new(
            game.draw_order,
            game.boards.into_iter().map(BingoBoard::inner).collect(),
        )
    }
}
