use std::collections::HashSet;

use crate::game::space::{BuildingType, Space};
use crate::game::board::Board;
use crate::game::piece::{BuildingConfig, MagentaBuilding, RedBuilding};
use crate::score::{blue, magenta, orange, ScoringContext};

pub mod farm;
pub mod granary;
pub mod greenhouse;
pub mod orchard;

// -----------------------------------------------------------------------------
fn is_feedable(building_config: &BuildingConfig, space: &Space) -> bool {
    let is_feedable = space.building_type_eq(BuildingType::Blue)
        || (space.building_type_eq(BuildingType::Magenta)
            && building_config.magenta() == MagentaBuilding::BarrettCastle);

    is_feedable
}

// -----------------------------------------------------------------------------
fn feedable_idxs(
    board: &Board,
    building_config: &BuildingConfig,
) -> HashSet<usize> {
    let feedable_idxs = board.spaces().iter().enumerate().fold(
        HashSet::new(),
        |mut s, (idx, space)| {
            if is_feedable(building_config, space) {
                s.insert(idx);
            }
            s
        },
    );

    feedable_idxs
}

// -----------------------------------------------------------------------------
fn best_fed_idxs(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    permutations: Vec<HashSet<usize>>,
) -> HashSet<usize> {
    let best_fed_idxs = permutations
        .iter()
        .fold((HashSet::new(), 0), |(best, max), permutation| {
            let score = blue::score(
                board,
                building_config,
                scoring_context,
                permutation,
            )
            .values()
            .sum::<i32>()
                + orange::score(
                    board,
                    building_config,
                    scoring_context,
                    permutation,
                )
                .values()
                .sum::<i32>()
                + magenta::score(
                    board,
                    building_config,
                    scoring_context,
                    permutation,
                )
                .values()
                .sum::<i32>() as i32;
            if score > max {
                (permutation.clone(), score)
            } else {
                (best, max)
            }
        })
        .0;

    best_fed_idxs
}

// -----------------------------------------------------------------------------
pub fn feed(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
) -> HashSet<usize> {
    let fed_idxs = match building_config.red() {
        RedBuilding::Farm => {
            farm::feed(board, building_config, scoring_context)
        }
        RedBuilding::Granary => granary::feed(board, building_config),
        RedBuilding::Greenhouse => {
            greenhouse::feed(board, building_config, scoring_context)
        }
        RedBuilding::Orchard => orchard::feed(board, building_config),
    };

    fed_idxs
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::game::piece::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_feedable_idxs() {
        // Map without Barrett Castle.
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
        let mut board = Board::new(5, 6);
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Green);
        board.place(7, BuildingType::Blue);
        board.place(8, BuildingType::Orange);
        board.place(9, BuildingType::Yellow);
        board.place(10, BuildingType::Gray);
        board.place(14, BuildingType::Blue);
        board.place(19, BuildingType::Black);
        board.place(21, BuildingType::Red);
        board.place(24, BuildingType::Magenta);
        assert_eq!(
            feedable_idxs(&board, &building_config),
            HashSet::from([0, 7, 14])
        );

        // Map with Barrett Castle.
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
        assert_eq!(
            feedable_idxs(&board, &building_config),
            HashSet::from([0, 7, 14, 24])
        );
    }
}
