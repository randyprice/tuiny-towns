use std::collections::HashSet;
use std::cmp;

use colored::Colorize;
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Resource {
    Brick,
    Glass,
    Stone,
    Wheat,
    Wood
}

#[derive(Copy, Clone, Debug, EnumIter, Eq, Hash, PartialEq)]
enum Building {
    Black,
    Blue,
    Gray,
    Green,
    Magenta,
    Orange,
    Red,
    Yellow,
}

#[derive(Copy, Clone, Debug)]
enum OrangeBuilding {
    Abbey,
    Chapel,
    Cloister,
    Temple,
}

#[derive(Copy, Clone, Debug)]
enum BlueBuilding {
    Cottage,
}

#[derive(Copy, Clone, Debug)]
enum BlackBuilding {
    Bank,
    Factory,
    TradingPost,
    Warehouse,
}

#[derive(Copy, Clone, Debug)]
enum RedBuilding {
    Farm,
    Granary,
    Greenhouse,
    Orchard,
}

#[derive(Copy, Clone, Debug)]
enum GreenBuilding {
    Almshouse,
    FeastHall,
    Inn,
    Tavern,
}

#[derive(Copy, Clone, Debug)]
enum YellowBuilding {
    Bakery,
    Market,
    Tailor,
    Theater,
}

