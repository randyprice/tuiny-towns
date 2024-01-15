use std::collections::{HashMap, HashSet};

use crate::board::space::BuildingType;
use crate::board::Board;
use crate::building_config::{BlueBuilding, BuildingConfig, MagentaBuilding};
use crate::score::{score_if_in_idx_set, score_per_each, ScoringContext};

// -----------------------------------------------------------------------------
fn score_cottages(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = if building_config.magenta()
        == MagentaBuilding::GrandMausoleumOfTheRodina
        && board.count_building_type(BuildingType::Magenta) > 0
    {
        score_per_each(
            board,
            BuildingType::Blue,
            scoring_context
                .points_per_cottage_with_grand_mausoleum_of_the_rodina,
        )
    } else {
        score_if_in_idx_set(
            board,
            fed_idxs,
            BuildingType::Blue,
            scoring_context.points_per_fed_cottage,
        )
    };

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let score = match building_config.blue() {
        BlueBuilding::Cottage => {
            score_cottages(board, building_config, scoring_context, fed_idxs)
        }
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::building_config::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_cottages() {
        let scoring_context = ScoringContext::default();
        // Without Mausoleum.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::ShrineOfTheElderTree,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut board = Board::new(5, 6);
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Blue);
        board.place(7, BuildingType::Green);
        board.place(8, BuildingType::Orange);
        board.place(9, BuildingType::Yellow);
        board.place(10, BuildingType::Gray);
        board.place(14, BuildingType::Blue);
        board.place(19, BuildingType::Black);
        board.place(21, BuildingType::Red);
        board.place(24, BuildingType::Magenta);
        let fed_idxs = HashSet::from([0, 1, 14]);
        let expected = HashMap::from([(0, 3), (1, 3), (14, 3)]);
        assert_eq!(
            score_cottages(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs,
            ),
            expected,
        );

        // Add another cottage without feeding it.
        board.place(11, BuildingType::Blue);
        let expected = HashMap::from([(0, 3), (1, 3), (11, 0), (14, 3)]);
        assert_eq!(
            score_cottages(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        // Feed the added cottage.
        let fed_idxs = HashSet::from([0, 1, 11, 14]);
        let expected = HashMap::from([(0, 3), (1, 3), (11, 3), (14, 3)]);
        assert_eq!(
            score_cottages(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );

        // With Mausoleum.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::GrandMausoleumOfTheRodina,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        // Feed only one cottage.
        let fed_idxs = HashSet::from([0]);
        assert_eq!(
            score_cottages(
                &board,
                &building_config,
                &scoring_context,
                &fed_idxs
            ),
            expected
        );
    }
}
