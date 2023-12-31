use std::collections::HashMap;

use crate::board::Board;
use crate::building::{BuildingType, GreenBuilding};
use crate::building_config::BuildingConfig;

// -----------------------------------------------------------------------------
fn score_almshouses(board: &Board) -> i32 {
    let score = match board.count_building_type(BuildingType::Green) {
        0 => 0,
        1 => -1,
        2 => 5,
        3 => -3,
        4 => 15,
        5 => -5,
        _ => 26,
    };

    score
}

// -----------------------------------------------------------------------------
fn score_feast_halls(board: &Board, other: &Board) -> i32 {
    let count = board.count_building_type(BuildingType::Green);
    let score =
        if count > other.count_building_type(BuildingType::Green) {
            3 * count
        } else {
            2 * count
        }
        as i32;

    score
}

// -----------------------------------------------------------------------------
fn score_inns(board: &Board) -> i32 {
    // Count number of inns in each row and column.
    let (inns_in_row, inns_in_col) = board.spaces()
        .iter()
        .enumerate()
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut row, mut col), (idx, space)| {
                if space.building_type_eq(BuildingType::Green) {
                    *row.entry(board.row(idx)).or_insert(0) += 1;
                    *col.entry(board.col(idx)).or_insert(0) += 1;
                }
                (row, col)
            }
        );

    // Score only the inns that are alone in their column and row.
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |n, (idx, space)| {
            if space.building_type_eq(BuildingType::Green)
                && inns_in_row.get(&board.row(idx)).copied().unwrap_or(0) == 1
                && inns_in_col.get(&board.col(idx)).copied().unwrap_or(0) == 1 {
                n + 3
            } else {
                n
            }
        });

    score
}

// -----------------------------------------------------------------------------
fn score_taverns(board: &Board) -> i32 {
    let score = match board.count_building_type(BuildingType::Green) {
        0 => 0,
        1 => 2,
        2 => 5,
        3 => 9,
        4 => 14,
        _ => 20,
    };

    score
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    other: &Board,
) -> i32 {
    let score = match building_config.green() {
        GreenBuilding::Almshouse => score_almshouses(board),
        GreenBuilding::FeastHall => score_feast_halls(board, other),
        GreenBuilding::Inn => score_inns(board),
        GreenBuilding::Tavern => score_taverns(board)
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_almshouses() {
        let mut board = Board::new(4, 4);

        let score = score_almshouses(&board);
        assert_eq!(score, 0);

        board.place(0, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, -1);

        board.place(1, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, 5);

        board.place(2, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, -3);

        board.place(3, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, 15);

        board.place(4, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, -5);

        board.place(5, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, 26);

        board.place(6, BuildingType::Green);
        let score = score_almshouses(&board);
        assert_eq!(score, 26);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_feast_halls() {
        let mut board = Board::new(4, 4);
        let mut other = Board::new(4, 4);

        let score = score_feast_halls(&board, &other);
        assert_eq!(score, 0);

        board.place(0, BuildingType::Green);
        let score = score_feast_halls(&board, &other);
        assert_eq!(score, 3);

        other.place(0, BuildingType::Green);
        let score = score_feast_halls(&board, &other);
        assert_eq!(score, 2);

        other.place(1, BuildingType::Green);
        let score = score_feast_halls(&board, &other);
        assert_eq!(score, 2);

        board.place(1, BuildingType::Green);
        let score = score_feast_halls(&board, &other);
        assert_eq!(score, 4);

        board.place(2, BuildingType::Green);
        let score = score_feast_halls(&board, &other);
        assert_eq!(score, 9);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_inns() {
        let mut board = Board::new(4, 4);
        let score = score_inns(&board);
        assert_eq!(score, 0);

        board.place(0, BuildingType::Green);
        let score = score_inns(&board);
        assert_eq!(score, 3);

        board.place(1, BuildingType::Green);
        let score = score_inns(&board);
        assert_eq!(score, 0);

        board.place(5, BuildingType::Green);
        let score = score_inns(&board);
        assert_eq!(score, 0);

        board.remove(1);
        let score = score_inns(&board);
        assert_eq!(score, 6);

        board.place(10, BuildingType::Green);
        let score = score_inns(&board);
        assert_eq!(score, 9);

        board.place(12, BuildingType::Green);
        let score = score_inns(&board);
        assert_eq!(score, 6);

        board.remove(12);
        board.place(15, BuildingType::Green);
        let score = score_inns(&board);
        assert_eq!(score, 12);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_taverns() {
        let mut board = Board::new(4, 4);

        let score = score_taverns(&board);
        assert_eq!(score, 0);

        board.place(0, BuildingType::Green);
        let score = score_taverns(&board);
        assert_eq!(score, 2);

        board.place(1, BuildingType::Green);
        let score = score_taverns(&board);
        assert_eq!(score, 5);

        board.place(2, BuildingType::Green);
        let score = score_taverns(&board);
        assert_eq!(score, 9);

        board.place(3, BuildingType::Green);
        let score = score_taverns(&board);
        assert_eq!(score, 14);

        board.place(4, BuildingType::Green);
        let score = score_taverns(&board);
        assert_eq!(score, 20);

        board.place(5, BuildingType::Green);
        let score = score_taverns(&board);
        assert_eq!(score, 20);
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }
}
