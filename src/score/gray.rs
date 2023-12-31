use std::collections::HashSet;

use crate::board::Board;
use crate::building::{BuildingType, GrayBuilding};
use crate::building_config::BuildingConfig;
use crate::score::score_by_adjacency;

// -----------------------------------------------------------------------------
fn score_fountains(board: &Board) -> i32 {
    let score =
        board.contiguous_groups(&HashSet::from([BuildingType::Gray]))
        .iter()
        .filter(|group| group.len() > 1)
        .fold(0, |n, group| n + 2 * group.len())
        as i32;

    score
}

// -----------------------------------------------------------------------------
fn score_millstones(board: &Board) -> i32 {
    let score =
        score_by_adjacency(
            true,
            board,
            BuildingType::Gray,
            HashSet::from([BuildingType::Red, BuildingType::Yellow]),
            2,
        );

    println!("{score}");
    score
}

// -----------------------------------------------------------------------------
fn score_sheds(board: &Board) -> i32 {
    let score = board.count_building_type(BuildingType::Gray) as i32;

    score
}

// -----------------------------------------------------------------------------
fn score_wells(board: &Board) -> i32 {
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |n, (idx, space)| {
            if space.building_type_eq(BuildingType::Gray) {
                board.count_adjacent_building_types(
                    idx,
                    HashSet::from([BuildingType::Blue]),
                )
                + n
            } else {
                n
            }
        })
        as i32;

    score
}

// -----------------------------------------------------------------------------
pub fn score(board: &Board, building_config: &BuildingConfig) -> i32 {
    let score = match building_config.gray() {
        GrayBuilding::Fountain => score_fountains(board),
        GrayBuilding::Millstone => score_millstones(board),
        GrayBuilding::Shed => score_sheds(board),
        GrayBuilding::Well => score_wells(board),
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_fountains() {
        let mut board = Board::new(4, 4);
        assert_eq!(score_fountains(&board), 0);

        board.place(0, BuildingType::Gray);
        assert_eq!(score_fountains(&board), 0);

        board.place(1, BuildingType::Gray);
        assert_eq!(score_fountains(&board), 4);

        board.place(3, BuildingType::Gray);
        assert_eq!(score_fountains(&board), 4);

        board.place(11, BuildingType::Gray);
        assert_eq!(score_fountains(&board), 4);

        board.place(7, BuildingType::Gray);
        assert_eq!(score_fountains(&board), 10);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_millstones() {
        let mut board = Board::new(4, 4);
        assert_eq!(score_millstones(&board), 0);

        board.place(0, BuildingType::Gray);
        assert_eq!(score_millstones(&board), 0);

        board.place(4, BuildingType::Blue);
        assert_eq!(score_millstones(&board), 0);

        board.place(1, BuildingType::Red);
        assert_eq!(score_millstones(&board), 2);

        board.place(3, BuildingType::Gray);
        assert_eq!(score_millstones(&board), 2);

        board.place(7, BuildingType::Yellow);
        assert_eq!(score_millstones(&board), 4);

        board.place(2, BuildingType::Red);
        assert_eq!(score_millstones(&board), 4);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_sheds() {
        let mut board = Board::new(4, 4);
        assert_eq!(score_sheds(&board), 0);

        board.place(0, BuildingType::Gray);
        assert_eq!(score_sheds(&board), 1);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_sheds(&board), 1);

        board.place(2, BuildingType::Gray);
        assert_eq!(score_sheds(&board), 2);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_wells() {
        let mut board = Board::new(4, 4);
        assert_eq!(score_wells(&board), 0);

        board.place(5, BuildingType::Gray);
        assert_eq!(score_wells(&board), 0);

        board.place(1, BuildingType::Blue);
        assert_eq!(score_wells(&board), 1);

        board.place(4, BuildingType::Blue);
        assert_eq!(score_wells(&board), 2);

        board.place(6, BuildingType::Blue);
        assert_eq!(score_wells(&board), 3);

        board.place(9, BuildingType::Blue);
        assert_eq!(score_wells(&board), 4);

        board.place(2, BuildingType::Gray);
        assert_eq!(score_wells(&board), 6);
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }
}
