use std::collections::HashSet;

use crate::board::Board;
use crate::building_config::BuildingConfig;
use crate::building::BuildingType;
use crate::feed::feedable_idxs;

// -----------------------------------------------------------------------------
pub fn feed(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let fed_idxs = feedable_idxs(board, building_config)
        .into_iter()
        .fold(HashSet::new(), |mut fed_idxs, idx| {
            if board.unique_surrounding_building_types(idx)
                .contains(&BuildingType::Red) {
                    fed_idxs.insert(idx);
            }
            fed_idxs
        });

    fed_idxs
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
    fn test_feed() {

        let mut board = Board::new(4, 4);
        // Without Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Orchard,
            YellowBuilding::Theater,
        );
        assert!(feed(&board, &building_config).is_empty());

        board.place(1, BuildingType::Blue);
        assert!(feed(&board, &building_config).is_empty());

        board.place(5, BuildingType::Red);
        assert_eq!(feed(&board, &building_config), HashSet::from([1]));

        board.place(0, BuildingType::Blue);
        board.place(2, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        board.place(6, BuildingType::Magenta);
        board.place(8, BuildingType::Blue);
        board.place(9, BuildingType::Blue);
        board.place(10, BuildingType::Blue);
        board.place(11, BuildingType::Blue);

        let ans = HashSet::from([0, 1, 2, 4, 8, 9, 10]);
        assert_eq!(feed(&board, &building_config), ans);

        board.place(14, BuildingType::Red);
        let ans = HashSet::from([0, 1, 2, 4, 8, 9, 10, 11]);
        assert_eq!(feed(&board, &building_config), ans);

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Abbey,
            RedBuilding::Orchard,
            YellowBuilding::Theater,
        );

        let ans = HashSet::from([0, 1, 2, 4, 6, 8, 9, 10, 11]);
        assert_eq!(feed(&board, &building_config), ans);
    }
}