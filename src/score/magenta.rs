use std::collections::HashSet;

use crate::board::{Board, BuildingConfig, BuildingType, MagentaBuilding};

// -----------------------------------------------------------------------------
fn score_barrett_castle(board: &Board, fed_idxs: &HashSet<usize>) -> i32 {
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |mut n, (idx, space)| {
            if let Some(building_type) = space.building_type() {
                if building_type == BuildingType::Magenta && fed_idxs.contains(&idx) {
                    n += 5;
                }
            }
            n
        });

    score
}

// -----------------------------------------------------------------------------
pub fn score_magenta(board: &Board, building_config: &BuildingConfig, fed_idxs: &HashSet<usize>) -> i32 {
    let score = match building_config.magenta() {
        MagentaBuilding::BarrettCastle => score_barrett_castle(board, fed_idxs),
        _ => 0,
    };

    score
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_barrett_castle() {
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Black);

        let mut fed_idxs: HashSet<usize> = HashSet::new();
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

        board.place(1, BuildingType::Magenta);
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

        board.place(2, BuildingType::Blue);
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

        fed_idxs.insert(2);
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

        fed_idxs.insert(1);
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 5);

        board.place(3, BuildingType::Magenta);
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 5);

        fed_idxs.insert(3);
        assert_eq!(score_barrett_castle(&board, &fed_idxs), 10);
    }
}