use std::collections::HashSet;

use crate::game::space::BuildingType;
use crate::game::board::Board;
use crate::game::building::BuildingConfig;
use crate::score::feed::feedable_idxs;

// -----------------------------------------------------------------------------
pub fn feed(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let (fed_rows, fed_cols) = board.spaces().iter().enumerate().fold(
        (HashSet::new(), HashSet::new()),
        |(mut fed_rows, mut fed_cols), (idx, space)| {
            if space.building_type_eq(BuildingType::Red) {
                fed_rows.insert(board.row(idx));
                fed_cols.insert(board.col(idx));
            }
            (fed_rows, fed_cols)
        },
    );

    let fed_idxs = feedable_idxs(board, building_config).into_iter().fold(
        HashSet::new(),
        |mut fed_idxs, idx| {
            if fed_rows.contains(&board.row(idx))
                || fed_cols.contains(&board.col(idx))
            {
                fed_idxs.insert(idx);
            }
            fed_idxs
        },
    );

    fed_idxs
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::game::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
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
            RedBuilding::Granary,
            YellowBuilding::Theater,
        );

        assert!(feed(&board, &building_config).is_empty());

        board.place(1, BuildingType::Blue);
        assert!(feed(&board, &building_config).is_empty());

        board.place(5, BuildingType::Red);
        assert_eq!(feed(&board, &building_config), HashSet::from([1]));

        board.place(4, BuildingType::Blue);
        board.place(6, BuildingType::Blue);
        board.place(9, BuildingType::Magenta);
        board.place(12, BuildingType::Blue);
        assert_eq!(feed(&board, &building_config), HashSet::from([1, 4, 6]));

        board.place(13, BuildingType::Red);
        let ans = HashSet::from([1, 4, 6, 12]);
        assert_eq!(feed(&board, &building_config), ans);

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Abbey,
            RedBuilding::Granary,
            YellowBuilding::Theater,
        );
        let ans = HashSet::from([1, 4, 6, 9, 12]);
        assert_eq!(feed(&board, &building_config), ans);
    }
}
