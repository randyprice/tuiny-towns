use colored::{ColoredString, Colorize};

use crate::board::Board;
use crate::building::{BuildingType, Resource};
use crate::space::Space;

// =============================================================================
impl Space {
    pub fn as_str(&self) -> ColoredString {
        let colored_string = match &self {
            Space::Building(building_type)
            | Space::BuildingWithOptResource(building_type, _)
            | Space::BuildingWithResource(building_type, _)
            | Space::BuildingWithResources(building_type, _, _) => {
                let symbol = String::from("@");
                match building_type {
                    BuildingType::Orange => symbol.truecolor(230, 131, 2),
                    BuildingType::Blue => symbol.blue(),
                    BuildingType::Black => symbol.black(),
                    BuildingType::Red => symbol.red(),
                    BuildingType::Green => symbol.green(),
                    BuildingType::Yellow => symbol.yellow(),
                    BuildingType::Gray => symbol.truecolor(75, 75, 75),
                    BuildingType::Magenta => symbol.magenta(),
                }
            }
            Space::Resource(resource) => {
                let symbol = String::from(".");
                match resource {
                    Resource::Brick => symbol.red(),
                    Resource::Glass => symbol.blue(),
                    Resource::Stone => symbol.truecolor(60, 60, 60),
                    Resource::Wheat => symbol.yellow(),
                    Resource::Wood => symbol.truecolor(60, 50, 5),
                }
            }
            Space::Empty => ColoredString::from(" "),
        };

        colored_string
    }
}

// =============================================================================
impl Board {
    pub fn print(&self) {
        let horizontal_line = "-".repeat(self.cols() * 4 + 1);
        println!("{horizontal_line}");
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let space = &self.spaces()[self.idx(row, col)];
                let symbol = space.as_str();
                print!("| {symbol} ");
            }
            println!("|");
            println!("{horizontal_line}");
        }
    }
}
