use std::collections::HashMap;

use crate::board::Board;
use crate::building::{BuildingType, GreenBuilding};
use crate::building_config::BuildingConfig;
use crate::score::{score_by_count, score_per_each};

// -----------------------------------------------------------------------------
fn score_almshouses(board: &Board) -> HashMap<usize, i32> {
    let points_by_count = HashMap::from([
        (0, 0),
        (1, -1),
        (2, 5),
        (3, -3),
        (4, 15),
        (5, -5),
    ]);

    let scores = score_by_count(
        board,
        BuildingType::Green,
        &points_by_count,
        26
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_feast_halls(board: &Board, other: &Board) -> HashMap<usize, i32> {
    let points =
        if board.count_building_type(BuildingType::Green)
        > other.count_building_type(BuildingType::Green) {
            3
        } else {
            2
        };

    let scores = score_per_each(board, BuildingType::Green, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_inns(board: &Board) -> HashMap<usize, i32> {
    // Count number of inns in each row and column.
    let (inns_in_row, inns_in_col) =
        board.count_building_type_per_row_and_col(BuildingType::Green);

    // Score only the inns that are alone in their column and row.
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(BuildingType::Green) {
                let row = board.row(idx);
                let col = board.col(idx);
                let points =
                    if inns_in_row.get(&row).copied().unwrap_or(0) == 1
                    && inns_in_col.get(&col).copied().unwrap_or(0) == 1 {
                        3
                    } else {
                        0
                    };
                m.insert(idx, points);
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
fn score_taverns(board: &Board) -> HashMap<usize, i32> {
    let points_by_count = HashMap::from([
        (0, 0),
        (1, 2),
        (2, 5),
        (3, 9),
        (4, 14),
    ]);

    let scores = score_by_count(
        board,
        BuildingType::Green,
        &points_by_count,
        20
    );

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    other_opt: Option<&Board>,
) -> HashMap<usize, i32> {
    let score = match building_config.green() {
        GreenBuilding::Almshouse => score_almshouses(board),
        GreenBuilding::FeastHall => if let Some(other) = other_opt {
            score_feast_halls(board, other)
        } else {
            panic!("no second board provided to score feast halls");
        },
        GreenBuilding::Inn => score_inns(board),
        GreenBuilding::Tavern => score_taverns(board)
    };

    score
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
    fn test_score_almshouses() {
        let mut board = Board::new(4, 4);
        assert!(score_almshouses(&board).is_empty());

        board.place(0, BuildingType::Green);
        let ans = HashMap::from([(0, -1)]);
        assert_eq!(score_almshouses(&board), ans);

        board.place(1, BuildingType::Green);
        let ans = HashMap::from([(0, 5), (1, 0)]);
        assert_eq!(score_almshouses(&board), ans);

        board.place(2, BuildingType::Green);
        let ans = HashMap::from([(0, -3), (1, 0), (2, 0)]);
        assert_eq!(score_almshouses(&board), ans);

        board.place(3, BuildingType::Green);
        let ans = HashMap::from([(0, 15), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(score_almshouses(&board), ans);

        board.place(4, BuildingType::Green);
        let ans = HashMap::from([(0, -5), (1, 0), (2, 0), (3, 0), (4, 0)]);
        assert_eq!(score_almshouses(&board), ans);

        board.place(5, BuildingType::Green);
        let ans = HashMap::from([
            (0, 26),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
        ]);
        assert_eq!(score_almshouses(&board), ans);

        board.place(6, BuildingType::Green);
        let ans = HashMap::from([
            (0, 26),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0),
            (6, 0),
        ]);
        assert_eq!(score_almshouses(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_feast_halls() {
        let mut board = Board::new(4, 4);
        let mut other = Board::new(4, 4);
        assert!(score_feast_halls(&board, &other).is_empty());

        board.place(0, BuildingType::Green);
        let ans = HashMap::from([(0, 3)]);
        assert_eq!(score_feast_halls(&board, &other), ans);

        other.place(0, BuildingType::Green);
        let ans = HashMap::from([(0, 2)]);
        assert_eq!(score_feast_halls(&board, &other), ans);

        other.place(1, BuildingType::Green);
        let ans = HashMap::from([(0, 2)]);
        assert_eq!(score_feast_halls(&board, &other), ans);

        board.place(1, BuildingType::Green);
        let ans = HashMap::from([(0, 2), (1, 2)]);
        assert_eq!(score_feast_halls(&board, &other), ans);

        board.place(2, BuildingType::Green);
        let ans = HashMap::from([(0, 3), (1, 3), (2, 3)]);
        assert_eq!(score_feast_halls(&board, &other), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_inns() {
        let mut board = Board::new(4, 4);
        assert!(score_inns(&board).is_empty());

        board.place(0, BuildingType::Green);
        let ans = HashMap::from([(0, 3)]);
        assert_eq!(score_inns(&board), ans);

        board.place(1, BuildingType::Green);
        let ans = HashMap::from([(0, 0), (1, 0)]);
        assert_eq!(score_inns(&board), ans);

        board.place(5, BuildingType::Green);
        let ans = HashMap::from([(0, 0), (1, 0), (5, 0)]);
        assert_eq!(score_inns(&board), ans);

        board.remove(1);
        let ans = HashMap::from([(0, 3), (5, 3)]);
        assert_eq!(score_inns(&board), ans);

        board.place(10, BuildingType::Green);
        let ans = HashMap::from([(0, 3), (5, 3), (10, 3)]);
        assert_eq!(score_inns(&board), ans);

        board.place(12, BuildingType::Green);
        let ans = HashMap::from([(0, 0), (5, 3), (10, 3), (12, 0)]);
        assert_eq!(score_inns(&board), ans);

        board.remove(12);
        board.place(15, BuildingType::Green);
        let ans = HashMap::from([(0, 3), (5, 3), (10, 3), (15, 3)]);
        assert_eq!(score_inns(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_taverns() {
        let mut board = Board::new(4, 4);
        assert!(score_taverns(&board).is_empty());

        board.place(0, BuildingType::Green);
        let ans = HashMap::from([(0, 2)]);
        assert_eq!(score_taverns(&board), ans);

        board.place(1, BuildingType::Green);
        let ans = HashMap::from([(0, 5), (1, 0)]);
        assert_eq!(score_taverns(&board), ans);

        board.place(2, BuildingType::Green);
        let ans = HashMap::from([(0, 9), (1, 0), (2, 0)]);
        assert_eq!(score_taverns(&board), ans);

        board.place(3, BuildingType::Green);
        let ans = HashMap::from([(0, 14), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(score_taverns(&board), ans);

        board.place(4, BuildingType::Green);
        let ans = HashMap::from([(0, 20), (1, 0), (2, 0), (3, 0), (4, 0)]);
        assert_eq!(score_taverns(&board), ans);

        board.place(5, BuildingType::Green);
        let ans = HashMap::from([
            (0, 20),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (5, 0)],
        );
        assert_eq!(score_taverns(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Green);
        board.place(5, BuildingType::Green);
        board.place(10, BuildingType::Green);
        board.place(15, BuildingType::Green);
        board.place(12, BuildingType::Green);

        // Use almshouse.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(0, -5), (5, 0), (10, 0), (12, 0), (15, 0)]);
        assert_eq!(score(&board, &building_config, None), ans);

        // Use feast hall.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::FeastHall,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let result = std::panic::catch_unwind(
            || score(&board, &building_config, None)
        );
        assert!(result.is_err());

        let other = Board::new(4, 4);
        let ans = HashMap::from([(0, 3), (5, 3), (10, 3), (12, 3), (15, 3)]);
        assert_eq!(score(&board, &building_config, Some(&other)), ans);

        // Use inn.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Inn,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(0, 0), (5, 3), (10, 3), (12, 0), (15, 0)]);
        assert_eq!(score(&board, &building_config, None), ans);

        // Use tavern.
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
        let ans = HashMap::from([(0, 20), (5, 0), (10, 0), (12, 0), (15, 0)]);
        assert_eq!(score(&board, &building_config, None), ans);
    }
}
