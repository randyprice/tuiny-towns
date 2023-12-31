use std::collections::HashSet;

use crate::board::Board;
use crate::building::{BlueBuilding, BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;

// -----------------------------------------------------------------------------
fn score_unfed_cottages(
    board: &Board,
    building_config: &BuildingConfig,
) -> bool {
    let score =
        building_config.magenta() == MagentaBuilding::GrandMausoleumOfTheRodina
        && board.count_building_type(BuildingType::Magenta) > 0;

    score
}

// -----------------------------------------------------------------------------
fn score_cottages(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> i32 {
    let score_unfed = score_unfed_cottages(board, building_config);
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |n, (idx, space)|
            if space.building_type_eq(BuildingType::Blue)
                && (fed_idxs.contains(&idx) || score_unfed) {
                n + 3
            } else {
                n
            }
        );

    score
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> i32 {
    let score = match building_config.blue() {
        BlueBuilding::Cottage => score_cottages(board, building_config, fed_idxs),
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
    fn test_score_unfed_cottages() {
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
        board.place(0, BuildingType::Magenta);
        assert!(!score_unfed_cottages(&board, &building_config));

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
        board.place(0, BuildingType::Magenta);
        assert!(score_unfed_cottages(&board, &building_config));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_cottages() {
        // Without Mausoleum or Barrett Castle.
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
        let score = score_cottages(&board, &building_config, &fed_idxs);
        assert_eq!(score, 9);

        // Add another cottage without feeding it.
        board.place(11, BuildingType::Blue);
        let score = score_cottages(&board, &building_config, &fed_idxs);
        assert_eq!(score, 9);

        // Feed the added cottage.
        let fed_idxs = HashSet::from([0, 1, 11, 14]);
        let score = score_cottages(&board, &building_config, &fed_idxs);
        assert_eq!(score, 12);

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
        let score = score_cottages(&board, &building_config, &fed_idxs);
        assert_eq!(score, 12);

    }
}