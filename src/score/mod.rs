use std::collections::{HashMap, HashSet};

use crate::game::space::BuildingType;
use crate::game::board::Board;
use crate::game::building::{BuildingConfig, MagentaBuilding};
use crate::score::feed::feed;

pub mod black;
pub mod blue;
pub mod feed;
pub mod gray;
pub mod green;
pub mod magenta;
pub mod orange;
pub mod yellow;

pub struct ScoringContext {
    points_per_tailor_in_center: i32,
    base_points_per_tailor: i32,
    points_by_count_for_almshouses: HashMap<u32, i32>,
    points_by_count_for_taverns: HashMap<u32, i32>,
    points_per_abbey: i32,
    points_per_unique_type_for_theaters: i32,
    points_per_bakery: i32,
    points_per_bank: i32,
    points_per_fed_cottage: i32,
    points_per_factory: i32,
    points_per_feast_hall_with_equal_or_lesser_count: i32,
    points_per_feast_hall_with_greater_count: i32,
    points_per_fed_blue_building_for_chapels: i32,
    points_per_fountain: i32,
    points_per_inn: i32,
    points_per_millstone: i32,
    points_per_resource_on_warehouse: i32,
    points_per_yellow_building_for_markets: i32,
    points_per_shed: i32,
    points_per_temple: i32,
    points_per_trading_post: i32,
    points_per_adjacent_blue_building_for_wells: i32,
    points_per_cottage_with_grand_mausoleum_of_the_rodina: i32,
    default_score_for_almshouses: i32,
    default_score_for_taverns: i32,
    points_per_architects_guild: i32,
    points_per_unique_building_type_for_archive_of_the_second_age: i32,
    points_per_unique_adjacent_building_type_for_mandras_palace: i32,
    points_per_fed_barrett_castle: i32,
    points_per_cathedral_of_caterina: i32,
    points_per_fort_ironweed: i32,
    points_per_grand_mausoleum_of_the_rodina: i32,
    points_per_obelisk_of_the_crescent: i32,
    points_per_opaleyes_watch: i32,
    points_per_statue_of_the_bondmaker: i32,
    points_per_grove_university: i32,
    base_points_per_silva_forum: i32,
    points_per_building_in_largest_contiguous_group_for_silva_forum: i32,
    points_per_missing_building_type_for_the_sky_baths: i32,
    adjacent_building_types_for_abbeys: HashSet<BuildingType>,
    points_per_cloister_in_corner: i32,
    equivalent_num_of_blue_buildings_for_barrett_castle: u32,
    min_adjacent_blue_buildings_to_score_temple: u32,
    adjacent_building_types_for_bakeries: HashSet<BuildingType>,
    points_per_unused_space_with_cathedral_of_caterina: i32,
    points_per_unused_space: i32,
}

