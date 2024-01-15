use std::collections::{HashMap, HashSet};

use strum::IntoEnumIterator;

use crate::board::space::BuildingType;
use crate::board::Board;
use crate::building_config::{BuildingConfig, MagentaBuilding};
use crate::score::{score_if_in_idx_set, score_per_each, ScoringContext};

// -----------------------------------------------------------------------------
fn score_archive_of_the_second_age(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let num_unique_building_types = board
        .spaces()
        .iter()
        .fold(HashSet::new(), |mut unique_building_types, space| {
            if let Some(building_type) = space.building_type() {
                if building_type != BuildingType::Magenta {
                    unique_building_types.insert(building_type);
                }
            }
            unique_building_types
        })
        .len();

    let scores = score_per_each(
        board,
        BuildingType::Magenta,
        num_unique_building_types as i32
            * scoring_context
                .points_per_unique_building_type_for_archive_of_the_second_age,
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_mandras_palace(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let score = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut scores, (idx, space)| {
            if space.building_type_eq(BuildingType::Magenta) {
                let points = board.unique_adjacent_building_types(idx).len() as i32
                    * scoring_context.points_per_unique_adjacent_building_type_for_mandras_palace;
                scores.insert(idx, points);
            }
            scores
        },
    );

    score
}

// -----------------------------------------------------------------------------
fn score_silvia_forum(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let points = BuildingType::iter()
        // Vector of HashSets of indices of the largest contiguous group of each
        // building type.
        .fold(
            Vec::new(),
            |mut max_contiguous_group_sizes, building_type| {
                let max = board
                    .contiguous_groups(&HashSet::from([building_type]))
                    .iter()
                    .map(|contiguous_group| contiguous_group.len())
                    .max()
                    .unwrap_or(0);
                max_contiguous_group_sizes.push(max);
                max_contiguous_group_sizes
            },
        )
        .into_iter()
        .max()
        .unwrap_or(0) as i32
        * scoring_context
            .points_per_building_in_largest_contiguous_group_for_silva_forum
        + scoring_context.base_points_per_silva_forum;

    let scores = score_per_each(board, BuildingType::Magenta, points);

    scores
}

// -----------------------------------------------------------------------------
fn score_the_sky_baths(
    board: &Board,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let idxs = HashSet::from_iter(0..board.elems());
    let num_unique_building_types =
        board.unique_building_types_in_idx_set(&idxs).len();
    let num_missing_building_types =
        BuildingType::iter().count() - num_unique_building_types;
    let points = num_missing_building_types as i32
        * scoring_context.points_per_missing_building_type_for_the_sky_baths;

    let scores = score_per_each(board, BuildingType::Magenta, points);

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    fed_idxs: &HashSet<usize>,
) -> HashMap<usize, i32> {
    let scores = match building_config.magenta() {
        MagentaBuilding::ArchitectsGuild => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_architects_guild,
        ),
        MagentaBuilding::ArchiveOfTheSecondAge => {
            score_archive_of_the_second_age(board, scoring_context)
        }
        MagentaBuilding::BarrettCastle => score_if_in_idx_set(
            board,
            fed_idxs,
            BuildingType::Magenta,
            scoring_context.points_per_fed_barrett_castle,
        ),
        MagentaBuilding::CathedralOfCaterina => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_cathedral_of_caterina,
        ),
        MagentaBuilding::FortIronweed => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_fort_ironweed,
        ),
        MagentaBuilding::GrandMausoleumOfTheRodina => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_grand_mausoleum_of_the_rodina,
        ),
        MagentaBuilding::ObeliskOfTheCrescent => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_obelisk_of_the_crescent,
        ),
        MagentaBuilding::OpaleyesWatch => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_opaleyes_watch,
        ),
        MagentaBuilding::StatueOfTheBondmaker => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_statue_of_the_bondmaker,
        ),
        MagentaBuilding::GroveUniversity => score_per_each(
            board,
            BuildingType::Magenta,
            scoring_context.points_per_grove_university,
        ),
        MagentaBuilding::MandrasPalace => {
            score_mandras_palace(board, scoring_context)
        }
        MagentaBuilding::ShrineOfTheElderTree
        | MagentaBuilding::TheStarloom => {
            score_per_each(board, BuildingType::Magenta, -99)
        }
        MagentaBuilding::SilvaForum => {
            score_silvia_forum(board, scoring_context)
        }
        MagentaBuilding::TheSkyBaths => {
            score_the_sky_baths(board, scoring_context)
        }
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
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        assert!(score_archive_of_the_second_age(&board, &scoring_context)
            .is_empty());

        board.place(0, BuildingType::Magenta);
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(
            score_archive_of_the_second_age(&board, &scoring_context),
            expected
        );

        board.place(1, BuildingType::Black);
        board.place(2, BuildingType::Blue);
        board.place(3, BuildingType::Green);
        board.place(4, BuildingType::Gray);
        let expected = HashMap::from([(0, 4)]);
        assert_eq!(
            score_archive_of_the_second_age(&board, &scoring_context),
            expected
        );

        board.place(5, BuildingType::Blue);
        assert_eq!(
            score_archive_of_the_second_age(&board, &scoring_context),
            expected
        );

        board.place(6, BuildingType::Orange);
        board.place(7, BuildingType::Red);
        board.place(8, BuildingType::Yellow);
        let expected = HashMap::from([(0, 7)]);
        assert_eq!(
            score_archive_of_the_second_age(&board, &scoring_context),
            expected
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_mandras_palace() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);
        assert!(score_mandras_palace(&board, &scoring_context).is_empty());

        board.place(5, BuildingType::Magenta);
        assert_eq!(
            score_mandras_palace(&board, &scoring_context),
            HashMap::from([(5, 0)])
        );

        board.place(1, BuildingType::Blue);
        assert_eq!(
            score_mandras_palace(&board, &scoring_context),
            HashMap::from([(5, 2)])
        );

        board.place(4, BuildingType::Blue);
        assert_eq!(
            score_mandras_palace(&board, &scoring_context),
            HashMap::from([(5, 2)])
        );

        board.place(4, BuildingType::Red);
        assert_eq!(
            score_mandras_palace(&board, &scoring_context),
            HashMap::from([(5, 4)])
        );

        board.place(6, BuildingType::Yellow);
        assert_eq!(
            score_mandras_palace(&board, &scoring_context),
            HashMap::from([(5, 6)])
        );

        board.place(9, BuildingType::Orange);
        assert_eq!(
            score_mandras_palace(&board, &scoring_context),
            HashMap::from([(5, 8)])
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_silva_forum() {
        let scoring_context = ScoringContext::default();

        let mut board = Board::new(4, 4);
        assert!(score_silvia_forum(&board, &scoring_context).is_empty());

        board.place(15, BuildingType::Magenta);
        assert_eq!(
            score_silvia_forum(&board, &scoring_context),
            HashMap::from([(15, 2)])
        );

        board.place(0, BuildingType::Blue);
        assert_eq!(
            score_silvia_forum(&board, &scoring_context),
            HashMap::from([(15, 2)])
        );

        board.place(3, BuildingType::Blue);
        assert_eq!(
            score_silvia_forum(&board, &scoring_context),
            HashMap::from([(15, 2)])
        );

        board.place(2, BuildingType::Blue);
        assert_eq!(
            score_silvia_forum(&board, &scoring_context),
            HashMap::from([(15, 3)])
        );

        board.place(1, BuildingType::Blue);
        assert_eq!(
            score_silvia_forum(&board, &scoring_context),
            HashMap::from([(15, 5)])
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_the_sky_baths() {
        let mut board = Board::new(4, 4);
        let scoring_context = ScoringContext::default();
        assert!(score_the_sky_baths(&board, &scoring_context).is_empty());

        board.place(15, BuildingType::Magenta);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 14)])
        );

        board.place(0, BuildingType::Black);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 12)])
        );

        board.place(1, BuildingType::Blue);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 10)])
        );

        board.place(2, BuildingType::Gray);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 8)])
        );

        board.place(3, BuildingType::Green);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 6)])
        );

        board.place(4, BuildingType::Black);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 6)])
        );

        board.place(5, BuildingType::Orange);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 4)])
        );

        board.place(6, BuildingType::Red);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 2)])
        );

        board.place(7, BuildingType::Yellow);
        assert_eq!(
            score_the_sky_baths(&board, &scoring_context),
            HashMap::from([(15, 0)])
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {}
}
