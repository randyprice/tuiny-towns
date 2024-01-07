
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

    pub fn black(&self) -> BlackBuilding { self.black }
    pub fn blue(&self) -> BlueBuilding { self.blue }
    pub fn gray(&self) -> GrayBuilding { self.gray }
    pub fn green(&self) -> GreenBuilding { self.green }
    pub fn magenta(&self) -> MagentaBuilding { self.magenta }
    pub fn orange(&self) -> OrangeBuilding { self.orange }
    pub fn red(&self) -> RedBuilding { self.red }
    pub fn yellow(&self) -> YellowBuilding { self.yellow }
}