impl ScoringContext {
    pub fn default() -> Self {
        Self {
            points_per_tailor_in_center: 1,
            base_points_per_tailor: 1,
            points_by_count_for_almshouses: HashMap::from([
                (0, 0),
                (1, -1),
                (2, 5),
                (3, -3),
                (4, 15),
                (5, -5),
            ]),
            points_by_count_for_taverns: HashMap::from([
                (0, 0),
                (1, 2),
                (2, 5),
                (3, 9),
                (4, 14),
            ]),
            points_per_abbey: 3,
            points_per_unique_type_for_theaters: 1,
            points_per_bakery: 3,
            points_per_bank: 4,
            points_per_fed_cottage: 3,
            points_per_factory: 0,
            points_per_feast_hall_with_equal_or_lesser_count: 2,
            points_per_feast_hall_with_greater_count: 3,
            points_per_fed_blue_building_for_chapels: 1,
            points_per_fountain: 2,
            points_per_inn: 3,
            points_per_millstone: 2,
            points_per_resource_on_warehouse: -1,
            points_per_yellow_building_for_markets: 1,
            points_per_shed: 1,
            points_per_temple: 4,
            points_per_trading_post: 1,
            points_per_adjacent_blue_building_for_wells: 1,
            points_per_cottage_with_grand_mausoleum_of_the_rodina: 3,
            default_score_for_almshouses: 26,
            default_score_for_taverns: 20,
            points_per_architects_guild: 1,
            points_per_unique_building_type_for_archive_of_the_second_age: 1,
            points_per_unique_adjacent_building_type_for_mandras_palace: 2,
            points_per_fed_barrett_castle: 5,
            points_per_cathedral_of_caterina: 2,
            points_per_fort_ironweed: 7,
            points_per_grand_mausoleum_of_the_rodina: 0,
            points_per_obelisk_of_the_crescent: 0,
            points_per_opaleyes_watch: 0,
            points_per_statue_of_the_bondmaker: 0,
            points_per_grove_university: 3,
            base_points_per_silva_forum: 1,
            points_per_building_in_largest_contiguous_group_for_silva_forum: 1,
            points_per_missing_building_type_for_the_sky_baths: 2,
            adjacent_building_types_for_abbeys: HashSet::from([
                BuildingType::Black,
                BuildingType::Green,
                BuildingType::Yellow,
            ]),
            points_per_cloister_in_corner: 1,
            equivalent_num_of_blue_buildings_for_barrett_castle: 2,
            min_adjacent_blue_buildings_to_score_temple: 2,
            adjacent_building_types_for_bakeries: HashSet::from([
                BuildingType::Black,
                BuildingType::Red,
            ]),
            points_per_unused_space_with_cathedral_of_caterina: 0,
            points_per_unused_space: -1,
        }
    }
}

// =============================================================================
/// A `ScoreCard`.
pub struct ScoreCard {
    black: HashMap<usize, i32>,
    blue: HashMap<usize, i32>,
    gray: HashMap<usize, i32>,
    green: HashMap<usize, i32>,
    magenta: HashMap<usize, i32>,
    orange: HashMap<usize, i32>,
    red: HashMap<usize, i32>,
    yellow: HashMap<usize, i32>,
    unused: HashMap<usize, i32>,
}

impl ScoreCard {
    /// Return the total score of one building type by passing its respective
    /// `HashMap` to `score`.
    fn score(&self, map: &HashMap<usize, i32>) -> i32 {
        map.values().into_iter().sum()
    }

    // Public functions
    /// Combine the `ScoreCard`'s fields into a single `HashMap`.
    pub fn flatten(&self) -> HashMap<usize, i32> {
        let flattened: HashMap<usize, i32> = self
            .black
            .clone()
            .into_iter()
            .chain(self.blue.clone())
            .chain(self.gray.clone())
            .chain(self.green.clone())
            .chain(self.magenta.clone())
            .chain(self.orange.clone())
            .chain(self.red.clone())
            .chain(self.yellow.clone())
            .chain(self.unused.clone())
            .collect();
        flattened
    }

    /// Return the total score.
    pub fn score_all(&self) -> i32 {
        let score = self.score_black()
            + self.score_blue()
            + self.score_gray()
            + self.score_green()
            + self.score_magenta()
            + self.score_orange()
            + self.score_red()
            + self.score_yellow()
            + self.score_unused();

        score
    }

    /// Return the total score of all black buildings.
    pub fn score_black(&self) -> i32 {
        self.score(&self.black)
    }

    /// Return the total score of all blue buildings.
    pub fn score_blue(&self) -> i32 {
        self.score(&self.blue)
    }

    /// Return the total score of all gray buildings.
    pub fn score_gray(&self) -> i32 {
        self.score(&self.gray)
    }

    /// Return the total score of all green buildings.
    pub fn score_green(&self) -> i32 {
        self.score(&self.green)
    }

    /// Return the total score of all magenta buildings.
    pub fn score_magenta(&self) -> i32 {
        self.score(&self.magenta)
    }

    /// Return the total score of all orange buildings.
    pub fn score_orange(&self) -> i32 {
        self.score(&self.orange)
    }

    /// Return the total score of all red buildings.
    pub fn score_red(&self) -> i32 {
        self.score(&self.red)
    }

    /// Return the total score of all yellow buildings.
    pub fn score_yellow(&self) -> i32 {
        self.score(&self.yellow)
    }

