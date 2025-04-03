// Module that defines the state of a game.

// use std::collections::{HashMap, HashSet};

// use crate::game::space::{BuildingType, Place, Space};

pub mod board;
pub mod building;
pub mod space;

// // =============================================================================
// pub struct Board {
//     rows: usize,
//     cols: usize,
//     elems: usize,
//     spaces: Vec<Space>,
// }

// impl Board {
//     pub fn new(rows: usize, cols: usize) -> Self {
//         assert!(rows > 2);
//         assert!(cols > 2);
//         let elems = rows * cols;
//         let spaces = vec![Space::Empty; elems];
//         Self {
//             rows,
//             cols,
//             elems,
//             spaces,
//         }
//     }

//     // -------------------------------------------------------------------------
//     pub fn cols(&self) -> usize {
//         self.cols
//     }

//     // -------------------------------------------------------------------------
//     pub fn rows(&self) -> usize {
//         self.rows
//     }

//     // -------------------------------------------------------------------------
//     pub fn elems(&self) -> usize {
//         self.elems
//     }

//     // -------------------------------------------------------------------------
//     pub fn col(&self, idx: usize) -> usize {
//         idx % self.cols
//     }

//     // -------------------------------------------------------------------------
//     pub fn row(&self, idx: usize) -> usize {
//         idx / self.cols
//     }

//     // -------------------------------------------------------------------------
//     pub fn idx(&self, row: usize, col: usize) -> usize {
//         row * self.cols + col
//     }

//     // -------------------------------------------------------------------------
//     // Methods that return a HashSet of indices.
//     // -------------------------------------------------------------------------
//     pub fn adjacent_idxs(&self, idx: usize) -> HashSet<usize> {
//         let mut adjacent_idxs = HashSet::new();

//         // Northern neighbor.
//         if self.row(idx) > 0 {
//             adjacent_idxs.insert(idx - self.cols);
//         }

//         // Western neighbor.
//         if self.col(idx) > 0 {
//             adjacent_idxs.insert(idx - 1);
//         }

//         // Eastern neighbor.
//         if self.col(idx) < self.cols - 1 {
//             adjacent_idxs.insert(idx + 1);
//         }

//         // Southern neighbor.
//         if self.row(idx) < self.rows - 1 {
//             adjacent_idxs.insert(idx + self.cols);
//         }

//         adjacent_idxs
//     }

//     // -------------------------------------------------------------------------
//     fn diagonal_idxs(&self, idx: usize) -> HashSet<usize> {
//         let mut diagonal_idxs = HashSet::new();

//         // Northwestern neighbor.
//         if self.row(idx) > 0 && self.col(idx) > 0 {
//             diagonal_idxs.insert(idx - self.cols - 1);
//         }

//         // Northeastern neighbor.
//         if self.row(idx) > 0 && self.col(idx) < self.cols - 1 {
//             diagonal_idxs.insert(idx - self.cols + 1);
//         }

//         // Southwestern neighbor.
//         if self.row(idx) < self.rows - 1 && self.col(idx) > 0 {
//             diagonal_idxs.insert(idx + self.cols - 1);
//         }

//         // Southeastern neighbor.
//         if self.row(idx) < self.rows - 1 && self.col(idx) < self.cols - 1 {
//             diagonal_idxs.insert(idx + self.cols + 1);
//         }

//         diagonal_idxs
//     }

//     // -------------------------------------------------------------------------
//     pub fn surrounding_idxs(&self, idx: usize) -> HashSet<usize> {
//         let mut surrounding_idxs = self.adjacent_idxs(idx);
//         surrounding_idxs.extend(self.diagonal_idxs(idx));

//         surrounding_idxs
//     }

//     // -------------------------------------------------------------------------
//     pub fn center_idxs(&self) -> HashSet<usize> {
//         assert!(self.rows % 2 == 0);
//         assert!(self.cols % 2 == 0);

//         let southeast_center = self.rows / 2 * self.cols + self.cols / 2;
//         let center_idxs = HashSet::from([
//             southeast_center - self.cols - 1,
//             southeast_center - self.cols,
//             southeast_center - 1,
//             southeast_center,
//         ]);

//         center_idxs
//     }

//     // -------------------------------------------------------------------------
//     pub fn corner_idxs(&self) -> HashSet<usize> {
//         let corner_idxs = HashSet::from([
//             0,
//             self.cols - 1,
//             self.elems - self.cols,
//             self.elems - 1,
//         ]);

//         corner_idxs
//     }

