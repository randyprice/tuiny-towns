use std::cmp;
use std::collections::HashSet;

use itertools::Itertools;

use crate::game::space::BuildingType;
use crate::game::board::Board;
use crate::game::building::BuildingConfig;
use crate::score::feed::{best_fed_idxs, feedable_idxs};
use crate::score::ScoringContext;

// -----------------------------------------------------------------------------
fn feedable_permutations(
    board: &Board,
    feedable_idxs: HashSet<usize>,
) -> Vec<HashSet<usize>> {
    let n_feedable = feedable_idxs.len();
    let permutations = feedable_idxs
        .into_iter()
        // Create Vec<Vec<usize>> of permutations.
        .combinations(cmp::min(
            4 * board.count_building_type(BuildingType::Red) as usize,
            n_feedable,
        ))
        // Convert to Vec<HashSet<usize>>.
        .fold(Vec::new(), |mut perms, v| {
            perms.push(HashSet::from_iter(v));
            perms
        });

    permutations
}

// -----------------------------------------------------------------------------
pub fn feed(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
) -> HashSet<usize> {
    let feedable_idxs = feedable_idxs(board, building_config);
    let permutations = feedable_permutations(board, feedable_idxs);
    let fed_idxs =
        best_fed_idxs(board, building_config, scoring_context, permutations);

    fed_idxs
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::game::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_feedable_permutations() {
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
        let permutations = feedable_permutations(&board, feedable);
        assert_eq!(permutations, vec![HashSet::from([0, 1, 14, 24])]);

        // Add another cottage.
        board.place(25, BuildingType::Blue);
        let feedable = feedable_idxs(&board, &building_config);
        let permutations = feedable_permutations(&board, feedable);
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
    fn test_feed() {
        let scoring_context = ScoringContext::default();
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
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert!(fed_idxs.is_empty());

        // No farm, so no fed buildings.
        board.place(0, BuildingType::Blue);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert!(fed_idxs.is_empty());

        // Add a farm to feed the single blue building.
        board.place(15, BuildingType::Red);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([0]));

        // Add a few more blue buildings.
        board.place(1, BuildingType::Blue);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([0, 1]));

        board.place(2, BuildingType::Blue);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([0, 1, 2]));

        board.place(3, BuildingType::Blue);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([0, 1, 2, 3]));

        // Five blue buildings - only four will be fed.
        board.place(4, BuildingType::Blue);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert!(vec![
            HashSet::from([0, 1, 2, 3]),
            HashSet::from([0, 1, 2, 4]),
            HashSet::from([0, 1, 3, 4]),
            HashSet::from([0, 2, 3, 4]),
            HashSet::from([1, 2, 3, 4]),
        ]
        .contains(&fed_idxs));
        // Add another farm to feed all five.
        board.place(14, BuildingType::Red);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
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
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([1, 4, 8, 13]));

        // Add another farm to feed all blue buildings.
        board.place(14, BuildingType::Red);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
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
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert!(fed_idxs.is_empty());

        // Add a farm to feed one Barrett Castle.
        board.place(15, BuildingType::Red);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([14]));

        // Add four blue buildings.
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Blue);
        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Blue);

        // Ensure Barret Castle is in fed buildings.
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert!(fed_idxs.contains(&14));

        // Add another Barrett Castle.
        board.place(13, BuildingType::Magenta);
        let fed_idxs = feed(&board, &building_config, &scoring_context);
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
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert_eq!(fed_idxs, HashSet::from([1, 2, 4, 7]));

        // Move one of the cottages.
        board.remove(4);
        board.place(8, BuildingType::Blue);

        // Make sure the cottages adjacent to the top right chapel are still
        // fed, along with the Barrett Castle.
        let fed_idxs = feed(&board, &building_config, &scoring_context);
        assert!(
            fed_idxs.contains(&2)
                && fed_idxs.contains(&7)
                && fed_idxs.contains(&5),
        );
    }
}
