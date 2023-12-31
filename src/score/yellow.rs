use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::building::{BuildingType, YellowBuilding};
use crate::building_config::BuildingConfig;
use crate::score::score_by_adjacency;

// -----------------------------------------------------------------------------
fn score_bakeries(board: &Board) -> i32 {
    let score = score_by_adjacency(
        true,
        board,
        BuildingType::Yellow,
        HashSet::from([BuildingType::Black, BuildingType::Red]),
        3,
    );

    score
}

// -----------------------------------------------------------------------------
fn score_markets(board: &Board) -> i32 {
    let (markets_in_row, markets_in_col) = board.spaces()
        .iter()
        .enumerate()
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut rows, mut cols), (idx, space)| {
                if space.building_type_eq(BuildingType::Yellow) {
                    *rows.entry(board.row(idx)).or_insert(0) += 1;
                    *cols.entry(board.col(idx)).or_insert(0) += 1;
                }
                (rows, cols)
            }
        );

    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |n, (idx, space)|
            if space.building_type_eq(BuildingType::Yellow) {
                n + cmp::max(
                    markets_in_row.get(&board.row(idx)).copied().unwrap_or(0),
                    markets_in_col.get(&board.col(idx)).copied().unwrap_or(0),
                )
            } else {
                n
            }
        );

    score
}

// -----------------------------------------------------------------------------
fn score_tailors(board: &Board) -> i32 {
    let (tailors, tailors_in_center) = board.spaces()
        .iter()
        .enumerate()
        .fold((0, 0), |(n, m), (idx, space)|
            if space.building_type_eq(BuildingType::Yellow) {
                if board.center_idxs().contains(&idx) {
                    (n + 1, m + 1)
                } else {
                    (n + 1, m)
                }
            } else {
                (n, m)
            }
        );

    let score = (tailors_in_center + 1) * tailors;

    score
}

// -----------------------------------------------------------------------------
fn score_theaters(board: &Board) -> i32 {
    let (unique_building_types_in_row, unique_building_types_in_col) = board.spaces()
        .iter()
        .enumerate()
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut rows, mut cols), (idx, space)| {
                if let Some(building_type) = space.building_type() {
                    if building_type != BuildingType::Yellow {
                        rows.entry(board.row(idx))
                            .or_insert(HashSet::new())
                            .insert(building_type);
                        cols.entry(board.col(idx))
                            .or_insert(HashSet::new())
                            .insert(building_type);
                    }
                }
                (rows, cols)
            }
        );

    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |n, (idx, space)|
            if space.building_type_eq(BuildingType::Yellow) {
                unique_building_types_in_row.get(&board.row(idx))
                    .unwrap_or(&HashSet::new())
                    .union(
                        unique_building_types_in_col.get(&board.col(idx))
                            .unwrap_or(&HashSet::new())
                    )
                    .count()
                    + n
            } else {
                n
            }
        )
        as i32;

    score
}

// -----------------------------------------------------------------------------
pub fn score(board: &Board, building_config: &BuildingConfig) -> i32 {
    let score = match building_config.yellow() {
        YellowBuilding::Bakery => score_bakeries(board),
        YellowBuilding::Market => score_markets(board),
        YellowBuilding::Tailor => score_tailors(board),
        YellowBuilding::Theater => score_theaters(board),
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;


    // -------------------------------------------------------------------------
    #[test]
    fn test_score_bakeries() {
        let mut board = Board::new(4, 4);
        let score = score_bakeries(&board);
        assert_eq!(score, 0);

        board.place(0, BuildingType::Yellow);
        let score = score_bakeries(&board);
        assert_eq!(score, 0);

        board.place(1, BuildingType::Red);
        let score = score_bakeries(&board);
        assert_eq!(score, 3);

        board.place(2, BuildingType::Black);
        let score = score_bakeries(&board);
        assert_eq!(score, 3);

        board.place(3, BuildingType::Yellow);
        let score = score_bakeries(&board);
        assert_eq!(score, 6);

        board.place(4, BuildingType::Black);
        let score = score_bakeries(&board);
        assert_eq!(score, 6);

        board.place(5, BuildingType::Yellow);
        let score = score_bakeries(&board);
        assert_eq!(score, 9);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_markets() {
        let mut board = Board::new(4, 4);
        let score = score_markets(&board);
        assert_eq!(score, 0);

        board.place(0, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 1);

        board.place(15, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 2);

        board.place(1, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 5);

        board.place(4, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 7);

        board.place(2, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 12);

        board.place(8, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 16);

        board.place(12, BuildingType::Yellow);
        assert_eq!(score_markets(&board), 24);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_tailors() {
        let mut board = Board::new(4, 4);
        assert_eq!(score_tailors(&board), 0);

        board.place(0, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 1);

        board.place(5, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 4);

        board.place(1, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 6);

        board.place(6, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 12);

        board.place(9, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 20);

        board.place(10, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 30);

        board.place(11, BuildingType::Yellow);
        assert_eq!(score_tailors(&board), 35);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_theaters() {
        let mut board = Board::new(4, 4);
        assert_eq!(score_theaters(&board), 0);

        board.place(0, BuildingType::Yellow);
        assert_eq!(score_theaters(&board), 0);

        board.place(1, BuildingType::Green);
        assert_eq!(score_theaters(&board), 1);

        board.place(2, BuildingType::Black);
        assert_eq!(score_theaters(&board), 2);

        board.place(3, BuildingType::Blue);
        assert_eq!(score_theaters(&board), 3);

        board.place(4, BuildingType::Red);
        assert_eq!(score_theaters(&board), 4);

        board.place(8, BuildingType::Gray);
        assert_eq!(score_theaters(&board), 5);

        board.place(12, BuildingType::Yellow);
        assert_eq!(score_theaters(&board), 7);

        board.remove(12);
        board.place(12, BuildingType::Magenta);
        assert_eq!(score_theaters(&board), 6);
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }
}
