use std::collections::HashSet;

use crate::board::{BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
    MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding};
use crate::board::{Board, Build, Building, BuildingConfig, BuildingType};
use crate::feed::feed;

// -----------------------------------------------------------------------------
fn score_unfed_cottages(board: &Board, building_config: &BuildingConfig) -> bool {
    building_config.magenta() == MagentaBuilding::GrandMausoleumOfTheRodina
        && board.count_building_type(BuildingType::Magenta) > 0
}

// -----------------------------------------------------------------------------
pub fn score_cottages(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score_unfed_cottages = score_unfed_cottages(board, building_config);
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |mut n, (idx, space_opt)| {
            if let Some(space) = space_opt {
                if let Some(building_type) = space.building_type() {
                    if building_type == BuildingType::Blue
                        && (fed_idxs.contains(&idx)
                            || score_unfed_cottages) {
                        n += 3;
                    }
                }
            }
            n
        });

    score
}

// -----------------------------------------------------------------------------
pub fn score_blue(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score = match building_config.blue() {
        BlueBuilding::Cottage => score_cottages(board, building_config, fed_idxs),
    };

    score
}

// -----------------------------------------------------------------------------
pub fn score_magenta(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    0
}

// -----------------------------------------------------------------------------
pub fn score_orange(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score = match building_config.orange() {
        OrangeBuilding::Abbey => score_abbeys(board),
        // OrangeBuilding::Chapel => score_chapels(board, fed_buildings),
        // OrangeBuilding::Cloister => score_cloisters(board),
        // OrangeBuilding::Temple => score_temples(board, fed_buildings),
        _ => 0
    };

    score
}

// -----------------------------------------------------------------------------
pub fn score(board: &Board, building_config: &BuildingConfig) -> i32 {
    let fed_buildings = feed(board, building_config);
    let score = -1;
    println!("Total score: {score}");

    score
}

// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

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