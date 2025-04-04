use std::collections::{HashMap, HashSet};

use crate::game::space::BuildingType;
use crate::game::board::Board;
use crate::game::piece::{BuildingConfig, MagentaBuilding, OrangeBuilding};
use crate::score::{score_if_not_adjacent_to, score_per_each, ScoringContext};

// -----------------------------------------------------------------------------
fn score_chapels(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let num_fed_blue_buildings =
        fed_idxs.iter().fold(0, |num_fed_blue_buildings, idx| {
            let space = &board.spaces()[*idx];
            if space.building_type_eq(BuildingType::Blue) {
                num_fed_blue_buildings + 1
            } else if space.building_type_eq(BuildingType::Magenta)
                && building_config.magenta() == MagentaBuilding::BarrettCastle
            {
                num_fed_blue_buildings + 2
            } else {
                num_fed_blue_buildings
            }
        });
    let points = num_fed_blue_buildings
        * scoring_context.points_per_fed_blue_building_for_chapels;
    let scores = score_per_each(board, BuildingType::Orange, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_cloisters(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let points = board.corner_idxs().into_iter().fold(0, |points, idx| {
        let space = &board.spaces()[idx];
        if space.building_type_eq(BuildingType::Orange) {
            points + scoring_context.points_per_cloister_in_corner
        } else {
            points
        }
    });

    let scores = score_per_each(board, BuildingType::Orange, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_temple(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    fed_idxs: &HashSet<usize>,
    idx: usize,
) -> bool {
    let score = board.adjacent_idxs(idx).intersection(fed_idxs).fold(
        0,
        |num_adjacent_blue_buildings, ii| {
            let space = &board.spaces()[*ii];
            if space.building_type_eq(BuildingType::Blue) {
                num_adjacent_blue_buildings + 1
            } else if space.building_type_eq(BuildingType::Magenta)
                && building_config.magenta() == MagentaBuilding::BarrettCastle
            {
                num_adjacent_blue_buildings
                    + scoring_context
                        .equivalent_num_of_blue_buildings_for_barrett_castle
            } else {
                num_adjacent_blue_buildings
            }
        },
    ) >= scoring_context
        .min_adjacent_blue_buildings_to_score_temple;

    score
}

// -----------------------------------------------------------------------------
fn score_temples(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut scores, (idx, space)| {
            if space.building_type_eq(BuildingType::Orange) {
                if score_temple(
                    board,
                    building_config,
                    scoring_context,
                    fed_idxs,
                    idx,
                ) {
                    scores.insert(idx, scoring_context.points_per_temple);
                } else {
                    scores.insert(idx, 0);
                }
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
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = match building_config.orange() {
        OrangeBuilding::Abbey => score_if_not_adjacent_to(
            board,
            BuildingType::Orange,
            &scoring_context.adjacent_building_types_for_abbeys,
            scoring_context.points_per_abbey,
        ),
        OrangeBuilding::Chapel => {
            score_chapels(board, building_config, scoring_context, fed_idxs)
        }
        OrangeBuilding::Cloister => score_cloisters(board, scoring_context),
        OrangeBuilding::Temple => {
            score_temples(board, building_config, scoring_context, fed_idxs)
        }
    };

    scores
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::game::piece::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_chapels() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        // Without Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut fed_idxs = HashSet::new();

        board.place(0, BuildingType::Orange);
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(1, BuildingType::Blue);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        fed_idxs.insert(1);
        let expected = HashMap::from([(0, 1)]);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        fed_idxs.insert(2);
        fed_idxs.insert(3);
        fed_idxs.insert(4);
        let expected = HashMap::from([(0, 4)]);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(5, BuildingType::Orange);
        let expected = HashMap::from([(0, 4), (5, 4)]);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(6, BuildingType::Magenta);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        fed_idxs.insert(6);
        let expected = HashMap::from([(0, 6), (5, 6)]);
        assert_eq!(
            score_chapels(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_cloisters() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);

        board.place(1, BuildingType::Orange);
        let expected = HashMap::from([(1, 0)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);

        board.remove(1);
        board.place(0, BuildingType::Orange);
        let expected = HashMap::from([(0, 1)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);

        board.place(1, BuildingType::Orange);
        let expected = HashMap::from([(0, 1), (1, 1)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);

        board.place(3, BuildingType::Orange);
        let expected = HashMap::from([(0, 2), (1, 2), (3, 2)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);

        board.place(12, BuildingType::Orange);
        let expected = HashMap::from([(0, 3), (1, 3), (3, 3), (12, 3)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);

        board.remove(1);
        board.place(15, BuildingType::Orange);
        let expected = HashMap::from([(0, 4), (3, 4), (12, 4), (15, 4)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);

        board.place(14, BuildingType::Orange);
        let expected =
            HashMap::from([(0, 4), (3, 4), (12, 4), (14, 4), (15, 4)]);
        assert_eq!(score_cloisters(&board, &scoring_context), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_temple() {
        let scoring_context = ScoringContext::default();
        // Without Barrett Castle.
        let mut board = Board::new(4, 4);
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut fed_idxs: HashSet<usize> = HashSet::new();
        board.place(0, BuildingType::Orange);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        board.place(1, BuildingType::Blue);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        fed_idxs.insert(1);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        board.place(2, BuildingType::Blue);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        fed_idxs.insert(2);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        board.place(4, BuildingType::Blue);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        fed_idxs.insert(4);
        assert!(score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            0
        ));

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );

        board.place(12, BuildingType::Orange);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            12
        ));

        board.place(8, BuildingType::Magenta);
        assert!(!score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            12
        ));

        fed_idxs.insert(8);
        assert!(score_temple(
            &board,
            &building_config,
            &scoring_context,
            &fed_idxs,
            12
        ));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_temples() {
        let scoring_context = ScoringContext::default();
        // Without Barrett Castle.
        let mut board = Board::new(4, 4);
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut fed_idxs: HashSet<usize> = HashSet::new();
        board.place(0, BuildingType::Orange);
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(1, BuildingType::Blue);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        fed_idxs.insert(1);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(2, BuildingType::Blue);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        fed_idxs.insert(2);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(4, BuildingType::Blue);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        fed_idxs.insert(4);
        let expected = HashMap::from([(0, 4)]);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        board.place(12, BuildingType::Orange);
        let expected = HashMap::from([(0, 4), (12, 0)]);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        board.place(8, BuildingType::Magenta);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        fed_idxs.insert(8);
        let expected = HashMap::from([(0, 4), (12, 4)]);
        assert_eq!(
            score_temples(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Orange);
        board.place(1, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        board.place(5, BuildingType::Orange);
        board.place(3, BuildingType::Orange);
        board.place(15, BuildingType::Red);
        board.place(7, BuildingType::Black);
        board.place(14, BuildingType::Blue);
        let fed_idxs = HashSet::from([1, 4, 14]);

        // Score with abbeys.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let expected = HashMap::from([(0, 3), (3, 0), (5, 3)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, &fed_idxs),
            expected
        );

        // Score with chapels.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let expected = HashMap::from([(0, 3), (3, 3), (5, 3)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, &fed_idxs),
            expected
        );

        // Score with cloisters.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Cloister,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let expected = HashMap::from([(0, 2), (3, 2), (5, 2)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, &fed_idxs),
            expected
        );

        // Score with temples.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Temple,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let expected = HashMap::from([(0, 4), (3, 0), (5, 4)]);
        assert_eq!(
            score(&board, &building_config, &scoring_context, &fed_idxs),
            expected
        );
    }
}