#[derive(Copy, Clone, Debug)]
enum GrayBuilding {
    Fountain,
    Millstone,
    Shed,
    Well,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum MagentaBuilding {
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

#[derive(Copy, Clone, Debug)]
enum BuildingVariant {
    WellVariant(GrayBuilding)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum SpaceOccupant {
    Building(Building, Vec<Resource>),
    Resource(Resource),
    None
}

struct Town {
    rows: usize,
    cols: usize,
    elems: usize,
    spaces: Vec<SpaceOccupant>,
    chapel_variant: OrangeBuilding,
    cottage_variant: BlueBuilding,
    factory_variant: BlackBuilding,
    farm_variant: RedBuilding,
    tavern_variant: GreenBuilding,
    theater_variant: YellowBuilding,
    well_variant: GrayBuilding,
    monument: MagentaBuilding,
}

// =============================================================================
impl Town {

    // -------------------------------------------------------------------------
    pub fn new(
        rows: usize, cols: usize,
        chapel_variant: OrangeBuilding,
        cottage_variant: BlueBuilding,
        factory_variant: BlackBuilding,
        farm_variant: RedBuilding,
        tavern_variant: GreenBuilding,
        theater_variant: YellowBuilding,
        well_variant: GrayBuilding,
        monument: MagentaBuilding,
    ) -> Self {
        assert!(rows > 1);
        assert!(cols > 1);
        let elems = rows * cols;
        Self {
            rows,
            cols,
            elems,
            spaces: vec![SpaceOccupant::None; elems],
            chapel_variant,
            cottage_variant,
            factory_variant,
            farm_variant,
            tavern_variant,
            theater_variant,
            well_variant,
            monument,
        }
    }

    // -------------------------------------------------------------------------
    pub fn set_occupant(&mut self, idx: usize, occupant: SpaceOccupant) {
        self.spaces[idx] = occupant;
    }

    // -------------------------------------------------------------------------
    pub fn get_occupant(&self, idx: usize) -> &SpaceOccupant {
        &self.spaces[idx]
    }

    // -------------------------------------------------------------------------
    pub fn count_building_type(&self, building_type: Building) -> u32{
        let count = self.spaces
            .iter()
            .fold(0, |n, s| match s {
                SpaceOccupant::Building(building, _) =>
                    if *building == building_type {
                        n + 1
                    } else {
                        n
                    },
                _ => n,
            });

        count
    }

    // -------------------------------------------------------------------------
    pub fn get_adjacent_idxs(&self, idx: usize) -> Vec<usize> {
        let mut adjacent_idxs = Vec::new();

        // Northern neighbor.
        if self.row(idx) > 0 {
            adjacent_idxs.push(idx - self.cols);
        }

        // Western neighbor.
        if self.col(idx) > 0 {
            adjacent_idxs.push(idx - 1);
        }

        // Eastern neighbor.
        if self.col(idx) < self.cols - 1 {
            adjacent_idxs.push(idx + 1);
        }

        // Southern neighbor.
        if self.row(idx) < self.rows - 1 {
            adjacent_idxs.push(idx + self.cols);
        }

        adjacent_idxs
    }

    // -------------------------------------------------------------------------
    pub fn get_diagonal_idxs(&self, idx: usize) -> Vec<usize> {
        let mut diagonal_idxs = Vec::new();

        // Northwestern neighbor.
        if self.row(idx) > 0 && self.col(idx) > 0{
            diagonal_idxs.push(idx - self.cols - 1);
        }

        // Northeastern neighbor.
        if self.row(idx) > 0 && self.col(idx) < self.cols - 1{
            diagonal_idxs.push(idx - self.cols + 1);
        }

        // Southwestern neighbor.
        if self.row(idx) < self.rows - 1 && self.col(idx) > 0 {
            diagonal_idxs.push(idx + self.cols - 1);
        }

        // Southeastern neighbor.
        if self.row(idx) < self.rows - 1 && self.col(idx) < self.cols - 1 {
            diagonal_idxs.push(idx + self.cols + 1);
        }

        diagonal_idxs
    }

    // -------------------------------------------------------------------------
    fn get_surrounding_idxs(&self, idx: usize) -> Vec<usize> {
        let mut surrounding_idxs = self.get_adjacent_idxs(idx);
        surrounding_idxs.append(&mut self.get_diagonal_idxs(idx));

        surrounding_idxs
    }
    // -------------------------------------------------------------------------
    // fn same_col_idxs(&self, idx: usize) -> Vec<usize> {
        // let col = idx % self.cols;
        // let same_col_idxs = (0..4).into_iter()
        //     .map(|x| 4 * x + col)
        //     .collect();

        // same_col_idxs
    // }

    // -------------------------------------------------------------------------
    // fn same_row_idxs(&self, idx: usize) -> Vec<usize> {
    //     let col = idx % self.cols;
    //     let same_row_idxs = (idx - col..idx - col + 4).collect();

    //     same_row_idxs
    // }

    // -------------------------------------------------------------------------
    fn col(&self, idx: usize) -> usize {
        idx % self.cols
    }

    // -------------------------------------------------------------------------
    fn row(&self, idx: usize) -> usize {
        idx / self.rows
    }

    // -------------------------------------------------------------------------
    fn rowcol(&self, idx: usize) -> (usize, usize) {
        (self.row(idx), self.col(idx))
    }

    // -------------------------------------------------------------------------
    fn corners(&self) -> HashSet<usize> {
        let corners = HashSet::from(
            [0, self.cols - 1, self.elems - self.cols, self.elems - 1]);

        corners
    }

    // -------------------------------------------------------------------------
    fn score_by_adjacency_to(
        &self,
        building_type: Building,
        adjacent_types: HashSet<Building>,
        points_per: i32,
        score_if_adjacent: bool,
    ) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(building, _) =>
                    if *building == building_type && self.get_adjacent_idxs(idx)
                        .iter()
                        .any(|ii| match &self.spaces[*ii] {
                            SpaceOccupant::Building(bt, _) => adjacent_types.contains(bt),
                            _ => false,
                        }) == score_if_adjacent {
                        n + points_per
                    } else {
                        n
                    },
                _ => n,
            });

        score
    }

    // -------------------------------------------------------------------------
    fn score_if_adjacent_to(
        &self,
        building_type: Building,
        adjacent_types: HashSet<Building>,
        points_per: i32,
    ) -> i32 {
        // Score each building of building_type if it is adjacent to any type
        // in adjacent_types.
        let score = self.score_by_adjacency_to(building_type, adjacent_types, points_per, true);

        score
    }

    // -------------------------------------------------------------------------
    fn score_if_not_adjacent_to(
        &self,
        building_type: Building,
        adjacent_types: HashSet<Building>,
        points_per: i32,
    ) -> i32 {
        // Score each building of building_type if it is adjacent to any type
        // in adjacent_types.
        let score = self.score_by_adjacency_to(building_type, adjacent_types, points_per, false);

        score
    }

