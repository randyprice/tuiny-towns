use std::collections::HashSet;

use itertools::Itertools;

use crate::board::Board;
use crate::building::{BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;
use crate::feed::best_fed_idxs;

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
        .collect_vec()
        .iter()
        .fold(Vec::new(), |mut perms, groups| {
            // Create one permutation by contcatenating sets of contiguous
            // groups.
            let permutation = groups.iter()
                .fold(HashSet::new(), |mut s, group| {
                    s.extend(group);
                    s
                    // // Add all indices from one contiguous group to the HashSet.
                    // group.iter()
                    //     .fold(s, |mut partial_set, idx| {
                    //         partial_set.insert(*idx);
                    //         partial_set
                    //     })
                });
            perms.push(permutation);
            perms
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
    use crate::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
    };
    use crate::utils::vec_hashset_eq;

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
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

        //TODO add second greenhouse.
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_feed() {

    }
}