//     /// Count the number of `Space::Building(building_type)` in each row and
//     /// column.
//     pub fn count_building_type_per_row_and_col(
//         &self,
//         building_type: BuildingType,
//     ) -> (HashMap<usize, u32>, HashMap<usize, u32>) {
//         let (count_per_row, count_per_col) =
//             self.spaces.iter().enumerate().fold(
//                 (HashMap::new(), HashMap::new()),
//                 |(mut count_per_row, mut count_per_col), (idx, space)| {
//                     if space.building_type_eq(building_type) {
//                         *count_per_row.entry(self.row(idx)).or_insert(0) += 1;
//                         *count_per_col.entry(self.col(idx)).or_insert(0) += 1;
//                     }
//                     (count_per_row, count_per_col)
//                 },
//             );

//         (count_per_row, count_per_col)
//     }

//     // -------------------------------------------------------------------------
//     // Return the set of different building types in the spaces specified by
//     // idxs.
//     pub fn unique_building_types_in_idx_set(
//         &self,
//         idxs: &HashSet<usize>,
//     ) -> HashSet<BuildingType> {
//         let unique_building_types =
//             idxs.into_iter().fold(HashSet::new(), |mut s, idx| {
//                 let space = &self.spaces[*idx];
//                 if let Some(building_type) = space.building_type() {
//                     s.insert(building_type);
//                 }
//                 s
//             });

//         unique_building_types
//     }

//     // -------------------------------------------------------------------------
//     pub fn unique_adjacent_building_types(
//         &self,
//         idx: usize,
//     ) -> HashSet<BuildingType> {
//         let unique_adjacent_building_types =
//             self.unique_building_types_in_idx_set(&self.adjacent_idxs(idx));

//         unique_adjacent_building_types
//     }

//     // -------------------------------------------------------------------------
//     pub fn unique_surrounding_building_types(
//         &self,
//         idx: usize,
//     ) -> HashSet<BuildingType> {
//         let unique_surrounding_building_types =
//             self.unique_building_types_in_idx_set(&self.surrounding_idxs(idx));

//         unique_surrounding_building_types
//     }

//     // -------------------------------------------------------------------------
//     fn contiguous_group(
//         &self,
//         building_types: &HashSet<BuildingType>,
//         visited: &mut Vec<bool>,
//         idx: usize,
//     ) -> HashSet<usize> {
//         let mut queue = vec![idx];
//         let mut group = HashSet::new();
//         visited[idx] = true;

//         // Breadth-first search to find one contiguous group of buildings of
//         // some type in building_types.
//         while !queue.is_empty() {
//             let cur = queue.pop().unwrap();
//             for adjacent_idx in self.adjacent_idxs(cur).into_iter() {
//                 let space = &self.spaces()[adjacent_idx];
//                 if let Some(building_type) = space.building_type() {
//                     if !visited[adjacent_idx]
//                         && building_types.contains(&building_type)
//                     {
//                         visited[adjacent_idx] = true;
//                         queue.push(adjacent_idx);
//                     }
//                 }
//             }
//             group.insert(cur);
//         }

//         group
//     }

//     // -------------------------------------------------------------------------
//     pub fn contiguous_groups(
//         &self,
//         building_types: &HashSet<BuildingType>,
//     ) -> Vec<HashSet<usize>> {
//         let contiguous_groups = self
//             .spaces()
//             .iter()
//             .enumerate()
//             .fold(
//                 (Vec::new(), vec![false; self.elems]),
//                 |(mut groups, mut visited), (idx, space)| {
//                     if let Some(building_type) = space.building_type() {
//                         if building_types.contains(&building_type)
//                             && !visited[idx]
//                         {
//                             let group = self.contiguous_group(
//                                 &building_types,
//                                 &mut visited,
//                                 idx,
//                             );
//                             groups.push(group);
//                         }
//                     }
//                     (groups, visited)
//                 },
//             )
//             .0;

//         contiguous_groups
//     }

//     // -------------------------------------------------------------------------
//     pub fn count_building_type(&self, building_type: BuildingType) -> u32 {
//         // TODO - replace with if-let chain with future Rust version.
//         let count = self.spaces.iter().fold(0, |mut n, space| {
//             if let Some(bt) = space.building_type() {
//                 if bt == building_type {
//                     n += 1;
//                 }
//             }
//             n
//         });

//         count
//     }

//     // -------------------------------------------------------------------------
//     // Return the number of buildings adjacent to space #idx that are of
//     // a type specified in building_types.
//     pub fn count_adjacent_buildings(
//         &self,
//         idx: usize,
//         adjacent_types: &HashSet<BuildingType>,
//     ) -> u32 {
//         let count = self.adjacent_idxs(idx).iter().fold(0, |mut n, idx| {
//             let space = &self.spaces()[*idx];
//             if let Some(building_type) = space.building_type() {
//                 if adjacent_types.contains(&building_type) {
//                     n += 1;
//                 }
//             }
//             n
//         });

