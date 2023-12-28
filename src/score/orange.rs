use std::collections::HashSet;

use crate::board::{OrangeBuilding, MagentaBuilding};
use crate::board::{Board, BuildingConfig, BuildingType};
use crate::score::score_by_adjacency;

// -----------------------------------------------------------------------------
fn score_abbeys(board: &Board) -> i32 {
    let adjacent_types = HashSet::from([
        BuildingType::Black,
        BuildingType::Green,
        BuildingType:: Yellow,
    ]);
    let score = score_by_adjacency(
        false, board, BuildingType::Orange, adjacent_types, 3);

    score
}

// -----------------------------------------------------------------------------
fn score_chapels(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score = fed_idxs
        .iter()
        .fold(0, |mut n, idx| {
            let space = &board.spaces()[*idx];
            if let Some(building_type) = space.building_type() {
                if building_type == BuildingType::Blue {
                    n += 1;
                } else if building_type == BuildingType::Magenta
                    && building_config.magenta() == MagentaBuilding::BarrettCastle {
                    n += 2;
                }
            }
            n
        })
        * board.count_building_type(BuildingType::Orange) as i32;

    score
}

// -----------------------------------------------------------------------------
fn score_cloisters(board: &Board) -> i32 {
    let corners = board.corner_idxs();
    let (cloisters, corner_cloisters) = board.spaces()
        .iter()
        .enumerate()
        .fold((0, 0), |(mut n, mut m), (idx, space)| {
            if let Some(building_type) = space.building_type() {
                if building_type == BuildingType::Orange {
                    n += 1;
                    if corners.contains(&idx) {
                        m += 1;
                    }
                }
            }
            (n, m)
        });

    let score = cloisters * corner_cloisters as i32;

    score
}
// -----------------------------------------------------------------------------
fn score_temple(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>, idx: usize) -> bool {
    let score = board.adjacent_idxs(idx)
        .intersection(fed_idxs)
        .fold(0, |mut n, ii| {
            let space = &board.spaces()[*ii];
            if let Some(building_type) = space.building_type() {
                if building_type == BuildingType::Blue {
                    n += 1;
                } else if building_type == BuildingType::Magenta
                    && building_config.magenta() == MagentaBuilding::BarrettCastle {
                    n += 2;
                }
            }
            n
        })
        >= 2;

    score
}

// -----------------------------------------------------------------------------
fn score_temples(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |mut n, (idx, space)| {
            if let Some(building_type) = space.building_type() {
                if building_type == BuildingType::Orange
                    && score_temple(board, building_config, fed_idxs, idx) {
                    n += 4;
                }
            }
            n
        });

    score
}

// -----------------------------------------------------------------------------
pub fn score_orange(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score = match building_config.orange() {
        OrangeBuilding::Abbey => score_abbeys(board),
        OrangeBuilding::Chapel => score_chapels(board, building_config, fed_idxs),
        OrangeBuilding::Cloister => score_cloisters(board),
        OrangeBuilding::Temple => score_temples(board, building_config, fed_idxs),
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_abbeys() {
        let mut board = Board::new(4, 4);
        // Adjacent to black - does not score.
        board.place(0, BuildingType::Orange);
        board.place(1, BuildingType::Black);
        board.place(2, BuildingType::Blue);
        // Adjacent to blue and orange - scores.
        board.place(3, BuildingType::Orange);
        // Adjacent to green and orange - does not score.
        board.place(4, BuildingType::Orange);
        board.place(5, BuildingType::Green);
        board.place(6, BuildingType::Yellow);
        // Adjacent to yellow and orange - does not score.
        board.place(7, BuildingType::Orange);
        // Adjacent to orange and gray - scores.
        board.place(8, BuildingType::Orange);
        board.place(9, BuildingType::Magenta);
        board.place(12, BuildingType::Gray);

        assert_eq!(score_abbeys(&board), 6);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_chapels() {
        let mut board = Board::new(4, 4);
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut fed_idxs = HashSet::new();

        board.place(0, BuildingType::Orange);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 0);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 0);

        fed_idxs.insert(1);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 1);

        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        fed_idxs.insert(2);
        fed_idxs.insert(3);
        fed_idxs.insert(4);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 4);

        board.place(5, BuildingType::Orange);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 8);

        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 8);

        board.place(6, BuildingType::Magenta);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 8);

        fed_idxs.insert(6);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), 12);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_cloisters() {
        let mut board = Board::new(4, 4);

        board.place(1, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 0);

        board.remove(1);
        board.place(0, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 1);

        board.place(1, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 2);

        board.place(3, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 6);

        board.place(12, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 12);

        board.remove(1);
        board.place(15, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 16);

        board.place(14, BuildingType::Orange);
        assert_eq!(score_cloisters(&board), 20);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_temple() {
        // Without Barrett Castle.
        let mut board = Board::new(4, 4);
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut fed_idxs: HashSet<usize> = HashSet::new();
        board.place(0, BuildingType::Orange);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 0));

        board.place(1, BuildingType::Blue);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 0));

        fed_idxs.insert(1);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 0));

        board.place(2, BuildingType::Blue);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 0));

        fed_idxs.insert(2);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 0));

        board.place(4, BuildingType::Blue);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 0));

        fed_idxs.insert(4);
        assert!(score_temple(&board, &building_config, &fed_idxs, 0));

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );

        board.place(12, BuildingType::Orange);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 12));

        board.place(8, BuildingType::Magenta);
        assert!(!score_temple(&board, &building_config, &fed_idxs, 12));

        fed_idxs.insert(8);
        assert!(score_temple(&board, &building_config, &fed_idxs, 12));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_temples() {
        // Without Barrett Castle.
        let mut board = Board::new(4, 4);
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut fed_idxs: HashSet<usize> = HashSet::new();
        board.place(0, BuildingType::Orange);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 0);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 0);

        fed_idxs.insert(1);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 0);

        board.place(2, BuildingType::Blue);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 0);

        fed_idxs.insert(2);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 0);

        board.place(4, BuildingType::Blue);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 0);

        fed_idxs.insert(4);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 4);

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );

        board.place(12, BuildingType::Orange);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 4);

        board.place(8, BuildingType::Magenta);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 4);

        fed_idxs.insert(8);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), 8);
    }
}