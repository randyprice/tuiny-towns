use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::building::{BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;
use crate::score::feed::feed;
use crate::space::Space;

pub mod black;
pub mod blue;
pub mod feed;
pub mod gray;
pub mod green;
pub mod magenta;
pub mod orange;
pub mod yellow;

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
    fn score(&self, map: &HashMap<usize, i32>) -> i32 {
        map.values().into_iter().sum()
    }
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
    pub fn flatten(&self) -> HashMap<usize, i32> {
        let flattened: HashMap<usize, i32> = self.black.clone().into_iter()
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
    pub fn score_black(&self) -> i32 { self.score(&self.black) }
    pub fn score_blue(&self) -> i32 { self.score(&self.blue) }
    pub fn score_gray(&self) -> i32 { self.score(&self.gray) }
    pub fn score_green(&self) -> i32 { self.score(&self.green) }
    pub fn score_magenta(&self) -> i32 { self.score(&self.magenta) }
    pub fn score_orange(&self) -> i32 { self.score(&self.orange) }
    pub fn score_red(&self) -> i32 { self.score(&self.red) }
    pub fn score_yellow(&self) -> i32 { self.score(&self.yellow) }
    pub fn score_unused(&self) -> i32 { self.score(&self.unused) }

}

// -----------------------------------------------------------------------------
// If is_disjoint is false, buildings will score if they are adjacent to any
// building with a BulidingType in adjacent_types; if is_disjont is true,
// they will not score.
fn score_by_adjacency(
    is_disjoint: bool,
    board: &Board,
    building_type: BuildingType,
    adjacent_types: &HashSet<BuildingType>,
    points: i32,
) -> HashMap<usize, i32> {
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
                if space.building_type_eq(building_type) {
                    let points_scored =
                        if board.unique_adjacent_building_types(idx)
                            .is_disjoint(&adjacent_types)
                        == is_disjoint {
                            points
                        } else {
                            0
                        };
                    m.insert(idx, points_scored);
                }
                m
            });

    scores
}

