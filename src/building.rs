use strum_macros::EnumIter;

use crate::space::Space;

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

// -----------------------------------------------------------------------------
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

// pub trait Build {
//     fn eq(&self, other: Self) -> bool;
// }

// impl Build for BlackBuilding {
//     fn eq(&self, other: Self) -> bool {
//         *self == other
//     }
// }

// // -----------------------------------------------------------------------------
// pub trait Build {
//     fn to_building(&self) -> Building;
// }

// impl Build for BlackBuilding {
//     fn to_building(&self) -> Building {
//         Building::Black(*self)
//     }
// }

// impl Build for BlueBuilding {
//     fn to_building(&self) -> Building {
//         Building::Blue(*self)
//     }
// }

// impl Build for GrayBuilding {
//     fn to_building(&self) -> Building {
//         Building::Gray(*self)
//     }
// }

// impl Build for GreenBuilding {
//     fn to_building(&self) -> Building {
//         Building::Green(*self)
//     }
// }

// impl Build for MagentaBuilding {
//     fn to_building(&self) -> Building {
//         Building::Magenta(*self)
//     }
// }

// impl Build for OrangeBuilding {
//     fn to_building(&self) -> Building {
//         Building::Orange(*self)
//     }
// }

// impl Build for RedBuilding {
//     fn to_building(&self) -> Building {
//         Building::Red(*self)
//     }
// }

// impl Build for YellowBuilding {
//     fn to_building(&self) -> Building {
//         Building::Yellow(*self)
//     }
// }