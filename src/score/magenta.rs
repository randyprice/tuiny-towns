use std::collections::{HashMap, HashSet};

use strum::IntoEnumIterator;

use crate::board::Board;
use crate::building::{BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;
use crate::score::{score_if_in_idx_set, score_per_each};

// -----------------------------------------------------------------------------
fn score_archive_of_the_second_age(board: &Board) -> HashMap<usize, i32> {
    let points = board.spaces()
        .iter()
        .fold(HashSet::new(), |mut s, space| {
            if let Some(building_type) = space.building_type() {
                if building_type != BuildingType::Magenta {
                    s.insert(building_type);
                }
            }
            s
        })
        .len()
        as i32;

    let scores = score_per_each(board, BuildingType::Magenta, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_mandras_palace(board: &Board) -> HashMap<usize, i32> {
    let score = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(BuildingType::Magenta) {
                let points = board.unique_adjacent_building_types(idx).len();
                m.insert(idx, 2 * points as i32);
            }
            m
        });

    score
}

// -----------------------------------------------------------------------------
fn score_silvia_forum(board: &Board) -> HashMap<usize, i32> {
    let points = BuildingType::iter()
        .fold(Vec::new(), |mut maxes, building_type| {
            let max = board.contiguous_groups(&HashSet::from([building_type]))
                .iter()
                .map(|s| s.len())
                .max()
                .unwrap_or(0);
            maxes.push(max);
            maxes
        })
        .into_iter()
        .max()
        .unwrap_or(0)
        as i32
        + 1;

    let scores = score_per_each(board, BuildingType::Magenta, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_the_sky_baths(board: &Board) -> HashMap<usize, i32> {
    let idxs = HashSet::from_iter(0..board.elems());
    let unique_building_types =
        board.unique_building_types_in_idx_set(&idxs).len();
    let missing_building_types = BuildingType::iter().count()
        - unique_building_types;
    let points = 2 * missing_building_types as i32;

    let scores = score_per_each(board, BuildingType::Magenta, points);

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = match building_config.magenta() {
        MagentaBuilding::ArchitectsGuild => {
            score_per_each(board, BuildingType::Magenta, 1)
        }
        MagentaBuilding::ArchiveOfTheSecondAge => {
            score_archive_of_the_second_age(board)
        }
        MagentaBuilding::BarrettCastle => {
            score_if_in_idx_set(board, fed_idxs, BuildingType::Magenta, 5)
        }
        MagentaBuilding::CathedralOfCaterina => {
            score_per_each(board, BuildingType::Magenta, 2)
        }
        MagentaBuilding::FortIronweed => {
            score_per_each(board, BuildingType::Magenta, 7)
        }
        MagentaBuilding::GrandMausoleumOfTheRodina
        | MagentaBuilding::ObeliskOfTheCrescent
        | MagentaBuilding::OpaleyesWatch
        | MagentaBuilding::StatueOfTheBondmaker => {
            score_per_each(board, BuildingType::Magenta, 0)
        }
        MagentaBuilding::GroveUniversity => {
            score_per_each(board, BuildingType::Magenta, 3)
        }
        MagentaBuilding::MandrasPalace => score_mandras_palace(board),
        MagentaBuilding::ShrineOfTheElderTree
        | MagentaBuilding::TheStarloom => {
            score_per_each(board, BuildingType::Magenta, -99)
        }
        MagentaBuilding::SilvaForum => score_silvia_forum(board),
        MagentaBuilding::TheSkyBaths => score_the_sky_baths(board),
    };

    scores
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_archive_of_the_second_age() {
        let mut board = Board::new(4, 4);
        assert!(score_archive_of_the_second_age(&board).is_empty());

        board.place(0, BuildingType::Magenta);
        let ans = HashMap::from([(0, 0)]);
        assert_eq!(score_archive_of_the_second_age(&board), ans);

        board.place(1, BuildingType::Black);
        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Green);
        board.place(4, BuildingType::Gray);
        let ans = HashMap::from([(0, 4)]);
        assert_eq!(score_archive_of_the_second_age(&board), ans);

        board.place(5, BuildingType::Blue);
        assert_eq!(score_archive_of_the_second_age(&board), ans);

        board.place(6, BuildingType::Orange);
        board.place(7, BuildingType::Red);
        board.place(8, BuildingType::Yellow);
        let ans = HashMap::from([(0, 7)]);
        assert_eq!(score_archive_of_the_second_age(&board), ans);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_mandras_palace() {
        let mut board = Board::new(4, 4);
        assert!(score_mandras_palace(&board).is_empty());

        board.place(5, BuildingType::Magenta);
        assert_eq!(score_mandras_palace(&board), HashMap::from([(5, 0)]));

        board.place(1, BuildingType::Blue);
        assert_eq!(score_mandras_palace(&board), HashMap::from([(5, 2)]));

        board.place(4, BuildingType::Blue);
        assert_eq!(score_mandras_palace(&board), HashMap::from([(5, 2)]));

        board.place(4, BuildingType::Red);
        assert_eq!(score_mandras_palace(&board), HashMap::from([(5, 4)]));

        board.place(6, BuildingType::Yellow);
        assert_eq!(score_mandras_palace(&board), HashMap::from([(5, 6)]));

        board.place(9, BuildingType::Orange);
        assert_eq!(score_mandras_palace(&board), HashMap::from([(5, 8)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_silva_forum() {
        let mut board = Board::new(4, 4);
        assert!(score_silvia_forum(&board).is_empty());

        board.place(15, BuildingType::Magenta);
        assert_eq!(score_silvia_forum(&board), HashMap::from([(15, 2)]));

        board.place(0, BuildingType::Blue);
        assert_eq!(score_silvia_forum(&board), HashMap::from([(15, 2)]));

        board.place(3, BuildingType::Blue);
        assert_eq!(score_silvia_forum(&board), HashMap::from([(15, 2)]));

        board.place(2, BuildingType::Blue);
        assert_eq!(score_silvia_forum(&board), HashMap::from([(15, 3)]));

        board.place(1, BuildingType::Blue);
        assert_eq!(score_silvia_forum(&board), HashMap::from([(15, 5)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_the_sky_baths() {
        let mut board = Board::new(4, 4);
        assert!(score_the_sky_baths(&board).is_empty());

        board.place(15, BuildingType::Magenta);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 14)]));

        board.place(0, BuildingType::Black);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 12)]));

        board.place(1, BuildingType::Blue);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 10)]));

        board.place(2, BuildingType::Gray);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 8)]));

        board.place(3, BuildingType::Green);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 6)]));

        board.place(4, BuildingType::Black);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 6)]));

        board.place(5, BuildingType::Orange);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 4)]));

        board.place(6, BuildingType::Red);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 2)]));

        board.place(7, BuildingType::Yellow);
        assert_eq!(score_the_sky_baths(&board), HashMap::from([(15, 0)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }
}
