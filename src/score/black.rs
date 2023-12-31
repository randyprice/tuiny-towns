use crate::board::Board;
use crate::building::{BlackBuilding, BuildingType};
use crate::building_config::BuildingConfig;
use crate::space::Space;

// -----------------------------------------------------------------------------
fn score_banks(board: &Board) -> i32 {
    let score = board.count_building_type(BuildingType::Black) as i32 * 4;

    score
}

// -----------------------------------------------------------------------------
fn score_factories() -> i32 {
    let score = 0;

    score
}

// -------------------------------------------------------------------------
fn score_trading_posts(board: &Board) -> i32 {
    let score = board.count_building_type(BuildingType::Black) as i32;

    score
}

// -------------------------------------------------------------------------
fn score_warehouses(board: &Board) -> i32 {
    let score = board.spaces()
        .iter()
        .fold(0, |n, space| match space {
            Space::BuildingWithResources(BuildingType::Black, resources, _)
                => n + resources.len(),
            _ => n,
        })
        as i32
        * -1;

    score
}

// -----------------------------------------------------------------------------
pub fn score(board: &Board, building_config: &BuildingConfig) -> i32 {
    let score = match building_config.black() {
        BlackBuilding::Bank => score_banks(board),
        BlackBuilding::Factory => score_factories(),
        BlackBuilding::TradingPost => score_trading_posts(board),
        BlackBuilding::Warehouse => score_warehouses(board),
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding};
    use crate::building::Resource;

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_banks() {
        let mut board = Board::new(4, 4);

        board.place(0, BuildingType::Blue);
        assert_eq!(score_banks(&board), 0);

        board.place(1, (BuildingType::Black, Resource::Brick));
        assert_eq!(score_banks(&board), 4);

        board.place(5, (BuildingType::Black, Resource::Glass));
        assert_eq!(score_banks(&board), 8);

    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_factories() {
        assert_eq!(score_factories(), 0);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_trading_posts() {
        let mut board = Board::new(4, 4);

        board.place(0, BuildingType::Blue);
        assert_eq!(score_trading_posts(&board), 0);

        board.place(1, BuildingType::Black);
        assert_eq!(score_trading_posts(&board), 1);

        board.place(5, BuildingType::Black);
        assert_eq!(score_trading_posts(&board), 2);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_warehouses() {
        let mut board = Board::new(4, 4);

        board.place(0, BuildingType::Blue);
        assert_eq!(score_warehouses(&board), 0);

        board.place(1, (BuildingType::Black, Vec::new(), 3));
        assert_eq!(score_warehouses(&board), 0);

        board.place(5, (BuildingType::Black, vec![Resource::Glass, Resource::Wood], 3));
        assert_eq!(score_warehouses(&board), -2);

        board.place(
            6,
            (
                BuildingType::Black,
                vec![Resource::Brick, Resource::Glass, Resource::Wood],
                3,
            ),
        );
        assert_eq!(score_warehouses(&board), -5);
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
        assert_eq!(score(&board, &building_config), 4);

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
        assert_eq!(score(&board, &building_config), 1);

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
        assert_eq!(score(&board, &building_config), 0);

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
        board.place(0, (BuildingType::Black, vec![Resource::Brick, Resource::Glass, Resource::Wheat], 3));
        assert_eq!(score(&board, &building_config), -3);
    }
}
