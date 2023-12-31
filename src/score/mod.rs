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

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score_by_adjacency() {

    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score_unused_spaces() {

    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }

}
