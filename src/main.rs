use crate::building_config::{
    BlackBuilding, BlueBuilding, GrayBuilding, GreenBuilding, MagentaBuilding,
    OrangeBuilding, RedBuilding, YellowBuilding};
use crate::board::Board;
use crate::building_config::BuildingConfig;
use crate::score::score;
use crate::board::space::{BuildingType, Resource};

pub mod board;
pub mod building;
pub mod building_config;
pub mod display;
pub mod score;
pub mod utils;

fn main() {
    let building_config = BuildingConfig::new(
        BlackBuilding::Warehouse,
        BlueBuilding::Cottage,
        GrayBuilding::Well,
        GreenBuilding::Almshouse,
        MagentaBuilding::SilvaForum,
        OrangeBuilding::Cloister,
        RedBuilding::Granary,
        YellowBuilding::Tailor,
    );

    let mut board = Board::new(4, 4);

    board.place(0, BuildingType::Orange);
    board.place(1, BuildingType::Blue);
    board.place(2, BuildingType::Orange);
    board.place(3, BuildingType::Orange);

    board.place(4, BuildingType::Blue);
    board.place(5, BuildingType::Red);
    board.place(6, BuildingType::Green);
    board.place(7, BuildingType::Blue);

    board.place(8, BuildingType::Orange);
    board.place(9, BuildingType::Yellow);
    board.place(10, BuildingType::Yellow);
    board.place(11, BuildingType::Gray);

    board.place(12, BuildingType::Orange);
    board.place(13, (BuildingType::Black, vec![Resource::Glass, Resource::Brick], 3));
    board.place(14, BuildingType::Magenta);
    board.place(15, Resource::Wood);

    let score_card = score(&board, &building_config, None);
    board.print_scores(&score_card, &building_config);

}