    // -------------------------------------------------------------------------
    fn do_bfs(
        &self, idx: usize, building_types: &HashSet<Building>, explored: &mut Vec<bool>
    ) -> Vec<usize> {
        // Create queue.
        let mut queue = vec![idx; 1];
        explored[idx] = true;
        let mut contiguous_group = Vec::new();

        // Explore.
        while !queue.is_empty() {
            // The element to explore.
            let cur = queue.pop().unwrap();

            // Add unexplored neighbors of building_type to queue.
            for ii in self.get_adjacent_idxs(cur).iter() {
                match &self.spaces[*ii] {
                    SpaceOccupant::Building(buliding_type, _) =>
                        if building_types.contains(buliding_type) && !explored[*ii] {
                            explored[*ii] = true;
                            queue.push(*ii);

                        },
                    _ => (),
                }
            }

            // Done with this element.
            contiguous_group.push(cur);
        }

        contiguous_group
    }

    // -------------------------------------------------------------------------
    fn get_contiguous_groups(&self, building_types: HashSet<Building>) -> Vec<Vec<usize>> {
        let (contiguous_groups, _) = self.spaces
            .iter()
            .enumerate()
            // gs: Vec<Vec<usize>>
            //     vector of vectors of indices of contiguous buildings of
            //     type building_type
            // v: Vec<bool>
            //     vector of booleans indicating whether or not its index
            //     has been explored in the bfs
            .fold((Vec::new(), vec![false; self.elems]), |(mut gs, mut v), (idx, s)| match s {
                SpaceOccupant::Building(building, _) => {
                    if building_types.contains(&building) && !v[idx] {
                        let group = self.do_bfs(idx, &building_types, &mut v);
                        gs.push(group);
                    }
                    (gs, v)
                },
                _ => (gs, v)
        });

        contiguous_groups
    }

    // -------------------------------------------------------------------------
    fn get_feedable_buildings(&self) -> Vec<usize> {
        let feedable_buildings = self.spaces
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut v, (idx, s)| match s {
                SpaceOccupant::Building(Building::Blue, _) => {
                    v.push(idx);
                    v
                },
                SpaceOccupant::Building(Building::Magenta, _) => {
                    if self.monument == MagentaBuilding::BarrettCastle {
                        v.push(idx);
                    }
                    v
                },
                _ => v,
            });

