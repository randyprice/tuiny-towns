use std::collections::{HashMap, HashSet};

use crate::board::Board;
use crate::building::{BuildingType, MagentaBuilding};
use crate::building_config::BuildingConfig;
use crate::feed::feed;
use crate::space::Space;

pub mod black;
pub mod blue;
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
fn score_if_in_set(
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
fn score_unused_spaces(
    board: &Board,
    building_config: &BuildingConfig,
) -> HashMap<usize, i32> {
    let points =
        if building_config.magenta() == MagentaBuilding::CathedralOfCaterina {
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
fn score_per_each(
    board: &Board,
    building_type: BuildingType,
    points: i32
) -> HashMap<usize, i32> {
    let scores = board.idxs_of_building_type(building_type)
        .into_iter()
        .fold(HashMap::new(), |mut m, idx| {
            m.insert(idx, points);
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

// // =============================================================================
// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::building::{
//         BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
//         MagentaBuilding, OrangeBuilding, RedBuilding, Resource, YellowBuilding
//     };

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_score_by_adjacency() {
//         let mut board = Board::new(4, 4);
//         assert_eq!(
//             score_by_adjacency(
//                 true,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             0,
//         );
//         board.place(0, BuildingType::Blue);
//         assert_eq!(
//             score_by_adjacency(
//                 true,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             0,
//         );
//         assert_eq!(
//             score_by_adjacency(
//                 false,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             2,
//         );

//         board.place(4, BuildingType::Orange);
//         assert_eq!(
//             score_by_adjacency(
//                 true,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             2,
//         );
//         assert_eq!(
//             score_by_adjacency(
//                 false,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             0,
//         );

//         board.place(4, BuildingType::Yellow);
//         assert_eq!(
//             score_by_adjacency(
//                 true,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             2,
//         );
//         assert_eq!(
//             score_by_adjacency(
//                 false,
//                 &board,
//                 BuildingType::Blue,
//                 HashSet::from([BuildingType::Orange, BuildingType::Yellow]),
//                 2),
//             0,
//         );

//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     #[ignore]
//     fn test_score_by_count() {

//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     #[ignore]
//     fn test_score_if_adjacent_to() {

//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     #[ignore]
//     fn test_score_if_fed() {

//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     #[ignore]
//     fn test_score_if_not_adjacent_to() {

//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_score_unused_spaces() {
//         let mut board = Board::new(4, 4);

//         // Without Cathedral of Caterina.
//         let building_config = BuildingConfig::new(
//             BlackBuilding::Factory,
//             BlueBuilding::Cottage,
//             GrayBuilding::Millstone,
//             GreenBuilding::Tavern,
//             MagentaBuilding::OpaleyesWatch,
//             OrangeBuilding::Abbey,
//             RedBuilding::Farm,
//             YellowBuilding::Theater,
//         );
//         assert_eq!(score_unused_spaces(&board, &building_config), -16);

//         board.place(0, Resource::Brick);
//         assert_eq!(score_unused_spaces(&board, &building_config), -16);

//         board.place(1, BuildingType::Blue);
//         assert_eq!(score_unused_spaces(&board, &building_config), -15);

//         // With Cathedral of Caterina.
//         let building_config = BuildingConfig::new(
//             BlackBuilding::Factory,
//             BlueBuilding::Cottage,
//             GrayBuilding::Millstone,
//             GreenBuilding::Tavern,
//             MagentaBuilding::CathedralOfCaterina,
//             OrangeBuilding::Abbey,
//             RedBuilding::Farm,
//             YellowBuilding::Theater,
//         );
//         assert_eq!(score_unused_spaces(&board, &building_config), 0);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     #[ignore]
//     fn test_score_per_each() {

//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     #[ignore]
//     fn test_score() {

//     }

// }
