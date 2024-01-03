use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::building::{BuildingType, MagentaBuilding, OrangeBuilding};
use crate::building_config::BuildingConfig;
use crate::score::{score_if_not_adjacent_to, score_per_each};

// -----------------------------------------------------------------------------
fn score_abbeys(board: &Board) -> HashMap<usize, i32> {
    let adjacent_types = HashSet::from([
        BuildingType::Black,
        BuildingType::Green,
        BuildingType:: Yellow,
    ]);
    let scores = score_if_not_adjacent_to(
        board,
        BuildingType::Orange,
        &adjacent_types,
        3,
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_chapels(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let points = fed_idxs
        .iter()
        .fold(0, |n, idx| {
            let space = &board.spaces()[*idx];
            if space.building_type_eq(BuildingType::Blue) {
                n + 1
            } else if space.building_type_eq(BuildingType::Magenta)
                && building_config.magenta() == MagentaBuilding::BarrettCastle {
                n + 2
            } else {
                n
            }
        });

    let scores = score_per_each(board, BuildingType::Orange, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_cloisters(board: &Board) -> HashMap<usize, i32> {
    let points = board.corner_idxs()
        .into_iter()
        .fold(0, |n, idx| {
            let space = &board.spaces()[idx];
            if space.building_type_eq(BuildingType::Orange) {
                n + 1
            } else {
                n
            }
        });

    let scores = score_per_each(board, BuildingType::Orange, points);

    scores
}
// -----------------------------------------------------------------------------
fn score_temple(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
    idx: usize,
) -> bool {
    let score = board.adjacent_idxs(idx)
        .intersection(fed_idxs)
        .fold(0, |n, ii| {
            let space = &board.spaces()[*ii];
            if space.building_type_eq(BuildingType::Blue) {
                n + 1
            } else if space.building_type_eq(BuildingType::Magenta)
                && building_config.magenta() == MagentaBuilding::BarrettCastle
            {
                n + 2
            } else {
                n
            }
        })
        >= 2;

    score
}

// -----------------------------------------------------------------------------
fn score_temples(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(BuildingType::Orange) {
                if score_temple(board, building_config, fed_idxs, idx) {
                    m.insert(idx, 4);
                } else {
                    m.insert(idx, 0);
                }
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = match building_config.orange() {
        OrangeBuilding::Abbey => score_abbeys(board),
        OrangeBuilding::Chapel => score_chapels(board, building_config, fed_idxs),
        OrangeBuilding::Cloister => score_cloisters(board),
        OrangeBuilding::Temple => score_temples(board, building_config, fed_idxs),
    };

    scores
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

        let ans = HashMap::from([(0, 0), (3, 3), (4, 0), (7, 0), (8, 3)]);
        assert_eq!(score_abbeys(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_chapels() {
        let mut board = Board::new(4, 4);
        // Without Barrett Castle.
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
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

        fed_idxs.insert(1);
        let ans = HashMap::from([(0, 1)]);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        fed_idxs.insert(2);
        fed_idxs.insert(3);
        fed_idxs.insert(4);
        let ans = HashMap::from([(0, 4)]);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

        board.place(5, BuildingType::Orange);
        let ans = HashMap::from([(0, 4), (5, 4)]);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

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
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

        board.place(6, BuildingType::Magenta);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);

        fed_idxs.insert(6);
        let ans = HashMap::from([(0, 6), (5, 6)]);
        assert_eq!(score_chapels(&board, &building_config, &fed_idxs), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_cloisters() {
        let mut board = Board::new(4, 4);

        board.place(1, BuildingType::Orange);
        let ans = HashMap::from([(1, 0)]);
        assert_eq!(score_cloisters(&board), ans);

        board.remove(1);
        board.place(0, BuildingType::Orange);
        let ans = HashMap::from([(0, 1)]);
        assert_eq!(score_cloisters(&board), ans);

        board.place(1, BuildingType::Orange);
        let ans = HashMap::from([(0, 1), (1, 1)]);
        assert_eq!(score_cloisters(&board), ans);

        board.place(3, BuildingType::Orange);
        let ans = HashMap::from([(0, 2), (1, 2), (3, 2)]);
        assert_eq!(score_cloisters(&board), ans);

        board.place(12, BuildingType::Orange);
        let ans = HashMap::from([(0, 3), (1, 3), (3, 3), (12, 3)]);
        assert_eq!(score_cloisters(&board), ans);

        board.remove(1);
        board.place(15, BuildingType::Orange);
        let ans = HashMap::from([(0, 4), (3, 4), (12, 4), (15, 4)]);
        assert_eq!(score_cloisters(&board), ans);

        board.place(14, BuildingType::Orange);
        let ans = HashMap::from([(0, 4), (3, 4), (12, 4), (14, 4), (15, 4)]);
        assert_eq!(score_cloisters(&board), ans);
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
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        fed_idxs.insert(1);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        board.place(2, BuildingType::Blue);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        fed_idxs.insert(2);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        board.place(4, BuildingType::Blue);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        fed_idxs.insert(4);
        let ans = HashMap::from([(0, 4)]);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

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
        let ans = HashMap::from([(0, 4), (12, 0)]);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        board.place(8, BuildingType::Magenta);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);

        fed_idxs.insert(8);
        let ans = HashMap::from([(0, 4), (12, 4)]);
        assert_eq!(score_temples(&board, &building_config, &fed_idxs), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score() {
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Orange);
        board.place(1, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        board.place(5, BuildingType::Orange);
        board.place(3, BuildingType::Orange);
        board.place(15, BuildingType::Red);
        board.place(7, BuildingType::Black);
        board.place(14, BuildingType::Blue);
        let fed_idxs = HashSet::from([1, 4, 14]);

        // Score with abbeys.
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
        let ans = HashMap::from([(0, 3), (3, 0), (5, 3)]);
        assert_eq!(score(&board, &building_config, &fed_idxs), ans);

        // Score with chapels.
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
        let ans = HashMap::from([(0, 3), (3, 3), (5, 3)]);
        assert_eq!(score(&board, &building_config, &fed_idxs), ans);

        // Score with cloisters.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Cloister,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let ans = HashMap::from([(0, 2), (3, 2), (5, 2)]);
        assert_eq!(score(&board, &building_config, &fed_idxs), ans);

        // Score with temples.
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
        let ans = HashMap::from([(0, 4), (3, 0), (5, 4)]);
        assert_eq!(score(&board, &building_config, &fed_idxs), ans);
    }
}
