use std::collections::HashSet;

use itertools::Itertools;

use crate::board::Board;
use crate::board::space::BuildingType;
use crate::building_config::{BuildingConfig, MagentaBuilding};
use crate::score::feed::best_fed_idxs;

// -----------------------------------------------------------------------------
fn feedable_permutations(
    board: &Board,
    building_config: &BuildingConfig,
) -> Vec<HashSet<usize>> {
    let building_types =
        if building_config.magenta() == MagentaBuilding::BarrettCastle {
            HashSet::from([BuildingType::Blue, BuildingType::Magenta])
        } else {
            HashSet::from([BuildingType::Blue])
        };

    let permutations = board.contiguous_groups(&building_types)
        .into_iter()
        .combinations(board.count_building_type(BuildingType::Red) as usize)
        .fold(Vec::new(), |mut permutations, groups| {
            // A single permutation consists of N contiguous groups of feedable
            // buildings, where N is the number of greenhouses on the board.
            let permutation = groups.iter()
                .fold(HashSet::new(), |mut permutation, group| {
                    permutation.extend(group);
                    permutation
                });
            permutations.push(permutation);
            permutations
        });

    permutations
}

// -----------------------------------------------------------------------------
pub fn feed(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let permutations = feedable_permutations(board, building_config);
    let fed_buildings = best_fed_idxs(board, building_config, permutations);

    fed_buildings
}

// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::building_config::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
    };
    use crate::utils::vec_hashset_eq;

    // -------------------------------------------------------------------------
    #[test]
    fn test_feedable_permutations() {
        // Without Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Chapel,
            RedBuilding::Greenhouse,
            YellowBuilding::Theater,
        );
        let mut board = Board::new(4, 4);
        let permutations = feedable_permutations(&board, &building_config);
        assert!(permutations.is_empty());

        board.place(0, BuildingType::Blue);
        let permutations = feedable_permutations(&board, &building_config);
        assert_eq!(permutations, vec![HashSet::new()]);

        board.place(15, BuildingType::Red);
        let permutations = feedable_permutations(&board, &building_config);
        assert_eq!(permutations, vec![HashSet::from([0])]);

        board.place(1, BuildingType::Blue);
        let permutations = feedable_permutations(&board, &building_config);
        assert_eq!(permutations, vec![HashSet::from([0, 1])]);

        board.place(3, BuildingType::Blue);
        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1]), HashSet::from([3])];
        assert!(vec_hashset_eq(&permutations, &ans));

        board.place(7, BuildingType::Blue);
        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1]), HashSet::from([3, 7])];
        assert!(vec_hashset_eq(&permutations, &ans));

        board.place(2, BuildingType::Blue);
        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1, 2, 3, 7])];
        assert!(vec_hashset_eq(&permutations, &ans));

        board.place(12, BuildingType::Magenta);
        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1, 2, 3, 7])];
        assert!(vec_hashset_eq(&permutations, &ans));

        // With Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Chapel,
            RedBuilding::Greenhouse,
            YellowBuilding::Theater,
        );

        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1, 2, 3, 7]), HashSet::from([12])];
        assert!(vec_hashset_eq(&permutations, &ans));

        board.place(13, BuildingType::Blue);
        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1, 2, 3, 7]), HashSet::from([12, 13])];
        assert!(vec_hashset_eq(&permutations, &ans));

        // Add second greenhouse - both contiguous groups can be fed now, so
        // they form one permutation.
        board.place(14, BuildingType::Red);
        let permutations = feedable_permutations(&board, &building_config);
        let ans = vec![HashSet::from([0, 1, 2, 3, 7, 12, 13])];
        println!("permutations: {:?}", permutations);
        println!("ans: {:?}", ans);

        assert!(vec_hashset_eq(&permutations, &ans));

    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_feed() {
        let mut board = Board::new(6, 6);

        board.place(0, BuildingType::Orange);
        board.place(1, BuildingType::Blue);
        board.place(6, BuildingType::Blue);
        board.place(7, BuildingType::Blue);

        board.place(4, BuildingType::Blue);
        board.place(5, BuildingType::Blue);
        board.place(10, BuildingType::Blue);
        board.place(11, BuildingType::Blue);

        board.place(25, BuildingType::Orange);
        board.place(30, BuildingType::Orange);
        board.place(31, BuildingType::Magenta);
        board.place(32, BuildingType::Orange);

        board.place(28, BuildingType::Magenta);
        board.place(29, BuildingType::Blue);
        board.place(34, BuildingType::Blue);
        board.place(35, BuildingType::Blue);

        // Without Temple or Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Abbey,
            RedBuilding::Greenhouse,
            YellowBuilding::Theater,
        );

        // Test empty board first.
        assert!(feed(&board, &building_config).is_empty());

        // Now place the greenhouse.
        board.place(3, BuildingType::Red);

        let ans = HashSet::from([4, 5, 10, 11]);
        assert_eq!(feed(&board, &building_config), ans);

        // With Temple, without Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::SilvaForum,
            OrangeBuilding::Temple,
            RedBuilding::Greenhouse,
            YellowBuilding::Theater,
        );

        let ans = HashSet::from([1, 6, 7]);
        assert_eq!(feed(&board, &building_config), ans);

        // Without Temple, with Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Abbey,
            RedBuilding::Greenhouse,
            YellowBuilding::Theater,
        );

        let ans = HashSet::from([28, 29, 34, 35]);
        assert_eq!(feed(&board, &building_config), ans);

        // With Temple and Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Fountain,
            GreenBuilding::Almshouse,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Temple,
            RedBuilding::Greenhouse,
            YellowBuilding::Theater,
        );

        let ans = HashSet::from([31]);
        assert_eq!(feed(&board, &building_config), ans);

    }
}
