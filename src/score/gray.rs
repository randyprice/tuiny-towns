use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::board::space::BuildingType;
use crate::building_config::{BuildingConfig, GrayBuilding};
use crate::score::{score_if_adjacent_to, score_if_in_idx_set, score_per_each};

// -----------------------------------------------------------------------------
fn score_fountains(board: &Board) -> HashMap<usize, i32> {
    let scoring_idxs: HashSet<usize> =
        board.contiguous_groups(&HashSet::from([BuildingType::Gray]))
            .into_iter()
            .filter(|group| group.len() > 1)
            .flatten()
            .collect();

    let scores = score_if_in_idx_set(
        board,
        &scoring_idxs,
        BuildingType::Gray,
        2,
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_millstones(board: &Board) -> HashMap<usize, i32> {
    let scores = score_if_adjacent_to(
        board,
        BuildingType::Gray,
        &HashSet::from([BuildingType::Red, BuildingType::Yellow]),
        2,
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_wells(board: &Board) -> HashMap<usize, i32> {
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(BuildingType::Gray) {
                let count = board.count_adjacent_buildings(
                    idx,
                    &HashSet::from([BuildingType::Blue])
                );
                m.insert(idx, count as i32);
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
) -> HashMap<usize, i32> {
    let scores = match building_config.gray() {
        GrayBuilding::Fountain => score_fountains(board),
        GrayBuilding::Millstone => score_millstones(board),
        GrayBuilding::Shed => score_per_each(board, BuildingType::Gray, 1),
        GrayBuilding::Well => score_wells(board),
    };

    scores
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::building_config::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_fountains() {
        let mut board = Board::new(4, 4);
        assert!(score_fountains(&board).is_empty());

        board.place(0, BuildingType::Gray);
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_fountains(&board), ans);

        board.place(1, BuildingType::Gray);
        let ans = HashMap::from([(0, 2), (1, 2)]);
        assert_eq!(score_fountains(&board), ans);

        board.place(3, BuildingType::Gray);
        let ans = HashMap::from([(0, 2), (1, 2), (3, 0)]);
        assert_eq!(score_fountains(&board), ans);

        board.place(11, BuildingType::Gray);
        let ans = HashMap::from([(0, 2), (1, 2), (3, 0), (11, 0)]);
        assert_eq!(score_fountains(&board), ans);

        board.place(7, BuildingType::Gray);
        let ans = HashMap::from([(0, 2), (1, 2), (3, 2), (11, 2), (7, 2)]);
        assert_eq!(score_fountains(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_millstones() {
        let mut board = Board::new(4, 4);
        assert!(score_millstones(&board).is_empty());

        board.place(0, BuildingType::Gray);
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_millstones(&board), ans);

        board.place(4, BuildingType::Blue);
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_millstones(&board), ans);

        board.place(1, BuildingType::Red);
        let ans = HashMap::from([(0, 2)]);
        assert_eq!(score_millstones(&board), ans);

        board.place(3, BuildingType::Gray);
        let ans = HashMap::from([(0, 2), (3, 0)]);
        assert_eq!(score_millstones(&board), ans);

        board.place(7, BuildingType::Yellow);
        let ans = HashMap::from([(0, 2), (3, 2)]);
        assert_eq!(score_millstones(&board), ans);

        board.place(2, BuildingType::Red);
        let ans = HashMap::from([(0, 2), (3, 2)]);
        assert_eq!(score_millstones(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_wells() {
        let mut board = Board::new(4, 4);
        assert!(score_wells(&board).is_empty());

        board.place(5, BuildingType::Gray);
        let ans = HashMap::from([(5, 0)]);
        assert_eq!(score_wells(&board), ans);

        board.place(1, BuildingType::Blue);
        let ans = HashMap::from([(5, 1)]);
        assert_eq!(score_wells(&board), ans);

        board.place(4, BuildingType::Blue);
        let ans = HashMap::from([(5, 2)]);
        assert_eq!(score_wells(&board), ans);

        board.place(6, BuildingType::Blue);
        let ans = HashMap::from([(5, 3)]);
        assert_eq!(score_wells(&board), ans);

        board.place(9, BuildingType::Blue);
        let ans = HashMap::from([(5, 4)]);
        assert_eq!(score_wells(&board), ans);

        board.place(2, BuildingType::Gray);
        let ans = HashMap::from([(5, 4), (2, 2)]);
        assert_eq!(score_wells(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Gray);
        board.place(1, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        board.place(15, BuildingType::Red);
        board.place(14, BuildingType::Gray);
        board.place(13, BuildingType::Gray);

        // Use fountain.
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
        let ans = HashMap::from([(0, 0), (13, 2), (14, 2)]);
        assert_eq!(score(&board, &building_config), ans);

        // Use millstone.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Millstone,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(0, 0), (13, 0), (14, 2)]);
        assert_eq!(score(&board, &building_config), ans);

        // Use shed.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Shed,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(0, 1), (13, 1), (14, 1)]);
        assert_eq!(score(&board, &building_config), ans);

        // Use well.
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
        let ans = HashMap::from([(0, 2), (13, 0), (14, 0)]);
        assert_eq!(score(&board, &building_config), ans);
    }
}