// -----------------------------------------------------------------------------
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
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                if m.len() == 0 {
                    m.insert(idx, points);
                } else {
                    m.insert(idx, 0);
                }
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
fn score_if_adjacent_to(
    board: &Board,
    building_type: BuildingType,
    adjacent_types: &HashSet<BuildingType>,
    points: i32,
) -> HashMap<usize, i32> {
    let scores = score_by_adjacency(
        false,
        board,
        building_type,
        adjacent_types,
        points
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_if_in_idx_set(
    board: &Board,
    idxs: &HashSet<usize>,
    building_type: BuildingType,
    points: i32
) -> HashMap<usize, i32> {
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                if idxs.contains(&idx) {
                    m.insert(idx, points);
                } else {
                    m.insert(idx, 0);
                }
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
fn score_if_not_adjacent_to(
    board: &Board,
    building_type: BuildingType,
    adjacent_types: &HashSet<BuildingType>,
    points: i32,
) -> HashMap<usize, i32> {
    let scores = score_by_adjacency(
        true,
        board,
        building_type,
        adjacent_types,
        points
    );

    scores
}

// -----------------------------------------------------------------------------
fn score_per_each(
    board: &Board,
    building_type: BuildingType,
    points: i32
) -> HashMap<usize, i32> {
    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            if space.building_type_eq(building_type) {
                m.insert(idx, points);
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
fn score_unused_spaces(
    board: &Board,
    building_config: &BuildingConfig,
) -> HashMap<usize, i32> {
    let points =
        if building_config.magenta() == MagentaBuilding::CathedralOfCaterina
        && board.count_building_type(BuildingType::Magenta) > 0 {
            0
        } else {
            -1
        };

    let scores = board.spaces()
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (idx, space)| {
            match space {
                Space::Resource(_) | Space::Empty => {
                    m.insert(idx, points);
                }
                _ => (),
            }
            m
        });

    scores
}

// -----------------------------------------------------------------------------
pub fn score(
    board: &Board,
    building_config: &BuildingConfig,
    other: Option<&Board>,
) -> ScoreCard {
    let fed_idxs = feed(board, building_config);
    let score_card = ScoreCard {
        black: black::score(board, building_config),
        blue: blue::score(board, building_config, &fed_idxs),
        gray: gray::score(board, building_config),
        green: green::score(board, building_config, other),
        magenta: magenta::score(board, building_config, &fed_idxs),
        orange: orange::score(board, building_config, &fed_idxs),
        red: score_per_each(board, BuildingType::Red, 0),
        yellow: yellow::score(board, building_config),
        unused: score_unused_spaces(board, building_config),
    };

    score_card
}

// =============================================================================
#[cfg(test)]
mod test {
    use super::*;
    use crate::building::{
        BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
        MagentaBuilding, OrangeBuilding, RedBuilding, Resource, YellowBuilding
    };

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_by_adjacency() {
        let mut board = Board::new(4, 4);
        let is_disjoint = false;
        let building_type = BuildingType::Blue;
        let adjacent_types = HashSet::from([
            BuildingType::Orange,
            BuildingType::Yellow
        ]);
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
        let points_by_count = HashMap::from([
            (0, 0),
            (2, -3),
            (3, 43),
        ]);
        let default = 9;
        let building_type = BuildingType::Red;
        let result = score_by_count(
            &board,
            building_type,
            &points_by_count,
            default,
        );
        assert!(result.is_empty());

        board.place(0, BuildingType::Blue);
        let result = score_by_count(
            &board,
            building_type,
            &points_by_count,
            default,
        );
        assert!(result.is_empty());

        board.place(1, BuildingType::Red);
        let result = score_by_count(
            &board,
            building_type,
            &points_by_count,
            default,
        );
        assert_eq!(result, HashMap::from([(1, 9)]));

        board.place(2, BuildingType::Red);
        let result = score_by_count(
            &board,
            building_type,
            &points_by_count,
            default,
        );
        assert_eq!(result, HashMap::from([(1, -3), (2, 0)]));

        board.place(3, BuildingType::Red);
        let result = score_by_count(
            &board,
            building_type,
            &points_by_count,
            default,
        );
        assert_eq!(result, HashMap::from([(1, 43), (2, 0), (3, 0)]));

        board.place(4, BuildingType::Red);
        let result = score_by_count(
            &board,
            building_type,
            &points_by_count,
            default,
        );
        assert_eq!(result, HashMap::from([(1, 9), (2, 0), (3, 0), (4, 0)]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_score_if_adjacent_to() {
        let mut board = Board::new(4, 4);
        let building_type = BuildingType::Black;
        let adjacent_types = HashSet::from([
            BuildingType::Orange,
            BuildingType::Yellow
        ]);
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
        let adjacent_types = HashSet::from([
            BuildingType::Blue,
            BuildingType::Gray
        ]);
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
        assert!(score_unused_spaces(&board, &building_config).is_empty());

        board.place(0, Resource::Stone);
        let expected = HashMap::from([(0, -1)]);
        assert_eq!(score_unused_spaces(&board, &building_config), expected);

        board.place(1, Resource::Stone);
        let expected = HashMap::from([(0, -1), (1, -1)]);
        assert_eq!(score_unused_spaces(&board, &building_config), expected);

        board.place(2, BuildingType::Magenta);
        assert_eq!(score_unused_spaces(&board, &building_config), expected);

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
        assert_eq!(score_unused_spaces(&board, &building_config), expected);

        board.place(3, BuildingType::Magenta);
        let expected = HashMap::from([(0, 0), (1, 0)]);
        assert_eq!(score_unused_spaces(&board, &building_config), expected);
    }

    // -------------------------------------------------------------------------
    #[test]
    #[ignore]
    fn test_score() {

    }

}
