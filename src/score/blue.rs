use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::building::{BlueBuilding, BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;
use crate::score::{score_if_in_set, score_per_each};

// -----------------------------------------------------------------------------
fn score_cottages(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores =
        if building_config.magenta() == MagentaBuilding::GrandMausoleumOfTheRodina
        && board.count_building_type(BuildingType::Magenta) > 0 {
            score_per_each(board, BuildingType::Blue, 3)
        } else {
            score_if_in_set(board, fed_idxs, BuildingType::Blue, 3)
        };

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let score = match building_config.blue() {
        BlueBuilding::Cottage => {
            score_cottages(board, building_config, fed_idxs)
        }
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
    fn test_score_cottages() {
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
        let ans = HashMap::from([(0, 3), (1, 3), (14, 3)]);
        assert_eq!(score_cottages(&board, &building_config, &fed_idxs), ans);

        // Add another cottage without feeding it.
        board.place(11, BuildingType::Blue);
        let ans = HashMap::from([(0, 3), (1, 3), (11, 0), (14, 3)]);
        assert_eq!(score_cottages(&board, &building_config, &fed_idxs), ans);

        // Feed the added cottage.
        let fed_idxs = HashSet::from([0, 1, 11, 14]);
        let ans = HashMap::from([(0, 3), (1, 3), (11, 3), (14, 3)]);
        assert_eq!(score_cottages(&board, &building_config, &fed_idxs), ans);

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
        assert_eq!(score_cottages(&board, &building_config, &fed_idxs), ans);
    }
}