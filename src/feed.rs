use std::cmp;
use std::collections::HashSet;

use itertools::Itertools;

use crate::board::{MagentaBuilding, RedBuilding,};
use crate::board::{Board, BuildingConfig, BuildingType, Space};
use crate::score::blue::score_blue;
use crate::score::magenta::score_magenta;
use crate::score::orange::score_orange;

// -----------------------------------------------------------------------------
fn is_feedable(building_config: &BuildingConfig, space: &Space) -> bool {
    let is_feedable = if let Some(building_type) = space.building_type() {
        building_type == BuildingType::Blue
            || (building_type == BuildingType::Magenta
                && building_config.magenta() == MagentaBuilding::BarrettCastle)
    } else {
        false
    };

    is_feedable
}

// -----------------------------------------------------------------------------
fn feedable_idxs(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let feedable_idxs = board.spaces()
        .iter()
        .enumerate()
        .fold(HashSet::new(), |mut s, (idx, space)| {
            if is_feedable(building_config, space) {
                s.insert(idx);
            }
            s
        });

    feedable_idxs
}

// -----------------------------------------------------------------------------
fn feedable_permutations_for_farms(board: &Board, feedable_idxs: HashSet<usize>) -> Vec<HashSet<usize>> {
    let n_feedable = feedable_idxs.len();
    let permutations = feedable_idxs.into_iter()
        // Create Vec<Vec<usize>> of permutations.
        .combinations(
            cmp::min(
                4 * board.count_building_type(BuildingType::Red) as usize,
                n_feedable
            )
        )
        // Convert to Vec<HashSet<usize>>.
        .fold(Vec::new(), |mut perms, v| {
            perms.push(HashSet::from_iter(v));
            perms
        });

    permutations
}

// -----------------------------------------------------------------------------
fn feedable_permutations_for_greenhouses(board: &Board, building_config: &BuildingConfig) -> Vec<HashSet<usize>> {
    let building_types = if building_config.magenta() == MagentaBuilding::BarrettCastle {
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
                .fold(HashSet::new(), |s, group| {
                    // Add all indices from one contiguous group to the HashSet.
                    group.iter()
                        .fold(s, |mut partial_set, idx| {
                            partial_set.insert(*idx);
                            partial_set
                        })
                });

            perms.push(permutation);
            perms
        });

    permutations
}

// -----------------------------------------------------------------------------
fn fed_buildings(board: &Board, building_config: &BuildingConfig, permutations: Vec<HashSet<usize>>) -> HashSet<usize> {
    let fed_buildings = permutations.iter()
        .fold((HashSet::new(), 0), |(best, max), permutation| {
            let score = score_blue(board, building_config, permutation)
                + score_orange(board, building_config, permutation)
                + score_magenta(board, building_config, permutation);
            if score > max {
                (permutation.clone(), score)
            } else {
                (best, max)
            }
        })
        .0;

    fed_buildings
}

// -----------------------------------------------------------------------------
fn feed_with_farms(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let feedable_idxs = feedable_idxs(board, building_config);
    let permutations = feedable_permutations_for_farms(board, feedable_idxs);
    let fed_buildings = fed_buildings(board, building_config, permutations);

   fed_buildings
}

// -----------------------------------------------------------------------------
fn feed_with_greenhouses(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let permutations = feedable_permutations_for_greenhouses(board, building_config);

    HashSet::new()
}

// -----------------------------------------------------------------------------
pub fn feed(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let fed_buildings = match building_config.red() {
        RedBuilding::Farm => feed_with_farms(board, building_config),
        RedBuilding::Granary => HashSet::new(),
        RedBuilding::Greenhouse => HashSet::new(),
        RedBuilding::Orchard => HashSet::new(),
    };

    fed_buildings
}

// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
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
        assert_eq!(feedable_idxs(&board, &building_config), HashSet::from([0, 7, 14]));

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
        assert_eq!(feedable_idxs(&board, &building_config), HashSet::from([0, 7, 14, 24]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_feedable_permutations_for_farms() {
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
        let mut board = Board::new(5, 6);
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Blue);
        board.place(7, BuildingType::Green);
        board.place(8, BuildingType::Orange);
        board.place(9, BuildingType::Yellow);
        board.place(10, BuildingType::Gray);
        board.place(14, BuildingType::Blue);
        board.place(19, BuildingType::Black);
        board.place(21, BuildingType::Red);
        board.place(24, BuildingType::Magenta);
        let feedable = feedable_idxs(&board, &building_config);
        let permutations = feedable_permutations_for_farms(&board, feedable);
        assert_eq!(permutations, vec![HashSet::from([0, 1, 14, 24])]);

        // Add another cottage.
        board.place(25, BuildingType::Blue);
        let feedable = feedable_idxs(&board, &building_config);
        let permutations = feedable_permutations_for_farms(&board, feedable);
        let ans = vec![
            HashSet::from([0, 1, 14, 24]),
            HashSet::from([0, 1, 14, 25]),
            HashSet::from([0, 1, 24, 25]),
            HashSet::from([0, 14, 24, 25]),
            HashSet::from([1, 14, 24, 25]),
        ];
        let eq = permutations.iter().all(|s| ans.contains(&s))
            && ans.iter().all(|s| permutations.contains(s));

        assert!(eq);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_feedable_permutations_for_greenhouses() {
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
        let permutations = feedable_permutations_for_greenhouses(&board, &building_config);
        assert!(permutations.is_empty());

        board.place(0, BuildingType::Blue);
        let permutations = feedable_permutations_for_greenhouses(&board, &building_config);
        assert_eq!(permutations, vec![HashSet::new()]);

        board.place(15, BuildingType::Red);
        let permutations = feedable_permutations_for_greenhouses(&board, &building_config);
        assert_eq!(permutations, vec![HashSet::from([0])]);

        board.place(1, BuildingType::Blue);
        let permutations = feedable_permutations_for_greenhouses(&board, &building_config);
        assert_eq!(permutations, vec![HashSet::from([0, 1])]);

        //TODO add more


    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_feed_with_farms() {
        // No temple, no Barrett Castle.
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

        // Empty board.
        let mut board = Board::new(4, 4);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(fed_idxs.is_empty());

        // No farm, so no fed buildings.
        board.place(0, BuildingType::Blue);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(fed_idxs.is_empty());

        // Add a farm to feed the single blue building.
        board.place(15, BuildingType::Red);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([0]));

        // Add a few more blue buildings.
        board.place(1, BuildingType::Blue);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([0, 1]));

        board.place(2, BuildingType::Blue);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([0, 1, 2]));

        board.place(3, BuildingType::Blue);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([0, 1, 2, 3]));

        // Five blue buildings - only four will be fed.
        board.place(4, BuildingType::Blue);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(
            vec![
                HashSet::from([0, 1, 2, 3]),
                HashSet::from([0, 1, 2, 4]),
                HashSet::from([0, 1, 3, 4]),
                HashSet::from([0, 2, 3, 4]),
                HashSet::from([1, 2, 3, 4]),
            ]
            .contains(&fed_idxs)
        );
        // Add another farm to feed all five.
        board.place(14, BuildingType::Red);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([0, 1, 2, 3, 4]));

        // Temple, no Barret Castle.
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

        // Two temples with two adjacent blue buildings each.
        let mut board = Board::new(4, 4);
        board.place(15, BuildingType::Red);
        board.place(0, BuildingType::Orange);
        board.place(1, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        board.place(12, BuildingType::Orange);
        board.place(8, BuildingType::Blue);
        board.place(13, BuildingType::Blue);
        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Blue);
        board.place(7, BuildingType::Blue);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([1, 4, 8, 13]));

        // Add another farm to feed all blue buildings.
        board.place(14, BuildingType::Red);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([1, 2, 3, 4, 7, 8, 13]));

        // No temple, Barrett Castle.
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

        // No farm.
        let mut board = Board::new(4, 4);
        board.place(14, BuildingType::Magenta);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(fed_idxs.is_empty());

        // Add a farm to feed one Barrett Castle.
        board.place(15, BuildingType::Red);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([14]));

        // Add four blue buildings.
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Blue);
        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Blue);

        // Ensure Barret Castle is in fed buildings.
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(fed_idxs.contains(&14));

        // Add another Barrett Castle.
        board.place(13, BuildingType::Magenta);
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(fed_idxs.contains(&13) && fed_idxs.contains(&14));

        // Temple and Barrett Castle.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Well,
            GreenBuilding::Tavern,
            MagentaBuilding::BarrettCastle,
            OrangeBuilding::Temple,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        let mut board = Board::new(4, 4);

        // Put two temples in the top corners, flanked by two cottages each.
        board.place(0, BuildingType::Orange);
        board.place(1, BuildingType::Blue);
        board.place(4, BuildingType::Blue);
        board.place(3, BuildingType::Orange);
        board.place(2, BuildingType::Blue);
        board.place(7, BuildingType::Blue);

        // One farm to feed four buildings.
        board.place(15, BuildingType::Red);

        // Finally, the Barrett Castle.
        board.place(5, BuildingType::Magenta);

        // The four cottages should be selected, as they score the most in
        // combination with the temples.
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert_eq!(fed_idxs, HashSet::from([1, 2, 4, 7]));

        // Move one of the cottages.
        board.remove(4);
        board.place(8, BuildingType::Blue);

        // Make sure the cottages adjacent to the top right chapel are still
        // fed, along with the Barrett Castle.
        let fed_idxs = feed_with_farms(&board, &building_config);
        assert!(fed_idxs.contains(&2) && fed_idxs.contains(&7) && fed_idxs.contains(&5));
    }
}