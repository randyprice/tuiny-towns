use crate::building::{
    BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding,
    MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding
};

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
