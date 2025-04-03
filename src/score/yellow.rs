use std::cmp;
use std::collections::{HashMap, HashSet};

use crate::game::space::BuildingType;
use crate::game::board::Board;
use crate::game::building::{BuildingConfig, YellowBuilding};
use crate::score::{score_if_adjacent_to, score_per_each, ScoringContext};

// -----------------------------------------------------------------------------
fn score_markets(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    // Count the number of markets in each row and column.
    let (markets_per_row, markets_per_col) =
        board.count_building_type_per_row_and_col(BuildingType::Yellow);

    // Score each market.
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut scores, (idx, space)| {
            if space.building_type_eq(BuildingType::Yellow) {
                let points = cmp::max(
                    markets_per_row.get(&board.row(idx)).copied().unwrap_or(0),
                    markets_per_col.get(&board.col(idx)).copied().unwrap_or(0),
                ) as i32
                    * scoring_context.points_per_yellow_building_for_markets;
                scores.insert(idx, points);
            }
            scores
        },
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_tailors(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let num_tailors_in_center =
        board.center_idxs().into_iter().fold(0, |n, idx| {
            let space = &board.spaces()[idx];
            if space.building_type_eq(BuildingType::Yellow) {
                n + 1
            } else {
                n
            }
        });
    let points = scoring_context.points_per_tailor_in_center
        * num_tailors_in_center
        + scoring_context.base_points_per_tailor;
    let scores = score_per_each(board, BuildingType::Yellow, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_theaters(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    // Create sets of unique building types in each row and column.
    let (unique_building_types_per_row, unique_building_types_per_col) =
        board.spaces().iter().enumerate().fold(
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
            },
        );

    // Score each theater.
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut scores, (idx, space)| {
            if space.building_type_eq(BuildingType::Yellow) {
                let empty_set = HashSet::new();
                let unique_building_types_in_row =
                    unique_building_types_per_row
                        .get(&board.row(idx))
                        .unwrap_or(&empty_set);
                let unique_building_types_in_col =
                    unique_building_types_per_col
                        .get(&board.col(idx))
                        .unwrap_or(&empty_set);
                let points = unique_building_types_in_row
                    .union(unique_building_types_in_col)
                    .count() as i32
                    * scoring_context.points_per_unique_type_for_theaters;
                scores.insert(idx, points);
            }
            scores
        },
    );

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let scores = match building_config.yellow() {
        YellowBuilding::Bakery => score_if_adjacent_to(
            board,
            BuildingType::Yellow,
            &scoring_context.adjacent_building_types_for_bakeries,
            scoring_context.points_per_bakery,
        ),
        YellowBuilding::Market => score_markets(board, scoring_context),
        YellowBuilding::Tailor => score_tailors(board, scoring_context),
        YellowBuilding::Theater => score_theaters(board, scoring_context),
    };

    scores
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::game::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_markets() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        assert!(score_markets(&board, &scoring_context).is_empty());

        board.place(0, BuildingType::Yellow);
        let expected = HashMap::from([(0, 1)]);
        assert_eq!(score_markets(&board, &scoring_context), expected);

        board.place(15, BuildingType::Yellow);
        let expected = HashMap::from([(0, 1), (15, 1)]);
        assert_eq!(score_markets(&board, &scoring_context), expected);

        board.place(1, BuildingType::Yellow);
        let expected = HashMap::from([(0, 2), (1, 2), (15, 1)]);
        assert_eq!(score_markets(&board, &scoring_context), expected);

        board.place(4, BuildingType::Yellow);
        let expected = HashMap::from([(0, 2), (1, 2), (4, 2), (15, 1)]);
        assert_eq!(score_markets(&board, &scoring_context), expected);

        board.place(2, BuildingType::Yellow);
        let expected = HashMap::from([(0, 3), (1, 3), (2, 3), (4, 2), (15, 1)]);
        assert_eq!(score_markets(&board, &scoring_context), expected);

        board.place(8, BuildingType::Yellow);
        let expected =
            HashMap::from([(0, 3), (1, 3), (2, 3), (4, 3), (8, 3), (15, 1)]);
        assert_eq!(score_markets(&board, &scoring_context), expected);

        board.place(12, BuildingType::Yellow);
        let expected = HashMap::from([
            (0, 4),
            (1, 3),
            (2, 3),
            (4, 4),
            (8, 4),
            (12, 4),
            (15, 2),
        ]);
        assert_eq!(score_markets(&board, &scoring_context), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_tailors() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        assert!(score_tailors(&board, &scoring_context).is_empty());

        board.place(0, BuildingType::Yellow);
        let expected = HashMap::from([(0, 1)]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);

        board.place(5, BuildingType::Yellow);
        let expected = HashMap::from([(0, 2), (5, 2)]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);

        board.place(1, BuildingType::Yellow);
        let expected = HashMap::from([(0, 2), (1, 2), (5, 2)]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);

        board.place(6, BuildingType::Yellow);
        let expected = HashMap::from([(0, 3), (1, 3), (5, 3), (6, 3)]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);

        board.place(9, BuildingType::Yellow);
        let expected = HashMap::from([(0, 4), (1, 4), (5, 4), (6, 4), (9, 4)]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);

        board.place(10, BuildingType::Yellow);
        let expected =
            HashMap::from([(0, 5), (1, 5), (5, 5), (6, 5), (9, 5), (10, 5)]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);

        board.place(11, BuildingType::Yellow);
        let expected = HashMap::from([
            (0, 5),
            (1, 5),
            (5, 5),
            (6, 5),
            (9, 5),
            (10, 5),
            (11, 5),
        ]);
        assert_eq!(score_tailors(&board, &scoring_context), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_theaters() {
        let scoring_context = ScoringContext::default();

        let mut board = Board::new(4, 4);
        assert!(score_theaters(&board, &scoring_context).is_empty());

        board.place(0, BuildingType::Yellow);
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.place(1, BuildingType::Green);
        let expected = HashMap::from([(0, 1)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.place(2, BuildingType::Black);
        let expected = HashMap::from([(0, 2)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.place(3, BuildingType::Blue);
        let expected = HashMap::from([(0, 3)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.place(4, BuildingType::Red);
        let expected = HashMap::from([(0, 4)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.place(8, BuildingType::Gray);
        let expected = HashMap::from([(0, 5)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.place(12, BuildingType::Yellow);
        let expected = HashMap::from([(0, 5), (12, 2)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);

        board.remove(12);
        board.place(12, BuildingType::Magenta);
        let expected = HashMap::from([(0, 6)]);
        assert_eq!(score_theaters(&board, &scoring_context), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let scoring_context = ScoringContext::default();
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
        let expected = HashMap::from([(5, 3), (9, 3)]);
        assert_eq!(score(&board, &building_config, &scoring_context), expected);

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
        let expected = HashMap::from([(5, 2), (9, 2)]);
        assert_eq!(score(&board, &building_config, &scoring_context), expected);

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
        let expected = HashMap::from([(5, 3), (9, 3)]);
        assert_eq!(score(&board, &building_config, &scoring_context), expected);

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
        let expected = HashMap::from([(5, 3), (9, 2)]);
        assert_eq!(score(&board, &building_config, &scoring_context), expected);
    }
}
