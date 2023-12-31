use std::collections::HashSet;

use crate::board::Board;
use crate::building::BuildingType;
use crate::building_config::BuildingConfig;
use crate::feed::feedable_idxs;

// -----------------------------------------------------------------------------
pub fn feed(board: &Board, building_config: &BuildingConfig) -> HashSet<usize> {
    let (fed_rows, fed_cols) = board.spaces()
        .iter()
        .enumerate()
        .fold((HashSet::new(), HashSet::new()), |(mut rows, mut cols), (idx, space)| {
            if space.building_type_eq(BuildingType::Red) {
                rows.insert(board.row(idx));
                cols.insert(board.col(idx));
            }
            (rows, cols)
        });

    let fed_idxs = feedable_idxs(board, building_config)
        .into_iter()
        .fold(HashSet::new(), |mut s, idx| {
            if fed_rows.contains(&board.row(idx))
            || fed_cols.contains(&board.col(idx)) {
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