    /// Return the total score of all unused spaces.
    pub fn score_unused(&self) -> i32 {
        self.score(&self.unused)
    }
}

// -----------------------------------------------------------------------------
/// Score buildings of a given type based on adjacency to the building types
/// in `adjacent_types`. If `is_disjoint` is `false`, buildings of type
/// `building_type` will score if they are adjacent to any types in
/// `adjacent_types`; if `is_disjont` is true, they will not score.
fn score_by_adjacency(
    is_disjoint: bool,
    board: &Board,
    building_type: BuildingType,
    adjacent_types: &HashSet<BuildingType>,
    points: i32,
) -> HashMap<usize, i32> {
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                let points_scored = if board
                    .unique_adjacent_building_types(idx)
                    .is_disjoint(&adjacent_types)
                    == is_disjoint
                {
                    points
                } else {
                    0
                };
                m.insert(idx, points_scored);
            }
            m
        },
    );

    scores
}

// -----------------------------------------------------------------------------
/// Score based on the total number of buildings of type `building_type` as
/// indicated in `points_by_count` and `default`. The first `building_type`
/// building is assigned the score, and the rest are given a score of `0`.
fn score_by_count(
    board: &Board,
    building_type: BuildingType,
    points_by_count: &HashMap<u32, i32>,
    default: i32,
) -> HashMap<usize, i32> {
    assert!(points_by_count.contains_key(&0));
    assert_eq!(points_by_count.get(&0).copied().unwrap(), 0);
    let count = board.count_building_type(building_type);
    let points = *points_by_count.get(&count).unwrap_or(&default);
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                if m.len() == 0 {
                    m.insert(idx, points);
                } else {
                    m.insert(idx, 0);
                }
            }
            m
        },
    );

    scores
}

// -----------------------------------------------------------------------------
/// Score each `building_type` building if it is adjacent to any building types
/// in `adjacent_types`.
fn score_if_adjacent_to(
    board: &Board,
    building_type: BuildingType,
    adjacent_types: &HashSet<BuildingType>,
    points: i32,
) -> HashMap<usize, i32> {
    let scores =
        score_by_adjacency(false, board, building_type, adjacent_types, points);

    scores
}

// -----------------------------------------------------------------------------
/// Score each `building_type` building if `idxs` contains its index.
fn score_if_in_idx_set(
    board: &Board,
    idxs: &HashSet<usize>,
    building_type: BuildingType,
    points: i32,
) -> HashMap<usize, i32> {
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                if idxs.contains(&idx) {
                    m.insert(idx, points);
                } else {
                    m.insert(idx, 0);
                }
            }
            m
        },
    );

    scores
}

// -----------------------------------------------------------------------------
/// Score each `building_type` buildings if it is not adjacent to any building
/// types in `adjacent_types`.
fn score_if_not_adjacent_to(
    board: &Board,
    building_type: BuildingType,
    adjacent_types: &HashSet<BuildingType>,
    points: i32,
) -> HashMap<usize, i32> {
    let scores =
        score_by_adjacency(true, board, building_type, adjacent_types, points);

    scores
}

// -----------------------------------------------------------------------------
/// Score each `building_type` building.
fn score_per_each(
    board: &Board,
    building_type: BuildingType,
    points: i32,
) -> HashMap<usize, i32> {
    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                m.insert(idx, points);
            }
            m
        },
    );

    scores
}

