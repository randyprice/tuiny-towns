use std::collections::HashSet;

use crate::board::Board;
use crate::building::{BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;
use crate::feed::feed;
use crate::space::Space;

pub mod black;
pub mod blue;
pub mod gray;
pub mod green;
pub mod magenta;
pub mod orange;
pub mod yellow;

// -----------------------------------------------------------------------------
fn score_by_adjacency(
    score: bool,
    board: &Board,
    building_type: BuildingType,
    adjacent_types: HashSet<BuildingType>,
    points: i32,
) -> i32 {
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |n, (idx, space)|
            if space.building_type_eq(building_type)
            && board.unique_adjacent_building_types(idx)
                .into_iter()
                .any(|bty| adjacent_types.contains(&bty))
                == score {
                n + points
            } else {
                n
            }
        );

    score
}

// -----------------------------------------------------------------------------
fn score_unused_spaces(board: &Board, building_config: &BuildingConfig) -> i32 {
    let score = if building_config.magenta() == MagentaBuilding::CathedralOfCaterina {
        0
    } else {
        board.spaces()
            .iter()
            .fold(0, |n, space| match space {
                Space::Resource(_) | Space::Empty => n - 1,
                _ => n,
            })
    };

    score
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    other: &Board,
) -> i32 {
    let fed_idxs = feed(board, building_config);
    let score = black::score(board, building_config)
        + blue::score(board, building_config, &fed_idxs)
        + gray::score(board, building_config)
        + green::score(board, building_config, other)
        + orange::score(board, building_config, &fed_idxs)
        + yellow::score(board, building_config)
        + score_unused_spaces(board, building_config);

    println!("total score: {score}");

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, Resource, YellowBuilding
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_by_adjacency() {
        let mut board = Board::new(4, 4);
        assert_eq!(
            score_by_adjacency(
                true,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            0,
        );
        board.place(0, BuildingType::Blue);
        assert_eq!(
            score_by_adjacency(
                true,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            0,
        );
        assert_eq!(
            score_by_adjacency(
                false,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            2,
        );

        board.place(4, BuildingType::Orange);
        assert_eq!(
            score_by_adjacency(
                true,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            2,
        );
        assert_eq!(
            score_by_adjacency(
                false,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            0,
        );

        board.place(4, BuildingType::Yellow);
        assert_eq!(
            score_by_adjacency(
                true,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            2,
        );
        assert_eq!(
            score_by_adjacency(
                false,
                &board,
                BuildingType::Blue,
                HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
                2),
            0,
        );

    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_unused_spaces() {
        let mut board = Board::new(4, 4);

        // Without Cathedral of Caterina.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Millstone,
            GreenBuilding::Tavern,
            MagentaBuilding::OpaleyesWatch,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        assert_eq!(score_unused_spaces(&board, &building_config), -16);

        board.place(0, Resource::Brick);
        assert_eq!(score_unused_spaces(&board, &building_config), -16);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_unused_spaces(&board, &building_config), -15);

        // With Cathedral of Caterina.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Millstone,
            GreenBuilding::Tavern,
            MagentaBuilding::CathedralOfCaterina,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        assert_eq!(score_unused_spaces(&board, &building_config), 0);
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }

}
