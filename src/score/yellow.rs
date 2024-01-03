use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::building::{BuildingType, YellowBuilding};
use crate::building_config::BuildingConfig;
use crate::score::score_if_adjacent_to;

use super::score_per_each;

// -----------------------------------------------------------------------------
fn score_bakeries(board: &Board) -> HashMap<usize, i32> {
    let scores = score_if_adjacent_to(
        board,
        BuildingType::Yellow,
        &HashSet::from([BuildingType::Black, BuildingType::Red]),
        3
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_markets(board: &Board) -> HashMap<usize, i32> {
    // Count the number of markets in each row and column.
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

    // Score each market.
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(BuildingType::Yellow) {
                let points = cmp::max(
                    markets_in_row.get(&board.row(idx)).copied().unwrap_or(0),
                    markets_in_col.get(&board.col(idx)).copied().unwrap_or(0),
                );
                m.insert(idx, points);
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
fn score_tailors(board: &Board) -> HashMap<usize, i32> {
    let tailors_in_center = board.center_idxs()
        .into_iter()
        .fold(0, |n, idx| {
            let space = &board.spaces()[idx];
            if space.building_type_eq(BuildingType::Yellow) {
                n + 1
            } else {
                n
            }
        });
    let points = tailors_in_center + 1;
    let scores = score_per_each(board, BuildingType::Yellow, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_theaters(board: &Board) -> HashMap<usize, i32> {
    // Create sets of unique building types in each row and column.
    let (unique_building_types_per_row, unique_building_types_per_col)
        = board.spaces()
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

    // Score each theater.
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(BuildingType::Yellow) {
                let empty_set = HashSet::new();
                let unique_building_types_in_row = unique_building_types_per_row
                    .get(&board.row(idx))
                    .unwrap_or(&empty_set);
                let unique_building_types_in_col = unique_building_types_per_col
                    .get(&board.col(idx))
                    .unwrap_or(&empty_set);
                let points = unique_building_types_in_row
                    .union(unique_building_types_in_col)
                    .count()
                    as i32;
                m.insert(idx, points);
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig
) -> HashMap<usize, i32> {
    let scores = match building_config.yellow() {
        YellowBuilding::Bakery => score_bakeries(board),
        YellowBuilding::Market => score_markets(board),
        YellowBuilding::Tailor => score_tailors(board),
        YellowBuilding::Theater => score_theaters(board),
    };

    scores
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_bakeries() {
        let mut board = Board::new(4, 4);
        assert!(score_bakeries(&board).is_empty());

        board.place(0, BuildingType::Yellow);
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_bakeries(&board), ans);

        board.place(1, BuildingType::Red);
        let ans = HashMap::from([(0, 3)]);
        assert_eq!(score_bakeries(&board), ans);

        board.place(2, BuildingType::Black);
        assert_eq!(score_bakeries(&board), ans);

        board.place(3, BuildingType::Yellow);
        let ans = HashMap::from([(0, 3), (3, 3)]);
        assert_eq!(score_bakeries(&board), ans);

        board.place(4, BuildingType::Black);
        let ans = HashMap::from([(0, 3), (3, 3)]);
        assert_eq!(score_bakeries(&board), ans);

        board.place(5, BuildingType::Yellow);
        let ans = HashMap::from([(0, 3), (3, 3), (5, 3)]);
        assert_eq!(score_bakeries(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_markets() {
        let mut board = Board::new(4, 4);
        assert!(score_markets(&board).is_empty());

        board.place(0, BuildingType::Yellow);
        let ans = HashMap::from([(0, 1)]);
        assert_eq!(score_markets(&board), ans);

        board.place(15, BuildingType::Yellow);
        let ans = HashMap::from([(0, 1), (15, 1)]);
        assert_eq!(score_markets(&board), ans);

        board.place(1, BuildingType::Yellow);
        let ans = HashMap::from([(0, 2), (1, 2), (15, 1)]);
        assert_eq!(score_markets(&board), ans);

        board.place(4, BuildingType::Yellow);
        let ans = HashMap::from([(0, 2), (1, 2), (4, 2), (15, 1)]);
        assert_eq!(score_markets(&board), ans);

        board.place(2, BuildingType::Yellow);
        let ans = HashMap::from([(0, 3), (1, 3), (2, 3), (4, 2), (15, 1)]);
        assert_eq!(score_markets(&board), ans);

        board.place(8, BuildingType::Yellow);
        let ans = HashMap::from([
            (0, 3),
            (1, 3),
            (2, 3),
            (4, 3),
            (8, 3),
            (15, 1),
        ]);
        assert_eq!(score_markets(&board), ans);

        board.place(12, BuildingType::Yellow);
        let ans = HashMap::from([
            (0, 4),
            (1, 3),
            (2, 3),
            (4, 4),
            (8, 4),
            (12, 4),
            (15, 2),
        ]);
        assert_eq!(score_markets(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_tailors() {
        let mut board = Board::new(4, 4);
        assert!(score_tailors(&board).is_empty());

        board.place(0, BuildingType::Yellow);
        let ans = HashMap::from([(0, 1)]);
        assert_eq!(score_tailors(&board), ans);

        board.place(5, BuildingType::Yellow);
        let ans = HashMap::from([(0, 2), (5, 2)]);
        assert_eq!(score_tailors(&board), ans);

        board.place(1, BuildingType::Yellow);
        let ans = HashMap::from([(0, 2), (1, 2), (5, 2)]);
        assert_eq!(score_tailors(&board), ans);

        board.place(6, BuildingType::Yellow);
        let ans = HashMap::from([(0, 3), (1, 3), (5, 3), (6, 3)]);
        assert_eq!(score_tailors(&board), ans);

        board.place(9, BuildingType::Yellow);
        let ans = HashMap::from([(0, 4), (1, 4), (5, 4), (6, 4), (9, 4)]);
        assert_eq!(score_tailors(&board), ans);

        board.place(10, BuildingType::Yellow);
        let ans = HashMap::from([
            (0, 5),
            (1, 5),
            (5, 5),
            (6, 5),
            (9, 5),
            (10, 5),
        ]);
        assert_eq!(score_tailors(&board), ans);

        board.place(11, BuildingType::Yellow);
        let ans = HashMap::from([
            (0, 5),
            (1, 5),
            (5, 5),
            (6, 5),
            (9, 5),
            (10, 5),
            (11, 5),
        ]);
        assert_eq!(score_tailors(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_theaters() {
        let mut board = Board::new(4, 4);
        assert!(score_theaters(&board).is_empty());

        board.place(0, BuildingType::Yellow);
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_theaters(&board), ans);

        board.place(1, BuildingType::Green);
        let ans = HashMap::from([(0, 1)]);
        assert_eq!(score_theaters(&board), ans);

        board.place(2, BuildingType::Black);
        let ans = HashMap::from([(0, 2)]);
        assert_eq!(score_theaters(&board), ans);

        board.place(3, BuildingType::Blue);
        let ans = HashMap::from([(0, 3)]);
        assert_eq!(score_theaters(&board), ans);

        board.place(4, BuildingType::Red);
        let ans = HashMap::from([(0, 4)]);
        assert_eq!(score_theaters(&board), ans);

        board.place(8, BuildingType::Gray);
        let ans = HashMap::from([(0, 5)]);
        assert_eq!(score_theaters(&board), ans);

        board.place(12, BuildingType::Yellow);
        let ans = HashMap::from([(0, 5), (12, 2)]);
        assert_eq!(score_theaters(&board), ans);

        board.remove(12);
        board.place(12, BuildingType::Magenta);
        let ans = HashMap::from([(0, 6)]);
        assert_eq!(score_theaters(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let mut board = Board::new(4, 4);
        board.place(1, BuildingType::Green);
        board.place(4, BuildingType::Black);
        board.place(5, BuildingType::Yellow);
        board.place(9, BuildingType::Yellow);
        board.place(13, BuildingType::Red);


        // Use bakery.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Bakery,
        );
        let ans = HashMap::from([(5, 3), (9, 3)]);
        assert_eq!(score(&board, &building_config), ans);

        // Use market.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Market,
        );
        let ans = HashMap::from([(5, 2), (9, 2)]);
        assert_eq!(score(&board, &building_config), ans);

        // Use tailor.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Tailor,
        );
        let ans = HashMap::from([(5, 3), (9, 3)]);
        assert_eq!(score(&board, &building_config), ans);

        // Use theater.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(5, 3), (9, 2)]);
        assert_eq!(score(&board, &building_config), ans);
    }
}
