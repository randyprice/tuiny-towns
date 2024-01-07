use std::collections::HashMap;

use crate::board::Board;
use crate::board::space::BuildingType;
use crate::building_config::{BlackBuilding, BuildingConfig};
use crate::score::score_per_each;

// -------------------------------------------------------------------------
fn score_warehouses(board: &Board) -> HashMap<usize, i32> {
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut scores, (idx, space)| {
            if let Some(resources) = space.resources() {
                scores.insert(idx, resources.len() as i32 * -1);
            }
            scores
        });

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig
) -> HashMap<usize, i32> {
    let scores = match building_config.black() {
        BlackBuilding::Bank => score_per_each(board, BuildingType::Black, 4),
        BlackBuilding::Factory => score_per_each(board, BuildingType::Black, 0),
        BlackBuilding::TradingPost => {
            score_per_each(board, BuildingType::Black, 1)
        }
        BlackBuilding::Warehouse => score_warehouses(board),
    };

    scores
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::board::space::Resource;
    use crate::building_config::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding};

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_warehouses() {
        let mut board = Board::new(4, 4);

        board.place(0, BuildingType::Blue);
        assert!(score_warehouses(&board).is_empty());

        board.place(1, (BuildingType::Black, Vec::new(), 3));
        let ans = HashMap::from([(1, 0)]);
        assert_eq!(score_warehouses(&board), ans);

        board.place(5, (BuildingType::Black, vec![Resource::Glass, Resource::Wood], 3));
        let ans = HashMap::from([(1, 0), (5, -2)]);
        assert_eq!(score_warehouses(&board), ans);

        board.place(
            6,
            (
                BuildingType::Black,
                vec![Resource::Brick, Resource::Glass, Resource::Wood],
                3,
            ),
        );
        let ans = HashMap::from([(1, 0), (5, -2), (6, -3)]);
        assert_eq!(score_warehouses(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let mut board = Board::new(4, 4);

        // Score with banks.
        let building_config = BuildingConfig::new(
            BlackBuilding::Bank,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Temple,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        board.place(0, BuildingType::Black);
        let ans = HashMap::from([(0, 4)]);
        assert_eq!(score(&board, &building_config), ans);

        // Score with trading posts.
        let building_config = BuildingConfig::new(
            BlackBuilding::TradingPost,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Temple,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(0, 1)]);
        assert_eq!(score(&board, &building_config), ans);

        // Score with factories.
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
        board.place(0, (BuildingType::Black, Resource::Brick));
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score(&board, &building_config), ans);

        // Score with warehouses.
        let building_config = BuildingConfig::new(
            BlackBuilding::Warehouse,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Temple,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        board.place(
            0,
            (BuildingType::Black, vec![Resource::Brick, Resource::Glass, Resource::Wheat], 3)
        );
        let ans = HashMap::from([(0, -3)]);
        assert_eq!(score(&board, &building_config), ans);
    }
}
