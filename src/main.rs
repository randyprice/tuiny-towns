use crate::board::{
    BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding, MagentaBuilding,
    OrangeBuilding, RedBuilding, YellowBuilding};
use crate::board::{Board, BuildingConfig, Resource};
use crate::score::score;

pub mod board;
pub mod feed;
pub mod score;

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
    board.place(2, Resource::Brick);
    board.print();
    score(&board, &building_config);


}