use std::collections::HashMap;

use crate::game::space::BuildingType;
use crate::game::board::Board;
use crate::game::building::{BuildingConfig, GreenBuilding};
use crate::score::{score_by_count, score_per_each, ScoringContext};

// -----------------------------------------------------------------------------
fn score_feast_halls(
    board: &Board,
    scoring_context: &ScoringContext,
    other: &Board,
) -> HashMap<usize, i32> {
    let points = if board.count_building_type(BuildingType::Green)
        > other.count_building_type(BuildingType::Green)
    {
        scoring_context.points_per_feast_hall_with_greater_count
    } else {
        scoring_context.points_per_feast_hall_with_equal_or_lesser_count
    };

    let scores = score_per_each(board, BuildingType::Green, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_inns(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    // Count number of inns in each row and column.
    let (inns_in_row, inns_in_col) =
        board.count_building_type_per_row_and_col(BuildingType::Green);

    // Score only the inns that are alone in their column and row.
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut scores, (idx, space)| {
            if space.building_type_eq(BuildingType::Green) {
                let row = board.row(idx);
                let col = board.col(idx);
                let points = if inns_in_row.get(&row).copied().unwrap_or(0) == 1
                    && inns_in_col.get(&col).copied().unwrap_or(0) == 1
                {
                    scoring_context.points_per_inn
                } else {
                    0
                };
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
    other_opt: Option<&Board>,
) -> HashMap<usize, i32> {
    let score = match building_config.green() {
        GreenBuilding::Almshouse => score_by_count(
            board,
            BuildingType::Green,
            &scoring_context.points_by_count_for_almshouses,
            scoring_context.default_score_for_almshouses,
        ),
        GreenBuilding::FeastHall => {
            if let Some(other) = other_opt {
                score_feast_halls(board, scoring_context, other)
            } else {
                panic!("no second board provided to score feast halls");
            }
        }
        GreenBuilding::Inn => score_inns(board, scoring_context),
        GreenBuilding::Tavern => score_by_count(
            board,
            BuildingType::Green,
            &scoring_context.points_by_count_for_taverns,
            scoring_context.default_score_for_taverns,
        ),
    };

    score
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
    fn test_score_feast_halls() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        let mut other = Board::new(4, 4);
        assert!(score_feast_halls(&board, &scoring_context, &other).is_empty());

        board.place(0, BuildingType::Green);
        let expected = HashMap::from([(0, 3)]);
        assert_eq!(
            score_feast_halls(&board, &scoring_context, &other),
            expected
        );

        other.place(0, BuildingType::Green);
        let expected = HashMap::from([(0, 2)]);
        assert_eq!(
            score_feast_halls(&board, &scoring_context, &other),
            expected
        );

        other.place(1, BuildingType::Green);
        let expected = HashMap::from([(0, 2)]);
        assert_eq!(
            score_feast_halls(&board, &scoring_context, &other),
            expected
        );

        board.place(1, BuildingType::Green);
        let expected = HashMap::from([(0, 2), (1, 2)]);
        assert_eq!(
            score_feast_halls(&board, &scoring_context, &other),
            expected
        );

        board.place(2, BuildingType::Green);
        let expected = HashMap::from([(0, 3), (1, 3), (2, 3)]);
        assert_eq!(
            score_feast_halls(&board, &scoring_context, &other),
            expected
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_inns() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        assert!(score_inns(&board, &scoring_context).is_empty());

        board.place(0, BuildingType::Green);
        let expected = HashMap::from([(0, 3)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);

        board.place(1, BuildingType::Green);
        let expected = HashMap::from([(0, 0), (1, 0)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);

        board.place(5, BuildingType::Green);
        let expected = HashMap::from([(0, 0), (1, 0), (5, 0)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);

        board.remove(1);
        let expected = HashMap::from([(0, 3), (5, 3)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);

        board.place(10, BuildingType::Green);
        let expected = HashMap::from([(0, 3), (5, 3), (10, 3)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);

        board.place(12, BuildingType::Green);
        let expected = HashMap::from([(0, 0), (5, 3), (10, 3), (12, 0)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);

        board.remove(12);
        board.place(15, BuildingType::Green);
        let expected = HashMap::from([(0, 3), (5, 3), (10, 3), (15, 3)]);
        assert_eq!(score_inns(&board, &scoring_context), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let scoring_context = ScoringContext::default();
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
        let expected =
            HashMap::from([(0, -5), (5, 0), (10, 0), (12, 0), (15, 0)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, None),
            expected
        );

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
        let result = std::panic::catch_unwind(|| {
            score(&board, &building_config, &scoring_context, None)
        });
        assert!(result.is_err());

        let other = Board::new(4, 4);
        let expected =
            HashMap::from([(0, 3), (5, 3), (10, 3), (12, 3), (15, 3)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, Some(&other)),
            expected
        );

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
        let expected =
            HashMap::from([(0, 0), (5, 3), (10, 3), (12, 0), (15, 0)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, None),
            expected
        );

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
        let expected =
            HashMap::from([(0, 20), (5, 0), (10, 0), (12, 0), (15, 0)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, None),
            expected
        );
    }
}