        println!("feedable buildings: {:?}", feedable_buildings);
        feedable_buildings
    }

    // -------------------------------------------------------------------------
    fn get_best_fed_building_permutation(&self, permutations: Vec<Vec<usize>>) -> Vec<usize> {
        let best_permutation = permutations
            .iter()
            .fold((Vec::new(), 0), |(best, max), p| {
                let score = self.score_cottage_variants(p, false)
                    + self.score_chapel_variants(p, false)
                    + self.score_monuments(p, false);
                if score > max {
                    (p.clone(), score)
                } else {
                    (best, max)
                }
            }).0;

        best_permutation
    }

    // -------------------------------------------------------------------------
    fn get_fed_buildings_for_farms(&self) -> Vec<usize> {
        let feedable_buildings = self.get_feedable_buildings();
        let n_feedable = feedable_buildings.len();
        let permutations = feedable_buildings.into_iter()
                    .combinations(cmp::min(
                        4 * self.count_building_type(Building::Red) as usize,
                        n_feedable))
                    .collect_vec();

        let fed_buildings = self.get_best_fed_building_permutation(permutations);

        fed_buildings
    }

    // -------------------------------------------------------------------------
    fn get_fed_buildings_for_granaries(&self) -> Vec<usize> {
        let (fed_rows, fed_cols) = self.spaces
            .iter()
            .enumerate()
            .fold((HashSet::new(), HashSet::new()), |(mut rows, mut cols), (idx, s)| match s {
                SpaceOccupant::Building(Building::Red, _) => {
                    rows.insert(self.row(idx));
                    cols.insert(self.col(idx));
                    (rows, cols)
                },
                _ => (rows, cols)
            });

        let fed_buildings = self.get_feedable_buildings()
            .into_iter()
            .fold(Vec::new(), |mut v, idx| {
                if fed_rows.contains(&self.row(idx)) || fed_cols.contains(&self.col(idx)) {
                    v.push(idx);
                }
                v
            });

        fed_buildings
    }

    // -------------------------------------------------------------------------
    fn get_fed_buildings_for_greenhouses(&self) -> Vec<usize> {
        let building_types = if self.monument == MagentaBuilding::BarrettCastle {
            HashSet::from([Building::Blue, Building::Magenta])
        } else {
            HashSet::from([Building::Blue])
        };

        // Each greenhouse feeds one contiguous group; if there is
        // more than one greenhouse, combine 2+ contiguous groups
        // to form one permutation.
        let permutations = self.get_contiguous_groups(building_types)
            .into_iter()
            .combinations(self.count_building_type(Building::Red) as usize)
            .collect_vec()
            .iter_mut()
            // Create vector of fed building permutations.
            .fold(Vec::new(), |mut vs, v| {
                vs.push(
                    v.iter_mut()
                        // Combine contiguous groups to create one permutation.
                        .fold(Vec::new(), |mut p, u| {
                            p.append(u);
                            p
                        }));
                vs
            });

        let fed_buildings = self.get_best_fed_building_permutation(permutations);

        fed_buildings
    }

    // -------------------------------------------------------------------------
    fn get_fed_buildings_for_orchards(&self) -> Vec<usize> {
        let fed_buildings = self.get_feedable_buildings()
            .into_iter()
            .fold(Vec::new(), |mut v, idx| {
                if self.get_surrounding_idxs(idx)
                    .into_iter()
                    .any(|ii| match self.spaces[ii] {
                        SpaceOccupant::Building(Building::Red, _) => true,
                        _ => false,
                    }) {
                    v.push(idx);
                }
                v
            });

        fed_buildings
    }
    // -------------------------------------------------------------------------
    fn get_fed_buildings(&self) -> Vec<usize> {
        let fed_buildings = match self.farm_variant {
            RedBuilding::Farm => self.get_fed_buildings_for_farms(),
            RedBuilding::Greenhouse => self.get_fed_buildings_for_greenhouses(),
            RedBuilding::Granary => self.get_fed_buildings_for_granaries(),
            RedBuilding::Orchard => self.get_fed_buildings_for_orchards(),
        };
        println!("fed_buildings: {:?}", fed_buildings);

        fed_buildings
    }

    // SCORING METHODS =========================================================

    fn score_abbeys(&self) -> i32 {
        let score = self.score_if_not_adjacent_to(
            Building::Orange,
            HashSet::from([Building::Black,
                Building::Green,
                Building::Yellow]),
            3);

        score
    }

    // -------------------------------------------------------------------------
    fn score_chapels(&self, fed_buildings: &Vec<usize>) -> i32 {
        let score = fed_buildings
            .iter()
            .fold(0, |n, idx| match self.spaces[*idx] {
                SpaceOccupant::Building(Building::Blue, _) => n + 1,
                SpaceOccupant::Building(Building::Magenta, _) =>
                    if self.monument == MagentaBuilding::BarrettCastle {
                        n + 2
                    } else {
                        n
                    },
                _ => n,
            })
            * self.count_building_type(Building::Orange)
            as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_cloisters(&self) -> i32 {
        let (cloisters, cloisters_in_corners) = self.spaces
            .iter()
            .enumerate()
            .fold((0, 0), |(n, m), (idx, s)| match s {
                SpaceOccupant::Building(Building::Orange, _) =>
                    if self.corners().contains(&idx) {
                        (n + 1, m + 1)
                    } else {
                        (n + 1, m)
                    },
                _ => (n, m)
            });

        let score = cloisters * cloisters_in_corners as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_temples(&self, fed_buildings: &Vec<usize>) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Orange, _) =>
                    if self.get_adjacent_idxs(idx)
                        .iter()
                        .fold(0, |m, ii|
                            if fed_buildings.contains(&ii) {
                                match self.spaces[*ii] {
                                    SpaceOccupant::Building(Building::Blue, _) =>
                                        m + 1,
                                    SpaceOccupant::Building(Building::Magenta, _) =>
                                        if self.monument == MagentaBuilding::BarrettCastle {
                                            m + 2
                                        } else {
                                            m
                                        },
                                    _ => panic!("bad fed building index"),
                                }
                            } else {
                                m
                            }
                        )
                        >= 2 {
                        n + 4
                    } else {
                        n
                    },
                _ => n,
            });

        score
    }

    // -------------------------------------------------------------------------
    fn score_cottages(&self, fed_buildings: &Vec<usize>) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Blue, _) =>
                    if self.monument == MagentaBuilding::GrandMausoleumOfTheRodina
                        || fed_buildings.contains(&idx) {
                        n + 3
                    } else {
                        n
                    },
                _ => n,
            });

        score
    }
    // -------------------------------------------------------------------------
    fn score_banks(&self) -> i32 {
        let score = self.count_building_type(Building::Black)
            as i32
            * 4;

        score
    }

    // -------------------------------------------------------------------------
    fn score_factories(&self) -> i32 {
        0
    }

    // -------------------------------------------------------------------------
    fn score_trading_posts(&self) -> i32 {
        let score = self.count_building_type(Building::Black)
            as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_warehouses(&self) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Black, resources) =>
                    n + resources.len(),
                _ => n,
            })
            as i32
            * -1;
        println!("warehouses score {score}");

        score
    }

    // -------------------------------------------------------------------------
    fn score_almshouses(&self) -> i32 {
        let count = self.count_building_type(Building::Green);
        let score = match count {
            0 => 0,
            1 => -1,
            2 => 5,
            3 => -3,
            4 => 15,
            5 => -5,
            _ => 26
        };

        score
    }

    // -------------------------------------------------------------------------
    fn score_feast_halls(&self, other: &Town) -> i32 {
        let my_count = self.count_building_type(Building::Green);
        let other_count = other.count_building_type(Building::Green);
        let points_per = if my_count > other_count {
            3
        } else {
            2
        };
        let score = points_per * my_count as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_inns(&self) -> i32 {
        // Counts inns per row and column.
        let (inns_per_row, inns_per_col) = self.spaces
            .iter()
            .enumerate()
            .fold((vec![0; 4], vec![0; 4]), |(mut r, mut c), (idx, s)| match s {
                SpaceOccupant::Building(Building::Green, _) => {
                    r[self.row(idx)] += 1;
                    c[self.col(idx)] += 1;
                    (r, c)
                },
                _ => (r, c),
        });

        // Score inns whose inn-per-row/col count is 1 (i.e. they're the only
        // inn in their row/col).
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Green, _) => {
                    if inns_per_row[self.row(idx)] == 1 && inns_per_col[self.col(idx)] == 1 {
                        n + 3
                    } else {
                        n
                    }
                },
                _ => n,
            });

        score
    }

    // -------------------------------------------------------------------------
    fn score_taverns(&self) -> i32 {
        let count = self.count_building_type(Building::Green);
        let score = match count {
            0 => 0,
            1 => 2,
            2 => 5,
            3 => 9,
            4 => 14,
            _ => 20,
        };

        score
    }

    // -------------------------------------------------------------------------
    fn score_bakeries(&self) -> i32 {
        let score = self.score_if_adjacent_to(
            Building::Yellow,
            HashSet::from([Building::Black, Building::Red]),
            3);

        score
    }

    // -------------------------------------------------------------------------
    fn score_markets(&self) -> i32 {
        // Count the number of markets in each row and column.
        let (markets_in_row, markets_in_col) = self.spaces
            .iter()
            .enumerate()
            .fold((vec![0; 4], vec![0; 4]), |(mut r, mut c), (idx, s)| match s {
                SpaceOccupant::Building(Building::Yellow, _) => {
                    r[self.row(idx)] += 1;
                    c[self.col(idx)] += 1;
                    (r, c)
                },
                _ => (r, c),
        });

        // Score each market.
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Yellow, _) => {
                    n + std::cmp::max(markets_in_row[self.row(idx)], markets_in_col[self.col(idx)])
                },
                _ => n,
            });

        score
    }

    // -------------------------------------------------------------------------
    fn score_tailors(&self) -> i32 {
        let center_idxs = vec![5, 6, 9, 10];
        let (tailors, tailors_in_center) = self.spaces
            .iter()
            .enumerate()
            .fold((0, 0), |(n, m), (idx, s)| match s {
                SpaceOccupant::Building(Building::Yellow, _) => {
                    let c = if center_idxs.contains(&idx) {
                        1
                    } else {
                        0
                    };
                    (n + 1, m + c)
                },
                _ => (n, m),

            });
        let score = tailors * (tailors_in_center + 1);

        score
    }

    // -------------------------------------------------------------------------
    fn score_theaters(&self) -> i32 {
        // Sets of unique buildings in each row and column.
        let (uniques_in_row, uniques_in_col) = self.spaces
            .iter()
            .enumerate()
            .fold((vec![HashSet::new(); self.rows], vec![HashSet::new(); self.cols]), |(mut r, mut c), (idx, s)| match s {
                SpaceOccupant::Building(Building::Yellow, _) => (r, c),
                SpaceOccupant::Building(building_type, _) => {
                    r[self.row(idx)].insert(building_type);
                    c[self.col(idx)].insert(building_type);
                    (r, c)
                },
                _ => (r, c)
            });

        // Score each theater by using the union of unique buildings in each row
        // and column.
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Yellow, _) => {
                    let row_uniques = &uniques_in_row[self.row(idx)];
                    let col_uniques = &uniques_in_col[self.col(idx)];
                    let uniques = row_uniques.union(col_uniques);
                    n + uniques.count()
                },
                _ => n
            })
            as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_fountains(&self) -> i32 {
        let score = self.get_contiguous_groups(HashSet::from([Building::Gray]))
            .iter()
            .filter(|v| v.len() > 1)
            .fold(0, |n, v| 2 * v.len())
            as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_millstones(&self) -> i32 {
        let score = self.score_if_adjacent_to(
            Building::Gray,
            HashSet::from([Building::Red, Building::Yellow]),
            2);

        score
    }

    // -------------------------------------------------------------------------
    fn score_sheds(&self) -> i32 {
        let score = self.count_building_type(Building::Gray) as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_wells(&self) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Gray, _) =>
                    n + self.get_adjacent_idxs(idx)
                        .iter()
                        .fold(0, |c, ii| match self.spaces[*ii] {
                            SpaceOccupant::Building(Building::Blue, _) => c + 1,
                            _ => c,
                        }),
                _ => n,
            })
            as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_chapel_variants(&self, fed_buildings: &Vec<usize>, p: bool) -> i32 {
        let score = match self.chapel_variant {
            OrangeBuilding::Abbey => self.score_abbeys(),
            OrangeBuilding::Chapel => self.score_chapels(fed_buildings),
            OrangeBuilding::Cloister => self.score_cloisters(),
            OrangeBuilding::Temple => self.score_temples(fed_buildings),
        };
        if p {
            println!("chapel variants score {score}");
        }

        score
    }

    // -------------------------------------------------------------------------
    fn score_cottage_variants(&self, fed_buildings: &Vec<usize>, p: bool) -> i32 {
        let score = match self.cottage_variant {
            BlueBuilding::Cottage => self.score_cottages(fed_buildings)
        };
        if p {
            println!("cottage variants score {score}");
        }

        score
    }

    // -------------------------------------------------------------------------
    fn score_factory_variants(&self) -> i32 {
        let score = match self.factory_variant {
            BlackBuilding::Bank => self.score_banks(),
            BlackBuilding::Factory => self.score_factories(),
            BlackBuilding::TradingPost => self.score_trading_posts(),
            BlackBuilding::Warehouse => self.score_warehouses(),
        };

        println!("factory variants score {score}");
        score
    }

    // -------------------------------------------------------------------------
    fn score_tavern_variants(&self, other: &Town) -> i32 {
        let score = match self.tavern_variant {
            GreenBuilding::Almshouse => self.score_almshouses(),
            GreenBuilding::FeastHall => self.score_feast_halls(other),
            GreenBuilding::Inn => self.score_inns(),
            GreenBuilding::Tavern => self.score_taverns(),
        };
        println!("tavern variants score {score}");
        score
    }

    // -------------------------------------------------------------------------
    fn score_theater_variants(&self) -> i32 {
        let score = match self.theater_variant {
            YellowBuilding::Bakery => self.score_bakeries(),
            YellowBuilding::Market => self.score_markets(),
            YellowBuilding::Tailor => self.score_tailors(),
            YellowBuilding::Theater => self.score_theaters(),
        };

        println!("theater variants score {score}");
        score
    }

    // -------------------------------------------------------------------------
    fn score_well_variants(&self) -> i32 {
        let score = match self.well_variant {
            GrayBuilding::Fountain => self.score_fountains(),
            GrayBuilding::Millstone => self.score_millstones(),
            GrayBuilding::Shed => self.score_sheds(),
            GrayBuilding::Well => self.score_wells(),
        };

        println!("well variants score {score}");
        score
    }

    // -------------------------------------------------------------------------
    fn score_architects_guild(&self) -> i32 {
        let score = self.count_building_type(Building::Magenta) as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_archive_of_the_second_age(&self) -> i32 {
        let score = self.spaces
            .iter()
            .fold(HashSet::new(), |mut m, s| match s {
                SpaceOccupant::Building(Building::Magenta, _) => m,
                SpaceOccupant::Building(building_type, _) => {
                    m.insert(building_type);
                    m
                }
                _ => m,
            })
            .len() as i32
            * self.count_building_type(Building::Magenta) as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_barret_castle(&self, fed_buildings: &Vec<usize>) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Magenta, _) =>
                    if fed_buildings.contains(&idx) {
                        n + 5
                    } else {
                        n
                    },
                _ => n,
            });

        score
    }

    // -------------------------------------------------------------------------
    fn score_cathedral_of_caterina(&self) -> i32 {
        let score = self.count_building_type(Building::Magenta)
            as i32
            * 2;

        score
    }

    // -------------------------------------------------------------------------
    fn score_fort_ironweed(&self) -> i32 {
        let score = self.count_building_type(Building::Magenta)
            as i32
            * 7;

        score
    }

    // -------------------------------------------------------------------------
    fn score_grove_university(&self) -> i32 {
        let score: i32 = self.count_building_type(Building::Magenta)
            as i32
            * 3;

        score
    }

    // -------------------------------------------------------------------------
    fn score_mandras_palace(&self) -> i32 {
        let score = self.spaces
            .iter()
            .enumerate()
            .fold(0, |n, (idx, s)| match s {
                SpaceOccupant::Building(Building::Magenta, _) =>
                    n + self.get_adjacent_idxs(idx)
                        .into_iter()
                        .fold(HashSet::new(), |mut m, ii| match self.spaces[ii] {
                            SpaceOccupant::Building(building_type, _) => {
                                m.insert(building_type);
                                m
                            },
                            _ => m,
                        })
                        .len() * 2,
                    _ => n,
            })
            as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_silva_forum(&self) -> i32{
        let score = (*Building::iter()
            .fold(Vec::new(), |mut v, b| {
                v.push(
                    self.get_contiguous_groups(HashSet::from([b]))
                        .iter()
                        .map(|v| v.len())
                        .max()
                        .unwrap_or(0));
                v
            })
            .iter()
            .max()
            .unwrap_or(&0)
            + 1)
            as i32
            * self.count_building_type(Building::Magenta) as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_the_sky_baths(&self) -> i32 {
        let unique_types = self.spaces
            .iter()
            .fold(HashSet::new(), |mut m, s| match s {
                SpaceOccupant::Building(building_type, _) => {
                    m.insert(*building_type);
                    m
                }
                _ => m,
            })
            .len();
        let missing_types = Building::iter()
            .count()
            - unique_types;

        let score = 2
            * missing_types as i32
            * self.count_building_type(Building::Magenta) as i32;

        score
    }

    // -------------------------------------------------------------------------
    fn score_monuments(&self, fed_buildings: &Vec<usize>, p: bool) -> i32 {
        let score = match self.monument {
            MagentaBuilding::ArchitectsGuild => self.score_architects_guild(),
            MagentaBuilding::ArchiveOfTheSecondAge => self.score_archive_of_the_second_age(),
            MagentaBuilding::BarrettCastle => self.score_barret_castle(fed_buildings),
            MagentaBuilding::CathedralOfCaterina => self.score_cathedral_of_caterina(),
            MagentaBuilding::FortIronweed => self.score_fort_ironweed(),
            MagentaBuilding::GrandMausoleumOfTheRodina => 0,
            MagentaBuilding::GroveUniversity => self.score_grove_university(),
            MagentaBuilding::MandrasPalace => self.score_mandras_palace(),
            MagentaBuilding::ObeliskOfTheCrescent => 0,
            MagentaBuilding::OpaleyesWatch => 0,
            MagentaBuilding::ShrineOfTheElderTree => -99,
            MagentaBuilding::SilvaForum => self.score_silva_forum(),
            MagentaBuilding::StatueOfTheBondmaker => 0,
            MagentaBuilding::TheSkyBaths => self.score_the_sky_baths(),
            MagentaBuilding::TheStarloom => -99,
        };


        if p {
            println!("monuments score {score}");
        }

        score
    }

    // -------------------------------------------------------------------------
    fn score_empty_spaces(&self) -> i32 {
        let score = match self.monument {
            MagentaBuilding::CathedralOfCaterina => 0,
            _ => self.spaces
                .iter()
                .filter(|s| match s {
                    SpaceOccupant::Resource(_) => true,
                    _ => false,
                })
                .count()
        }
        as i32
        * -1;

        println!("empty spaces: {}", score);
        score
    }

    // -------------------------------------------------------------------------
    pub fn score(&self, other: &Town) -> i32 {
        let fed_buildings = self.get_fed_buildings();
        let score = self.score_chapel_variants(&fed_buildings, true)
            + self.score_cottage_variants(&fed_buildings, true)
            + self.score_factory_variants()
            + self.score_tavern_variants(other)
            + self.score_theater_variants()
            + self.score_well_variants()
            + self.score_monuments(&fed_buildings, true)
            + self.score_empty_spaces();
        println!("total score: {score}");

        score
    }

    // -------------------------------------------------------------------------
    pub fn print(&self) {
        for (idx, s) in self.spaces.iter().enumerate() {
            let sym = String::from("@");
            let colored_sym = match s {
                SpaceOccupant::Building(building_type, _) => match building_type {
                    Building::Orange => sym.truecolor(230, 131, 2),
                    Building::Blue => sym.blue(),
                    Building::Black => sym.black(),
                    Building::Red => sym.red(),
                    Building::Green => sym.green(),
                    Building::Yellow => sym.yellow(),
                    Building::Gray => sym.truecolor(75, 75, 75),
                    Building::Magenta => sym.magenta(),
                },
                SpaceOccupant::Resource(resource) => match resource {
                    _ => String::from(".").black(),
                }
                SpaceOccupant::None => String::from("_").black(),
            };
            print!("{colored_sym}");
            if self.col(idx) == self.cols - 1 {
                println!("");
            } else {
                print!(" ");
            }
        }
    }
}

// =============================================================================
fn main() {
    // Define building variants.
    let chapel_variant = OrangeBuilding::Temple;
    let cottage_variant = BlueBuilding::Cottage;
    let factory_variant = BlackBuilding::Warehouse;
    let farm_variant = RedBuilding::Granary;
    let tavern_variant = GreenBuilding::Inn;
    let theater_variant = YellowBuilding::Bakery;
    let well_variant = GrayBuilding::Well;
    let monument = MagentaBuilding::TheSkyBaths;

    // Create template buildings.
    let tavernlike_building = SpaceOccupant::Building(Building::Green, Vec::new());
    let theaterlike_building = SpaceOccupant::Building(Building::Yellow, Vec::new());
    let welllike_building = SpaceOccupant::Building(Building::Gray, Vec::new());

    let mut town = Town::new(
        4, 4,
        chapel_variant, cottage_variant, factory_variant, farm_variant,
        tavern_variant, theater_variant, well_variant, monument);
    let mut town2 = Town::new(
        4, 4,
        chapel_variant, cottage_variant, factory_variant, farm_variant,
        tavern_variant, theater_variant, well_variant, monument);

    town.set_occupant(0, SpaceOccupant::Building(Building::Blue, Vec::new()));
    town.set_occupant(1, SpaceOccupant::Building(Building::Yellow, Vec::new()));
    town.set_occupant(2, SpaceOccupant::Building(Building::Blue, Vec::new()));
    town.set_occupant(3, SpaceOccupant::Building(Building::Red, Vec::new()));

    town.set_occupant(4, SpaceOccupant::Building(Building::Blue, Vec::new()));
    town.set_occupant(5, SpaceOccupant::Building(Building::Blue, Vec::new()));
    town.set_occupant(6, SpaceOccupant::Building(Building::Blue, Vec::new()));
    town.set_occupant(7, SpaceOccupant::Building(Building::Blue, Vec::new()));

    town.set_occupant(8, SpaceOccupant::Building(Building::Gray, Vec::new()));
    town.set_occupant(9, SpaceOccupant::Building(Building::Red, Vec::new()));
    town.set_occupant(10, SpaceOccupant::Building(Building::Green, Vec::new()));
    town.set_occupant(11, SpaceOccupant::Resource(Resource::Brick));


    town.set_occupant(12, SpaceOccupant::Building(Building::Blue, Vec::new()));
    town.set_occupant(13, SpaceOccupant::Building(Building::Magenta, Vec::new()));
    town.set_occupant(14, SpaceOccupant::Building(Building::Magenta, Vec::new()));






    // match &mut town.spaces[11] {
    //     SpaceOccupant::Building(BuildingType::FactoryLike, v) => {
    //         println!("pushing");
    //         v.push(Resource::Brick);
    //         v.push(Resource::Glass)
    //     },
    //     _ => println!("doing nothing"),
    // }


    let _ = town.score(&town2);
    town.print();

}
