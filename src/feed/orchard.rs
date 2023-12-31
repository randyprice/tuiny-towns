use std::collections::HashSet;

use crate::board::Board;
use crate::building_config::BuildingConfig;
use crate::building::BuildingType;
use crate::feed::feedable_idxs;

// -----------------------------------------------------------------------------
pub fn feed(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let fed_idxs = feedable_idxs(board, building_config)
        .into_iter()
        .fold(HashSet::new(), |mut s, idx| {
            if board.unique_surrounding_building_types(idx)
                .contains(&BuildingType::Red) {
                s.insert(idx);
            }
            s
        });

    fed_idxs
}

// =============================================================================
#[cfg(test)]
mod test {

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_feed() {

    }
}