// -----------------------------------------------------------------------------
/// Score unused spaces.
fn score_unused_spaces(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
) -> HashMap<usize, i32> {
    let points = if building_config.magenta()
        == MagentaBuilding::CathedralOfCaterina
        && board.count_building_type(BuildingType::Magenta) > 0
    {
        scoring_context.points_per_unused_space_with_cathedral_of_caterina
    } else {
        scoring_context.points_per_unused_space
    };

    let scores = board.spaces().iter().enumerate().fold(
        HashMap::new(),
        |mut scores, (idx, space)| {
            if space.is_unused() {
                scores.insert(idx, points);
            }
            scores
        },
    );

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    scoring_context: &ScoringContext,
    other: Option<&Board>,
) -> ScoreCard {
    let fed_idxs = feed(board, building_config, scoring_context);
    let score_card = ScoreCard {
        black: black::score(board, building_config, scoring_context),
        blue: blue::score(board, building_config, scoring_context, &fed_idxs),
        gray: gray::score(board, building_config, scoring_context),
        green: green::score(board, building_config, scoring_context, other),
        magenta: magenta::score(
            board,
            building_config,
            scoring_context,
            &fed_idxs,
        ),
        orange: orange::score(
            board,
            building_config,
            scoring_context,
            &fed_idxs,
        ),
        red: score_per_each(board, BuildingType::Red, 0),
        yellow: yellow::score(board, building_config, scoring_context),
        unused: score_unused_spaces(board, building_config, scoring_context),
    };

    score_card
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::game::space::Resource;
    use crate::game::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_by_adjacency() {
        let mut board = Board::new(4, 4);
        let is_disjoint = false;
        let building_type = BuildingType::Blue;
        let adjacent_types =
            HashSet::from([BuildingType::Orange, BuildingType::Yellow]);
        let points = 2;

        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert!(result.is_empty());

        board.place(0, BuildingType::Blue);
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(result, expected);

        let is_disjoint = true;
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        let expected = HashMap::from([(0, 2)]);
        assert_eq!(result, expected);

        board.place(4, BuildingType::Orange);
        let is_disjoint = false;
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, expected);

        let is_disjoint = true;
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(result, expected);

        board.place(1, BuildingType::Green);
        board.place(4, BuildingType::Yellow);
        let is_disjoint = false;
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        let expected = HashMap::from([(0, 2)]);
        assert_eq!(result, expected);

        let is_disjoint = true;
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        let expected = HashMap::from([(0, 0)]);
        assert_eq!(result, expected);

        board.place(2, BuildingType::Blue);
        let result = score_by_adjacency(
            is_disjoint,
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        let expected = HashMap::from([(0, 0), (2, 2)]);
        assert_eq!(result, expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_by_count() {
        let mut board = Board::new(4, 4);
        let points_by_count = HashMap::from([(0, 0), (2, -3), (3, 43)]);
        let default = 9;
        let building_type = BuildingType::Red;
        let result =
            score_by_count(&board, building_type, &points_by_count, default);
        assert!(result.is_empty());

        board.place(0, BuildingType::Blue);
        let result =
            score_by_count(&board, building_type, &points_by_count, default);
        assert!(result.is_empty());

        board.place(1, BuildingType::Red);
        let result =
            score_by_count(&board, building_type, &points_by_count, default);
        assert_eq!(result, HashMap::from([(1, 9)]));

        board.place(2, BuildingType::Red);
        let result =
            score_by_count(&board, building_type, &points_by_count, default);
        assert_eq!(result, HashMap::from([(1, -3), (2, 0)]));

        board.place(3, BuildingType::Red);
        let result =
            score_by_count(&board, building_type, &points_by_count, default);
        assert_eq!(result, HashMap::from([(1, 43), (2, 0), (3, 0)]));

        board.place(4, BuildingType::Red);
        let result =
            score_by_count(&board, building_type, &points_by_count, default);
        assert_eq!(result, HashMap::from([(1, 9), (2, 0), (3, 0), (4, 0)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_if_adjacent_to() {
        let mut board = Board::new(4, 4);
        let building_type = BuildingType::Black;
        let adjacent_types =
            HashSet::from([BuildingType::Orange, BuildingType::Yellow]);
        let points = 3;
        let result = score_if_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert!(result.is_empty());

        board.place(0, BuildingType::Black);
        let result = score_if_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 0)]));

        board.place(2, BuildingType::Orange);
        let result = score_if_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 0)]));

        board.place(1, BuildingType::Black);
        let result = score_if_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 0), (1, 3)]));

        board.place(4, BuildingType::Yellow);
        let result = score_if_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 3), (1, 3)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_if_in_idx_set() {
        let mut board = Board::new(4, 4);
        let mut idxs: HashSet<usize> = HashSet::new();
        let building_type = BuildingType::Yellow;
        let points = 4;

        let result = score_if_in_idx_set(&board, &idxs, building_type, points);
        assert!(result.is_empty());

        board.place(0, BuildingType::Red);
        let result = score_if_in_idx_set(&board, &idxs, building_type, points);
        assert!(result.is_empty());

        board.place(1, BuildingType::Yellow);
        let result = score_if_in_idx_set(&board, &idxs, building_type, points);
        assert_eq!(result, HashMap::from([(1, 0)]));

        idxs.insert(0);
        let result = score_if_in_idx_set(&board, &idxs, building_type, points);
        assert_eq!(result, HashMap::from([(1, 0)]));

        idxs.insert(1);
        let result = score_if_in_idx_set(&board, &idxs, building_type, points);
        assert_eq!(result, HashMap::from([(1, 4)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_if_not_adjacent_to() {
        let mut board = Board::new(4, 4);
        let building_type = BuildingType::Green;
        let adjacent_types =
            HashSet::from([BuildingType::Blue, BuildingType::Gray]);
        let points = 5;
        let result = score_if_not_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert!(result.is_empty());

        board.place(0, BuildingType::Green);
        let result = score_if_not_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 5)]));

        board.place(1, BuildingType::Gray);
        let result = score_if_not_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 0)]));

        board.place(3, BuildingType::Green);
        let result = score_if_not_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 0), (3, 5)]));

        board.place(2, BuildingType::Blue);
        let result = score_if_not_adjacent_to(
            &board,
            building_type,
            &adjacent_types,
            points,
        );
        assert_eq!(result, HashMap::from([(0, 0), (3, 0)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_per_each() {
        let mut board = Board::new(4, 4);
        let building_type = BuildingType::Black;
        let points = 6;
        assert!(score_per_each(&board, building_type, points).is_empty());

        board.place(0, BuildingType::Orange);
        assert!(score_per_each(&board, building_type, points).is_empty());

        board.place(1, BuildingType::Black);
        let expected = HashMap::from([(1, 6)]);
        assert_eq!(score_per_each(&board, building_type, points), expected);

        board.place(2, BuildingType::Black);
        let expected = HashMap::from([(1, 6), (2, 6)]);
        assert_eq!(score_per_each(&board, building_type, points), expected);

        let building_type = BuildingType::Orange;
        let expected = HashMap::from([(0, 6)]);
        assert_eq!(score_per_each(&board, building_type, points), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_unused_spaces() {
        let scoring_context = ScoringContext::default();
        let mut board = Board::new(4, 4);

        // Without Cathedral of Caterina.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Millstone,
            GreenBuilding::Tavern,
            MagentaBuilding::OpaleyesWatch,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        for idx in 0..board.elems() {
            board.place(idx, BuildingType::Red);
        }
        assert!(score_unused_spaces(
            &board,
            &building_config,
            &scoring_context
        )
        .is_empty());

        board.place(0, Resource::Stone);
        let expected = HashMap::from([(0, -1)]);
        assert_eq!(
            score_unused_spaces(&board, &building_config, &scoring_context),
            expected
        );

        board.place(1, Resource::Stone);
        let expected = HashMap::from([(0, -1), (1, -1)]);
        assert_eq!(
            score_unused_spaces(&board, &building_config, &scoring_context),
            expected
        );

        board.place(2, BuildingType::Magenta);
        assert_eq!(
            score_unused_spaces(&board, &building_config, &scoring_context),
            expected
        );

        // With Cathedral of Caterina.
        let building_config = BuildingConfig::new(
            BlackBuilding::Factory,
            BlueBuilding::Cottage,
            GrayBuilding::Millstone,
            GreenBuilding::Tavern,
            MagentaBuilding::CathedralOfCaterina,
            OrangeBuilding::Abbey,
            RedBuilding::Farm,
            YellowBuilding::Theater,
        );
        board.place(2, BuildingType::Blue);
        assert_eq!(
            score_unused_spaces(&board, &building_config, &scoring_context),
            expected
        );

        board.place(3, BuildingType::Magenta);
        let expected = HashMap::from([(0, 0), (1, 0)]);
        assert_eq!(
            score_unused_spaces(&board, &building_config, &scoring_context),
            expected
        );
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {}
}
