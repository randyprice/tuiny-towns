#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Resource {
    Brick,
    Glass,
    Stone,
    Wheat,
    Wood,
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum BuildingColor {
    Green,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BuildingId {
    Almshouse
}

impl BuildingId {
    pub fn color(&self) -> BuildingColor{
        match self {
            Self::Almshouse => BuildingColor::Green,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Building {
    id: BuildingId,
    color: BuildingColor,
    resources: Vec<Resource>,
}

impl Building {
    pub fn new(id: BuildingId) -> Self {
        Self {
            id,
            color: id.color(),
            resources: Vec::new(),
        }
    }
    pub fn id(&self) -> BuildingId {
        self.id
    }
    pub fn color(&self) -> BuildingColor {
        self.color
    }
}

// Different kinds of buildings.

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

pub struct BuildingConfig2 {
    green: BuildingId
}

impl BuildingConfig2 {
    pub fn new(green: BuildingId) -> Self {
        assert!(green.color() == BuildingColor::Green);
        Self {
            green,
        }
    }
}

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
    pub fn new(
        black: BlackBuilding,
        blue: BlueBuilding,
        gray: GrayBuilding,
        green: GreenBuilding,
        magenta: MagentaBuilding,
        orange: OrangeBuilding,
        red: RedBuilding,
        yellow: YellowBuilding,
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

    pub fn black(&self) -> BlackBuilding {
        self.black
    }
    pub fn blue(&self) -> BlueBuilding {
        self.blue
    }
    pub fn gray(&self) -> GrayBuilding {
        self.gray
    }
    pub fn green(&self) -> GreenBuilding {
        self.green
    }
    pub fn magenta(&self) -> MagentaBuilding {
        self.magenta
    }
    pub fn orange(&self) -> OrangeBuilding {
        self.orange
    }
    pub fn red(&self) -> RedBuilding {
        self.red
    }
    pub fn yellow(&self) -> YellowBuilding {
        self.yellow
    }
}
