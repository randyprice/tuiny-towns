use crate::building::{
    BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding, MagentaBuilding,
    OrangeBuilding, RedBuilding, YellowBuilding};
use crate::board::Board;
use crate::building::{BuildingType, Resource};
use crate::building_config::BuildingConfig;
use crate::score::score;

pub mod board;
pub mod building;
pub mod building_config;
pub mod display;
pub mod feed;
pub mod score;
pub mod space;
pub mod utils;

fn main() {
    let building_config = BuildingConfig::new(
        BlackBuilding::Factory,
        BlueBuilding::Cottage,
        GrayBuilding::Well,
        GreenBuilding::Tavern,
        MagentaBuilding::SilvaForum,
        OrangeBuilding::Chapel,
        RedBuilding::Farm,
        YellowBuilding::Theater,
    );

    let mut board = Board::new(4, 4);
    for idx in 0..2 {
        board.place(idx, BuildingType::Red);
    }
    for idx in 2..10 {
        board.place(idx, BuildingType::Blue);
    }
    for idx in 10..16 {
        board.place(idx, BuildingType::Orange);
    }
    board.print();
    score(&board, &building_config, &Board::new(4, 4));

}