//         count
//     }

//     // -------------------------------------------------------------------------
//     pub fn place<T>(&mut self, idx: usize, item: T)
//     where
//         T: Place,
//     {
//         self.spaces[idx] = item.to_space();
//     }

//     // -------------------------------------------------------------------------
//     pub fn remove(&mut self, idx: usize) {
//         self.spaces[idx] = Space::Empty;
//     }

//     // -------------------------------------------------------------------------
//     pub fn spaces(&self) -> &Vec<Space> {
//         &self.spaces
//     }
// }

// // =============================================================================
// #[cfg(test)]
// mod tests {
//     use super::*;

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_col() {
//         let board = Board::new(7, 4);
//         assert_eq!(board.col(0), 0);
//         assert_eq!(board.col(2), 2);
//         assert_eq!(board.col(15), 3);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_row() {
//         let board = Board::new(3, 5);
//         assert_eq!(board.row(0), 0);
//         assert_eq!(board.row(7), 1);
//         assert_eq!(board.row(10), 2);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_idx() {
//         let board = Board::new(5, 4);
//         assert_eq!(board.idx(0, 0), 0);
//         assert_eq!(board.idx(1, 0), 4);
//         assert_eq!(board.idx(0, 1), 1);
//         assert_eq!(board.idx(2, 3), 11);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_adjacent_idxs() {
//         let board = Board::new(7, 5);
//         assert_eq!(board.adjacent_idxs(0), HashSet::from([1, 5]));
//         assert_eq!(board.adjacent_idxs(1), HashSet::from([0, 2, 6]));
//         assert_eq!(board.adjacent_idxs(10), HashSet::from([5, 11, 15]));
//         assert_eq!(board.adjacent_idxs(14), HashSet::from([9, 13, 19]));
//         assert_eq!(board.adjacent_idxs(32), HashSet::from([27, 31, 33]));
//         assert_eq!(board.adjacent_idxs(12), HashSet::from([7, 11, 13, 17]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_diagonal_idxs() {
//         let board = Board::new(4, 8);
//         assert_eq!(board.diagonal_idxs(0), HashSet::from([9]));
//         assert_eq!(board.diagonal_idxs(1), HashSet::from([8, 10]));
//         assert_eq!(board.diagonal_idxs(8), HashSet::from([1, 17]));
//         assert_eq!(board.diagonal_idxs(15), HashSet::from([6, 22]));
//         assert_eq!(board.diagonal_idxs(17), HashSet::from([8, 10, 24, 26]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_surrounding_idxs() {
//         let board = Board::new(5, 5);
//         let surrounding_idxs = board.surrounding_idxs(0);
//         assert_eq!(surrounding_idxs, HashSet::from([1, 5, 6]));

//         let surrounding_idxs = board.surrounding_idxs(10);
//         assert_eq!(surrounding_idxs, HashSet::from([5, 6, 11, 15, 16]));

//         let surrounding_idxs = board.surrounding_idxs(18);
//         assert_eq!(
//             surrounding_idxs,
//             HashSet::from([12, 13, 14, 17, 19, 22, 23, 24])
//         );
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_corner_idxs() {
//         let board = Board::new(7, 8);
//         assert_eq!(board.corner_idxs(), HashSet::from([0, 7, 48, 55]));

//         let board = Board::new(4, 3);
//         assert_eq!(board.corner_idxs(), HashSet::from([0, 2, 9, 11]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_center_idxs() {
//         let board = Board::new(4, 4);
//         assert_eq!(board.center_idxs(), HashSet::from([5, 6, 9, 10]));

//         let board = Board::new(6, 6);
//         assert_eq!(board.center_idxs(), HashSet::from([14, 15, 20, 21]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_unique_building_types() {
//         let mut board = Board::new(7, 8);

//         board.place(0, BuildingType::Blue);
//         let unique_building_types =
//             board.unique_building_types_in_idx_set(&HashSet::from([1]));
//         assert!(unique_building_types.is_empty());

//         let unique_building_types =
//             board.unique_building_types_in_idx_set(&HashSet::from([0]));
//         assert_eq!(unique_building_types, HashSet::from([BuildingType::Blue]));

//         board.place(1, BuildingType::Red);
//         let unique_building_types =
//             board.unique_building_types_in_idx_set(&HashSet::from([0, 1]));
//         assert_eq!(
//             unique_building_types,
//             HashSet::from([BuildingType::Blue, BuildingType::Red])
//         );

//         board.place(6, BuildingType::Green);
//         let unique_building_types =
//             board.unique_building_types_in_idx_set(&HashSet::from([1, 5, 9]));
//         assert_eq!(unique_building_types, HashSet::from([BuildingType::Red]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_unique_adjacent_building_types() {
//         let mut board = Board::new(4, 8);
//         board.place(0, BuildingType::Blue);
//         board.place(1, BuildingType::Blue);
//         board.place(7, BuildingType::Green);
//         board.place(8, BuildingType::Orange);
//         board.place(9, BuildingType::Yellow);
//         board.place(10, BuildingType::Gray);
//         board.place(14, BuildingType::Blue);
//         board.place(19, BuildingType::Black);
//         board.place(21, BuildingType::Red);
//         board.place(23, BuildingType::Red);
//         board.place(24, BuildingType::Magenta);

//         // Two adjacent building of two different types.
//         assert_eq!(
//             board.unique_adjacent_building_types(0),
//             HashSet::from([BuildingType::Blue, BuildingType::Orange])
//         );
//         // Three adjacent buildings of three different types.
//         assert_eq!(
//             board.unique_adjacent_building_types(9),
//             HashSet::from([
//                 BuildingType::Blue,
//                 BuildingType::Orange,
//                 BuildingType::Gray
//             ])
//         );

//         // Three adjacent buildings of two different types.
//         assert_eq!(
//             board.unique_adjacent_building_types(22),
//             HashSet::from([BuildingType::Blue, BuildingType::Red])
//         );

//         // No adjacent buildings.
//         assert_eq!(board.unique_adjacent_building_types(24), HashSet::from([]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_unique_surrounding_building_types() {
//         let mut board = Board::new(5, 6);

//         board.place(7, BuildingType::Blue);
//         let adjacent_types = board.unique_surrounding_building_types(7);
//         assert!(adjacent_types.is_empty());

//         board.place(6, BuildingType::Red);
//         let adjacent_types = board.unique_surrounding_building_types(7);
//         assert_eq!(adjacent_types, HashSet::from([BuildingType::Red]));

//         board.place(8, BuildingType::Red);
//         let adjacent_types = board.unique_surrounding_building_types(7);
//         assert_eq!(adjacent_types, HashSet::from([BuildingType::Red]));

//         board.place(1, BuildingType::Blue);
//         let adjacent_types = board.unique_surrounding_building_types(7);
//         assert_eq!(
//             adjacent_types,
//             HashSet::from([BuildingType::Blue, BuildingType::Red])
//         );

//         board.place(0, BuildingType::Blue);
//         let adjacent_types = board.unique_surrounding_building_types(7);
//         assert_eq!(
//             adjacent_types,
//             HashSet::from([BuildingType::Blue, BuildingType::Red])
//         );

//         board.place(14, BuildingType::Green);
//         let adjacent_types = board.unique_surrounding_building_types(7);
//         assert_eq!(
//             adjacent_types,
//             HashSet::from([
//                 BuildingType::Blue,
//                 BuildingType::Green,
//                 BuildingType::Red
//             ])
//         );
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_count_building_type() {
//         let mut board = Board::new(4, 5);
//         board.place(0, BuildingType::Blue);
//         board.place(2, BuildingType::Green);
//         board.place(8, BuildingType::Gray);
//         board.place(13, BuildingType::Blue);

//         assert_eq!(board.count_building_type(BuildingType::Black), 0);
//         assert_eq!(board.count_building_type(BuildingType::Green), 1);
//         assert_eq!(board.count_building_type(BuildingType::Gray), 1);
//         assert_eq!(board.count_building_type(BuildingType::Blue), 2);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_count_adjacent_buildings() {
//         let mut board = Board::new(4, 4);

//         board.place(0, BuildingType::Blue);
//         let count = board.count_adjacent_buildings(0, &HashSet::new());
//         assert_eq!(count, 0);

//         let count = board.count_adjacent_buildings(1, &HashSet::new());
//         assert_eq!(count, 0);

//         let count = board
//             .count_adjacent_buildings(1, &HashSet::from([BuildingType::Blue]));
//         assert_eq!(count, 1);

//         board.place(1, BuildingType::Blue);
//         board.place(4, BuildingType::Red);
//         let count = board
//             .count_adjacent_buildings(5, &HashSet::from([BuildingType::Blue]));
//         assert_eq!(count, 1);

//         let count = board.count_adjacent_buildings(
//             5,
//             &HashSet::from([BuildingType::Blue, BuildingType::Red]),
//         );
//         assert_eq!(count, 2);

//         board.place(6, BuildingType::Blue);
//         let count = board.count_adjacent_buildings(
//             5,
//             &HashSet::from([BuildingType::Blue, BuildingType::Red]),
//         );
//         assert_eq!(count, 3);

//         board.place(9, BuildingType::Green);
//         let count = board.count_adjacent_buildings(
//             5,
//             &HashSet::from([BuildingType::Blue, BuildingType::Red]),
//         );
//         assert_eq!(count, 3);

//         let count = board.count_adjacent_buildings(
//             5,
//             &HashSet::from([
//                 BuildingType::Blue,
//                 BuildingType::Green,
//                 BuildingType::Red,
//             ]),
//         );
//         assert_eq!(count, 4);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_contiguous_group() {
//         let mut board = Board::new(4, 4);
//         board.place(0, BuildingType::Green);
//         board.place(1, BuildingType::Red);
//         board.place(2, BuildingType::Orange);
//         board.place(3, BuildingType::Gray);

//         let building_types = HashSet::from([
//             BuildingType::Green,
//             BuildingType::Gray,
//             BuildingType::Red,
//         ]);

//         let mut visited = vec![false; board.elems];
//         let group = board.contiguous_group(&building_types, &mut visited, 0);
//         assert_eq!(group, HashSet::from([0, 1]));

//         let mut visited = vec![false; board.elems];
//         let group = board.contiguous_group(&building_types, &mut visited, 3);
//         assert_eq!(group, HashSet::from([3]));

//         let mut visited = vec![false; board.elems];
//         let group = board.contiguous_group(&building_types, &mut visited, 3);
//         assert_eq!(group, HashSet::from([3]));

//         board.place(4, BuildingType::Green);
//         let building_types = HashSet::from([
//             BuildingType::Green,
//             BuildingType::Gray,
//             BuildingType::Red,
//             BuildingType::Orange,
//         ]);
//         let mut visited = vec![false; board.elems];
//         let group = board.contiguous_group(&building_types, &mut visited, 3);
//         assert_eq!(group, HashSet::from([0, 1, 2, 3, 4]));
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_contiguous_groups() {
//         let mut board = Board::new(4, 4);
//         board.place(0, BuildingType::Blue);
//         board.place(1, BuildingType::Blue);
//         board.place(2, BuildingType::Magenta);
//         board.place(3, BuildingType::Blue);
//         board.place(4, BuildingType::Blue);

//         let building_types = HashSet::from([BuildingType::Green]);
//         let groups = board.contiguous_groups(&building_types);
//         assert!(groups.is_empty());

//         let building_types = HashSet::from([BuildingType::Blue]);
//         let groups = board.contiguous_groups(&building_types);
//         let ans = vec![HashSet::from([0, 1, 4]), HashSet::from([3])];
//         let eq = groups.iter().all(|s| ans.contains(&s))
//             && ans.iter().all(|s| groups.contains(&s));
//         assert!(eq);

//         let building_types =
//             HashSet::from([BuildingType::Blue, BuildingType::Magenta]);
//         let groups = board.contiguous_groups(&building_types);
//         let ans = vec![HashSet::from([0, 1, 2, 3, 4])];
//         let eq = groups.iter().all(|s| ans.contains(&s))
//             && ans.iter().all(|s| groups.contains(&s));
//         assert!(eq);
//     }

//     // -------------------------------------------------------------------------
//     #[test]
//     fn test_count_building_type_per_row_and_col() {
//         let mut board = Board::new(4, 4);
//         let (count_per_row, count_per_col) =
//             board.count_building_type_per_row_and_col(BuildingType::Black);
//         assert!(count_per_row.is_empty());
//         assert!(count_per_col.is_empty());

//         board.place(0, BuildingType::Black);
//         board.place(2, BuildingType::Black);
//         board.place(5, BuildingType::Red);

//         let (count_per_row, count_per_col) =
//             board.count_building_type_per_row_and_col(BuildingType::Black);
//         assert_eq!(count_per_row, HashMap::from([(0, 2)]));
//         assert_eq!(count_per_col, HashMap::from([(0, 1), (2, 1)]));

//         let (count_per_row, count_per_col) =
//             board.count_building_type_per_row_and_col(BuildingType::Red);
//         assert_eq!(count_per_row, HashMap::from([(1, 1)]));
//         assert_eq!(count_per_col, HashMap::from([(1, 1)]));
//     }
// }
