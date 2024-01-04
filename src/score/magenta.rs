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
                m.insert(idx, points as i32);
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
        as i32;

    let scores = score_per_each(board, BuildingType::Magenta, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_the_sky_baths(board: &Board) -> HashMap<usize, i32> {
    let idxs = HashSet::from_iter(0..board.elems() - 1);
    let unique_building_types = board.unique_building_types(&idxs).len();
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
    // use super::*;

    // // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_architects_guild() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_architects_guild(&board), 0);

    //     board.place(0, BuildingType::Magenta);
    //     assert_eq!(score_architects_guild(&board), 1);
    // }

    // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_archive_of_the_second_age() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_archive_of_the_second_age(&board), 0);

    //     board.place(0, BuildingType::Magenta);
    //     assert_eq!(score_archive_of_the_second_age(&board), 0);

    //     board.place(1, BuildingType::Black);
    //     assert_eq!(score_archive_of_the_second_age(&board), 1);

    //     board.place(2, BuildingType::Blue);
    //     assert_eq!(score_archive_of_the_second_age(&board), 2);

    //     board.place(3, BuildingType::Green);
    //     assert_eq!(score_archive_of_the_second_age(&board), 3);

    //     board.place(4, BuildingType::Gray);
    //     assert_eq!(score_archive_of_the_second_age(&board), 4);

    //     board.place(5, BuildingType::Blue);
    //     assert_eq!(score_archive_of_the_second_age(&board), 4);

    //     board.place(6, BuildingType::Orange);
    //     assert_eq!(score_archive_of_the_second_age(&board), 5);

    //     board.place(7, BuildingType::Red);
    //     assert_eq!(score_archive_of_the_second_age(&board), 6);

    //     board.place(8, BuildingType::Yellow);
    //     assert_eq!(score_archive_of_the_second_age(&board), 7);
    // }

     // -------------------------------------------------------------------------
    //  #[test]
    //  fn test_score_barrett_castle() {
    //      let mut board = Board::new(4, 4);
    //      board.place(0, BuildingType::Black);

    //      let mut fed_idxs: HashSet<usize> = HashSet::new();
    //      assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

    //      board.place(1, BuildingType::Magenta);
    //      assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

    //      board.place(2, BuildingType::Blue);
    //      assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

    //      fed_idxs.insert(2);
    //      assert_eq!(score_barrett_castle(&board, &fed_idxs), 0);

    //      fed_idxs.insert(1);
    //      assert_eq!(score_barrett_castle(&board, &fed_idxs), 5);
    //  }

    // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_catherdral_of_caterina() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_cathedral_of_caterina(&board), 0);

    //     board.place(0, BuildingType::Magenta);
    //     assert_eq!(score_cathedral_of_caterina(&board), 2);
    // }

    // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_fort_ironweed() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_fort_ironweed(&board), 0);

    //     board.place(0, BuildingType::Magenta);
    //     assert_eq!(score_fort_ironweed(&board), 7);
    // }

    // // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_grove_university() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_grove_university(&board), 0);

    //     board.place(0, BuildingType::Magenta);
    //     assert_eq!(score_grove_university(&board), 3);
    // }

    // // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_mandras_palace() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_mandras_palace(&board), 0);

    //     board.place(5, BuildingType::Magenta);
    //     assert_eq!(score_mandras_palace(&board), 0);

    //     board.place(1, BuildingType::Blue);
    //     assert_eq!(score_mandras_palace(&board), 2);

    //     board.place(4, BuildingType::Blue);
    //     assert_eq!(score_mandras_palace(&board), 2);

    //     board.place(4, BuildingType::Red);
    //     assert_eq!(score_mandras_palace(&board), 4);

    //     board.place(6, BuildingType::Yellow);
    //     assert_eq!(score_mandras_palace(&board), 6);

    //     board.place(9, BuildingType::Orange);
    //     assert_eq!(score_mandras_palace(&board), 8);
    // }

    // // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_silva_forum() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_silvia_forum(&board), 0);

    //     board.place(15, BuildingType::Magenta);
    //     assert_eq!(score_silvia_forum(&board), 2);

    //     board.place(0, BuildingType::Blue);
    //     assert_eq!(score_silvia_forum(&board), 2);

    //     board.place(3, BuildingType::Blue);
    //     assert_eq!(score_silvia_forum(&board), 2);

    //     board.place(2, BuildingType::Blue);
    //     assert_eq!(score_silvia_forum(&board), 3);

    //     board.place(1, BuildingType::Blue);
    //     assert_eq!(score_silvia_forum(&board), 5);
    // }

    // // -------------------------------------------------------------------------
    // #[test]
    // fn test_score_the_sky_baths() {
    //     let mut board = Board::new(4, 4);
    //     assert_eq!(score_the_sky_baths(&board), 0);

    //     board.place(15, BuildingType::Magenta);
    //     assert_eq!(score_the_sky_baths(&board), 14);

    //     board.place(0, BuildingType::Black);
    //     assert_eq!(score_the_sky_baths(&board), 12);

    //     board.place(1, BuildingType::Blue);
    //     assert_eq!(score_the_sky_baths(&board), 10);

    //     board.place(2, BuildingType::Gray);
    //     assert_eq!(score_the_sky_baths(&board), 8);

    //     board.place(3, BuildingType::Green);
    //     assert_eq!(score_the_sky_baths(&board), 6);

    //     board.place(4, BuildingType::Black);
    //     assert_eq!(score_the_sky_baths(&board), 6);

    //     board.place(5, BuildingType::Orange);
    //     assert_eq!(score_the_sky_baths(&board), 4);

    //     board.place(6, BuildingType::Red);
    //     assert_eq!(score_the_sky_baths(&board), 2);

    //     board.place(7, BuildingType::Yellow);
    //     assert_eq!(score_the_sky_baths(&board), 0);
    // }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }
}
