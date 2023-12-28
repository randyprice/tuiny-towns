use std::collections::HashSet;

use crate::board::{BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
    MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding};
use crate::board::{Board, Build, Building, BuildingConfig, BuildingType, Space};
use crate::feed::feed;
use crate::score::orange::score_orange;

pub mod blue;
pub mod magenta;
pub mod orange;

// -----------------------------------------------------------------------------
fn score_by_adjacency(
    score: bool, board: &Board, building_type: BuildingType,
    adjacent_types: HashSet<BuildingType>, points: i32
) -> i32 {
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(0, |mut n, (idx, space)| {
            if let Some(bt) = space.building_type() {
                if bt == building_type
                    && board.adjacent_building_types(idx)
                        .into_iter()
                        .any(|bty| adjacent_types.contains(&bty))
                        == score {
                    n += points;
                }
            }
            n
        });

    score
}

// -----------------------------------------------------------------------------
pub fn score(board: &Board, building_config: &BuildingConfig) -> i32 {
    let fed_buildings = feed(board, building_config);
    let score = -1;
    println!("Total score: {score}");

    score
}

// =============================================================================
// #[cfg(test)]
// mod tests {
//     use super::*;


// }