use std::collections::{HashMap, HashSet};
use log::{warn};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Resource {
    Brick,
    Glass,
    Stone,
    Wheat,
    Wood
}

#[derive(Copy, Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
pub enum BuildingType {
    Black,
    Blue,
    Gray,
    Green,
    Magenta,
    Orange,
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlackBuilding {
    Bank,
    Factory,
    TradingPost,
    Warehouse,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BlueBuilding {
    Cottage,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GrayBuilding {
    Fountain,
    Millstone,
    Shed,
    Well,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GreenBuilding {
    Almshouse,
    FeastHall,
    Inn,
    Tavern,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MagentaBuilding {
    ArchitectsGuild,
    ArchiveOfTheSecondAge,
    BarrettCastle,
    CathedralOfCaterina,
    FortIronweed,
    GrandMausoleumOfTheRodina,
    GroveUniversity,
    MandrasPalace,
    ObeliskOfTheCrescent,
    OpaleyesWatch,
    ShrineOfTheElderTree,
    SilvaForum,
    StatueOfTheBondmaker,
    TheSkyBaths,
    TheStarloom,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrangeBuilding {
    Abbey,
    Chapel,
    Cloister,
    Temple,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RedBuilding {
    Farm,
    Granary,
    Greenhouse,
    Orchard,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum YellowBuilding {
    Bakery,
    Market,
    Tailor,
    Theater,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Building {
    Black(BlackBuilding),
    Blue(BlueBuilding),
    Gray(GrayBuilding),
    Green(GreenBuilding),
    Magenta(MagentaBuilding),
    Orange(OrangeBuilding),
    Red(RedBuilding),
    Yellow(YellowBuilding),
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Space {
    Building(BuildingType),
    BuildingWithOptResource(BuildingType, Option<Resource>),
    BuildingWithResource(BuildingType, Resource),
    BuildingWithResources(BuildingType, Vec<Resource>, usize),
    Empty,
    Resource(Resource),
}

impl Space {
    pub fn building_type(&self) -> Option<BuildingType> {
        let building_type_opt = match self {
            Space::Building(building_type)
            | Space::BuildingWithOptResource(building_type, _)
            | Space::BuildingWithResource(building_type, _)
            | Space::BuildingWithResources(building_type, _, _) =>
                Some(*building_type),
            _ => None
        };

        building_type_opt
    }
}

// -----------------------------------------------------------------------------
pub trait Build {
    fn to_building(&self) -> Building;
}

impl Build for BlackBuilding {
    fn to_building(&self) -> Building {
        Building::Black(*self)
    }
}

impl Build for BlueBuilding {
    fn to_building(&self) -> Building {
        Building::Blue(*self)
    }
}

impl Build for GrayBuilding {
    fn to_building(&self) -> Building {
        Building::Gray(*self)
    }
}

impl Build for GreenBuilding {
    fn to_building(&self) -> Building {
        Building::Green(*self)
    }
}

impl Build for MagentaBuilding {
    fn to_building(&self) -> Building {
        Building::Magenta(*self)
    }
}

impl Build for OrangeBuilding {
    fn to_building(&self) -> Building {
        Building::Orange(*self)
    }
}

impl Build for RedBuilding {
    fn to_building(&self) -> Building {
        Building::Red(*self)
    }
}

impl Build for YellowBuilding {
    fn to_building(&self) -> Building {
        Building::Yellow(*self)
    }
}

// =============================================================================
pub struct BuildingConfig {
    black: BlackBuilding,
    blue: BlueBuilding,
    gray: GrayBuilding,
    green: GreenBuilding,
    magenta: MagentaBuilding,
    orange: OrangeBuilding,
    red: RedBuilding,
    yellow: YellowBuilding,
}

impl BuildingConfig {
    pub fn new(black: BlackBuilding, blue: BlueBuilding, gray: GrayBuilding,
        green: GreenBuilding, magenta: MagentaBuilding, orange: OrangeBuilding,
        red: RedBuilding, yellow: YellowBuilding
    ) -> Self {
        Self {
            black,
            blue,
            gray,
            green,
            magenta,
            orange,
            red,
            yellow,
        }
    }
    pub fn black(&self) -> BlackBuilding { self.black }
    pub fn blue(&self) -> BlueBuilding { self.blue }
    pub fn gray(&self) -> GrayBuilding { self.gray }
    pub fn green(&self) -> GreenBuilding { self.green }
    pub fn magenta(&self) -> MagentaBuilding { self.magenta }
    pub fn orange(&self) -> OrangeBuilding { self.orange }
    pub fn red(&self) -> RedBuilding { self.red }
    pub fn yellow(&self) -> YellowBuilding { self.yellow }

}

// =============================================================================
pub trait Place {
    fn to_space(self) -> Space;
}

impl Place for Resource {
    fn to_space(self) -> Space {
        Space::Resource(self)
    }
}

impl Place for BuildingType {
    fn to_space(self) -> Space {
        Space::Building(self)
    }
}

impl Place for (BuildingType, Option<Resource>) {
    fn to_space(self) -> Space {
        Space::BuildingWithOptResource(self.0, self.1)
    }
}

impl Place for (BuildingType, Resource) {
    fn to_space(self) -> Space {
        Space::BuildingWithResource(self.0, self.1)
    }
}

impl Place for (BuildingType, Vec<Resource>, usize) {
    fn to_space(self) -> Space {
        Space::BuildingWithResources(self.0, self.1, self.2)
    }
}

// =============================================================================
pub struct Board {
    rows: usize,
    cols: usize,
    elems: usize,
    spaces: Vec<Space>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        let elems = rows * cols;
        let spaces = vec![Space::Empty; elems];
        Self {
            rows,
            cols,
            elems,
            spaces,
        }
    }

    // -------------------------------------------------------------------------
    fn col(&self, idx: usize) -> usize { idx % self.cols }

    // -------------------------------------------------------------------------
    fn row(&self, idx: usize) -> usize { idx / self.cols }

    // -------------------------------------------------------------------------
    pub fn adjacent_idxs(&self, idx: usize) -> HashSet<usize> {
        let mut adjacent_idxs = HashSet::new();

        // Northern neighbor.
        if self.row(idx) > 0 {
            adjacent_idxs.insert(idx - self.cols);
        }

        // Western neighbor.
        if self.col(idx) > 0 {
            adjacent_idxs.insert(idx - 1);
        }

        // Eastern neighbor.
        if self.col(idx) < self.cols - 1 {
            adjacent_idxs.insert(idx + 1);
        }

        // Southern neighbor.
        if self.row(idx) < self.rows - 1 {
            adjacent_idxs.insert(idx + self.cols);
        }

        adjacent_idxs
    }

    // -------------------------------------------------------------------------
    fn diagonal_idxs(&self, idx: usize) -> HashSet<usize> {
        let mut diagonal_idxs = HashSet::new();

        // Northwestern neighbor.
        if self.row(idx) > 0 && self.col(idx) > 0{
            diagonal_idxs.insert(idx - self.cols - 1);
        }

        // Northeastern neighbor.
        if self.row(idx) > 0 && self.col(idx) < self.cols - 1{
            diagonal_idxs.insert(idx - self.cols + 1);
        }

        // Southwestern neighbor.
        if self.row(idx) < self.rows - 1 && self.col(idx) > 0 {
            diagonal_idxs.insert(idx + self.cols - 1);
        }

        // Southeastern neighbor.
        if self.row(idx) < self.rows - 1 && self.col(idx) < self.cols - 1 {
            diagonal_idxs.insert(idx + self.cols + 1);
        }

        diagonal_idxs
    }

    // -------------------------------------------------------------------------
    pub fn corner_idxs(&self) -> HashSet<usize> {
        let corners = HashSet::from([
            0,
            self.cols - 1,
            self.elems - self.cols,
            self.elems - 1,
        ]);

        corners
    }

    // -------------------------------------------------------------------------
    pub fn adjacent_building_types(&self, idx: usize) -> HashSet<BuildingType> {
        let adjacent_types = self.adjacent_idxs(idx)
            .into_iter()
            .fold(HashSet::new(), |mut s, ii| {
                let space = &self.spaces[ii];
                if let Some(building_type) = space.building_type() {
                    s.insert(building_type);
                }
                s
            });

        adjacent_types
    }

    // -------------------------------------------------------------------------
    fn contiguous_group(&self, building_types: &HashSet<BuildingType>, visited: &mut Vec<bool>, idx: usize) -> HashSet<usize> {
        let mut queue = vec![idx];
        let mut group = HashSet::new();
        visited[idx] = true;

        // Breadth-first search to find one contiguous group of buildings of
        // some type in building_types.
        while !queue.is_empty() {
            let cur = queue.pop().unwrap();
            for adjacent_idx in self.adjacent_idxs(cur).into_iter() {
                let space = &self.spaces()[adjacent_idx];
                if let Some(building_type) = space.building_type() {
                    if !visited[adjacent_idx] && building_types.contains(&building_type) {
                        visited[adjacent_idx] = true;
                        queue.push(adjacent_idx);
                    }
                }
            }
            group.insert(cur);
        }

        group
    }

    // -------------------------------------------------------------------------
    pub fn contiguous_groups(&self, building_types: &HashSet<BuildingType>) -> Vec<HashSet<usize>> {
        let contiguous_groups = self.spaces()
            .iter()
            .enumerate()
            .fold((Vec::new(), vec![false; self.elems]), |(mut groups, mut visited), (idx, space)| {
                if let Some(building_type) = space.building_type() {
                    if building_types.contains(&building_type) && !visited[idx] {
                        let group = self.contiguous_group(building_types, &mut visited, idx);
                        groups.push(group);
                    }
                }

                (groups, visited)
            })
            .0;

        contiguous_groups
    }

    // -------------------------------------------------------------------------
    pub fn count_building_type(&self, building_type: BuildingType) -> u32 {
        // TODO - replace with if-let chain with future Rust version.
        let count = self.spaces
            .iter()
            .fold(0, |mut n, space| {
                if let Some(bt) = space.building_type() {
                    if bt == building_type {
                        n += 1;
                    }
                }
                n
            });

        count
    }

    // -------------------------------------------------------------------------
    pub fn place<T>(&mut self, idx: usize, item: T)
    where T: Place
    {
        self.spaces[idx] = item.to_space();
    }

    // -------------------------------------------------------------------------
    pub fn remove(&mut self, idx: usize) {
        self.spaces[idx] = Space::Empty;
    }

    // -------------------------------------------------------------------------
    pub fn print(&self) {
        for (idx, space) in self.spaces().iter().enumerate() {
            let symbol = match space {
                    Space::Building(_)
                    | Space::BuildingWithOptResource(_, _)
                    | Space::BuildingWithResource(_, _)
                    | Space::BuildingWithResources(_, _, _) => "@",
                    Space::Resource(_) => "o",
                    Space::Empty => "_",
            };
            print!("{symbol}");
            if self.col(idx) == self.cols - 1 {
                println!("");
            } else {
                print!(" ");
            }
        }
    }

    // -------------------------------------------------------------------------
    pub fn spaces(&self) -> &Vec<Space> {
        &self.spaces
    }
}

// =============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    #[test]
    fn test_col() {
        let board = Board::new(7, 4);
        assert_eq!(board.col(0), 0);
        assert_eq!(board.col(2), 2);
        assert_eq!(board.col(15),3);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_row() {
        let board = Board::new(3, 5);
        assert_eq!(board.row(0), 0);
        assert_eq!(board.row(7), 1);
        assert_eq!(board.row(10), 2);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_adjacent_idxs() {
        let board = Board::new(7, 5);
        assert_eq!(board.adjacent_idxs(0), HashSet::from([1, 5]));
        assert_eq!(board.adjacent_idxs(1), HashSet::from([0, 2, 6]));
        assert_eq!(board.adjacent_idxs(10), HashSet::from([5, 11, 15]));
        assert_eq!(board.adjacent_idxs(14), HashSet::from([9, 13, 19]));
        assert_eq!(board.adjacent_idxs(32), HashSet::from([27, 31, 33]));
        assert_eq!(board.adjacent_idxs(12), HashSet::from([7, 11, 13, 17]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_diagonal_idxs() {
        let board = Board::new(4, 8);
        assert_eq!(board.diagonal_idxs(0), HashSet::from([9]));
        assert_eq!(board.diagonal_idxs(1), HashSet::from([8, 10]));
        assert_eq!(board.diagonal_idxs(8), HashSet::from([1, 17]));
        assert_eq!(board.diagonal_idxs(15), HashSet::from([6, 22]));
        assert_eq!(board.diagonal_idxs(17), HashSet::from([8, 10, 24, 26]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_corner_idxs() {
        let board = Board::new(7, 8);
        assert_eq!(board.corner_idxs(), HashSet::from([0, 7, 48, 55]));

        let board = Board::new(4, 3);
        assert_eq!(board.corner_idxs(), HashSet::from([0, 2, 9, 11]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_adjacent_building_types() {
        let mut board = Board::new(4, 8);
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Blue);
        board.place(7, BuildingType::Green);
        board.place(8, BuildingType::Orange);
        board.place(9, BuildingType::Yellow);
        board.place(10, BuildingType::Gray);
        board.place(14, BuildingType::Blue);
        board.place(19, BuildingType::Black);
        board.place(21, BuildingType::Red);
        board.place(23, BuildingType::Red);
        board.place(24, BuildingType::Magenta);

        // Two adjacent building of two different types.
        assert_eq!(board.adjacent_building_types(0),
            HashSet::from([BuildingType::Blue, BuildingType::Orange])
        );
        // Three adjacent buildings of three different types.
        assert_eq!(board.adjacent_building_types(9),
            HashSet::from([BuildingType::Blue, BuildingType::Orange,
                BuildingType::Gray
            ])
        );

        // Three adjacent buildings of two different types.
        assert_eq!(board.adjacent_building_types(22),
            HashSet::from([BuildingType::Blue, BuildingType::Red])
        );

        // No adjacent buildings.
        assert_eq!(board.adjacent_building_types(24), HashSet::from([]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_count_building_type() {
        let mut board = Board::new(4, 5);
        board.place(0, BuildingType::Blue);
        board.place(2, BuildingType::Green);
        board.place(8, BuildingType::Gray);
        board.place(13, BuildingType::Blue);

        assert_eq!(board.count_building_type(BuildingType::Black), 0);
        assert_eq!(board.count_building_type(BuildingType::Green), 1);
        assert_eq!(board.count_building_type(BuildingType::Gray), 1);
        assert_eq!(board.count_building_type(BuildingType::Blue), 2);
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_contiguous_group() {
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Green);
        board.place(1, BuildingType::Red);
        board.place(2, BuildingType::Orange);
        board.place(3, BuildingType::Gray);

        let building_types = HashSet::from([
            BuildingType::Green, BuildingType::Gray, BuildingType::Red
        ]);

        let mut visited = vec![false; board.elems];
        let group = board.contiguous_group(&building_types, &mut visited, 0);
        assert_eq!(group, HashSet::from([0, 1]));

        let mut visited = vec![false; board.elems];
        let group = board.contiguous_group(&building_types, &mut visited, 3);
        assert_eq!(group, HashSet::from([3]));

        let mut visited = vec![false; board.elems];
        let group = board.contiguous_group(&building_types, &mut visited, 3);
        assert_eq!(group, HashSet::from([3]));

        board.place(4, BuildingType::Green);
        let building_types = HashSet::from([
            BuildingType::Green, BuildingType::Gray, BuildingType::Red, BuildingType::Orange,
        ]);
        let mut visited = vec![false; board.elems];
        let group = board.contiguous_group(&building_types, &mut visited, 3);
        assert_eq!(group, HashSet::from([0, 1, 2, 3, 4]));
    }

    // -------------------------------------------------------------------------
    #[test]
    fn test_contiguous_groups() {
        let mut board = Board::new(4, 4);
        board.place(0, BuildingType::Blue);
        board.place(1, BuildingType::Blue);
        board.place(2, BuildingType::Magenta);
        board.place(3, BuildingType::Blue);
        board.place(4, BuildingType::Blue);

        let building_types = HashSet::from([BuildingType::Green]);
        let groups = board.contiguous_groups(&building_types);
        assert!(groups.is_empty());

        let building_types = HashSet::from([BuildingType::Blue]);
        let groups = board.contiguous_groups(&building_types);
        let ans = vec![
            HashSet::from([0, 1, 4]),
            HashSet::from([3]),
        ];
        let eq = groups.iter().all(|s| ans.contains(&s))
            && ans.iter().all(|s| groups.contains(&s));
        assert!(eq);

        let building_types = HashSet::from([BuildingType::Blue, BuildingType::Magenta]);
        let groups = board.contiguous_groups(&building_types);
        let ans = vec![
            HashSet::from([0, 1, 2, 3, 4]),
        ];
        let eq = groups.iter().all(|s| ans.contains(&s))
            && ans.iter().all(|s| groups.contains(&s));
        assert!(eq);


    }
}
