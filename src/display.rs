use std::collections::HashMap;

use colored::{ColoredString, Colorize};

use crate::game::space::{BuildingType, Space};
use crate::game::board::Board;
use crate::game::building::{
    BlackBuilding, BlueBuilding, BuildingConfig, GrayBuilding, GreenBuilding,
    MagentaBuilding, OrangeBuilding, RedBuilding, YellowBuilding,
};
use crate::score::ScoreCard;

// =============================================================================
impl BlackBuilding {
    fn to_string(&self) -> String {
        match self {
            BlackBuilding::Bank => String::from("Bank"),
            BlackBuilding::Factory => String::from("Factory"),
            BlackBuilding::TradingPost => String::from("Trading Post"),
            BlackBuilding::Warehouse => String::from("Warehouse"),
        }
    }
    fn to_plural_string(&self) -> String {
        match self {
            BlackBuilding::Factory => String::from("Factories"),
            _ => format!("{}s", self.to_string()),
        }
    }
}

impl BlueBuilding {
    fn to_string(&self) -> String {
        match self {
            BlueBuilding::Cottage => String::from("Cottage"),
        }
    }
    fn to_plural_string(&self) -> String {
        format!("{}s", self.to_string())
    }
}

impl GrayBuilding {
    fn to_string(&self) -> String {
        match self {
            GrayBuilding::Fountain => String::from("Fountain"),
            GrayBuilding::Millstone => String::from("Millstone"),
            GrayBuilding::Shed => String::from("Shed"),
            GrayBuilding::Well => String::from("Well"),
        }
    }

    fn to_plural_string(&self) -> String {
        format!("{}s", self.to_string())
    }
}

impl GreenBuilding {
    fn to_string(&self) -> String {
        match self {
            GreenBuilding::Almshouse => String::from("Almshouse"),
            GreenBuilding::FeastHall => String::from("Feast Hall"),
            GreenBuilding::Inn => String::from("Inn"),
            GreenBuilding::Tavern => String::from("Tavern"),
        }
    }

    fn to_plural_string(&self) -> String {
        format!("{}s", self.to_string())
    }
}

impl MagentaBuilding {
    fn to_string(&self) -> String {
        match self {
            MagentaBuilding::ArchitectsGuild => {
                String::from("Architect's Guild")
            }
            MagentaBuilding::ArchiveOfTheSecondAge => {
                String::from("Archive of the Second Age")
            }
            MagentaBuilding::BarrettCastle => String::from("Barrett Castle"),
            MagentaBuilding::CathedralOfCaterina => {
                String::from("Cathedral of Caterina")
            }
            MagentaBuilding::FortIronweed => String::from("Fort Ironweed"),
            MagentaBuilding::GrandMausoleumOfTheRodina => {
                String::from("Grand Mausoleum of the Rodina")
            }
            MagentaBuilding::GroveUniversity => {
                String::from("Grove University")
            }
            MagentaBuilding::MandrasPalace => String::from("Mandras Palace"),
            MagentaBuilding::ObeliskOfTheCrescent => {
                String::from("Obelisk of the Crescent")
            }
            MagentaBuilding::OpaleyesWatch => String::from("Opaleye's Watch"),
            MagentaBuilding::ShrineOfTheElderTree => {
                String::from("Shrine of the Elder Tree")
            }
            MagentaBuilding::SilvaForum => String::from("Silva Forum"),
            MagentaBuilding::StatueOfTheBondmaker => {
                String::from("Statue of the Bondmaker")
            }
            MagentaBuilding::TheSkyBaths => String::from("The Sky Baths"),
            MagentaBuilding::TheStarloom => String::from("The Starloom"),
        }
    }
}

impl OrangeBuilding {
    fn to_string(&self) -> String {
        match self {
            OrangeBuilding::Abbey => String::from("Abbey"),
            OrangeBuilding::Chapel => String::from("Chapel"),
            OrangeBuilding::Cloister => String::from("Cloister"),
            OrangeBuilding::Temple => String::from("Temple"),
        }
    }

    fn to_plural_string(&self) -> String {
        format!("{}s", self.to_string())
    }
}

impl RedBuilding {
    fn to_string(&self) -> String {
        match self {
            RedBuilding::Farm => String::from("Farm"),
            RedBuilding::Granary => String::from("Granary"),
            RedBuilding::Greenhouse => String::from("Greenhouse"),
            RedBuilding::Orchard => String::from("Orchard"),
        }
    }

    // fn to_plural_string(&self) -> String {
    //     match self {
    //         RedBuilding::Granary => String::from("Granaries"),
    //         _ => format!("{}s", self.to_string()),
    //     }
    // }
}

impl YellowBuilding {
    fn to_string(&self) -> String {
        match self {
            YellowBuilding::Bakery => String::from("Bakery"),
            YellowBuilding::Market => String::from("Market"),
            YellowBuilding::Tailor => String::from("Tailor"),
            YellowBuilding::Theater => String::from("Theater"),
        }
    }

    fn to_plural_string(&self) -> String {
        match self {
            YellowBuilding::Bakery => String::from("Bakeries"),
            _ => format!("{}s", self.to_string()),
        }
    }
}

// =============================================================================
impl Space {
    pub fn as_str(
        &self,
        idx: usize,
        scores_opt: Option<&HashMap<usize, i32>>,
    ) -> ColoredString {
        let colored_string = match &self {
            Space::Building(building_type)
            | Space::BuildingWithOptResource(building_type, _)
            | Space::BuildingWithResource(building_type, _)
            | Space::BuildingWithResources(building_type, _) => {
                let symbol = if let Some(scores) = scores_opt {
                    scores.get(&idx).unwrap().to_string()
                } else {
                    String::from("@")
                };
                match building_type {
                    BuildingType::Orange => symbol.truecolor(230, 131, 2),
                    BuildingType::Blue => symbol.blue(),
                    BuildingType::Black => symbol.truecolor(10, 10, 10),
                    BuildingType::Red => symbol.red(),
                    BuildingType::Green => symbol.green(),
                    BuildingType::Yellow => symbol.yellow(),
                    BuildingType::Gray => symbol.truecolor(75, 75, 75),
                    BuildingType::Magenta => symbol.magenta(),
                }
            }
            Space::Resource(resource) => {
                let symbol = if let Some(scores) = scores_opt {
                    scores.get(&idx).unwrap().to_string()
                } else {
                    String::from(".")
                };
                match resource {
                    _ => symbol.white(),
                    // Resource::Brick => symbol.red(),
                    // Resource::Glass => symbol.blue(),
                    // Resource::Stone => symbol.truecolor(60, 60, 60),
                    // Resource::Wheat => symbol.yellow(),
                    // Resource::Wood => symbol.truecolor(60, 50, 5),
                }
            }
            Space::Empty => ColoredString::from(" "),
        };

        colored_string
    }
}

fn make_messages(
    score_card: &ScoreCard,
    building_config: &BuildingConfig,
) -> Vec<String> {
    let messages = vec![
        format!("{}", building_config.red().to_string().red()),
        format!(
            "{}: {}",
            building_config.blue().to_plural_string().blue(),
            score_card.score_blue()
        ),
        format!(
            "{}: {}",
            building_config
                .orange()
                .to_plural_string()
                .truecolor(230, 131, 2),
            score_card.score_orange()
        ),
        format!(
            "{}: {}",
            building_config.green().to_plural_string().green(),
            score_card.score_green()
        ),
        format!(
            "{}: {}",
            building_config
                .gray()
                .to_plural_string()
                .truecolor(75, 75, 75),
            score_card.score_gray()
        ),
        format!(
            "{}: {}",
            building_config.yellow().to_plural_string().yellow(),
            score_card.score_yellow()
        ),
        format!(
            "{}: {}",
            building_config.black().to_plural_string().black(),
            score_card.score_black()
        ),
        format!(
            "{}: {}",
            building_config.magenta().to_string().magenta(),
            score_card.score_magenta()
        ),
        format!("Unused spaces: {}", score_card.score_unused()),
    ];

    messages
}

// =============================================================================
impl Board {
    pub fn print(&self) {
        let horizontal_line = "-".repeat(self.cols() * 4 + 1);
        println!("{horizontal_line}");
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let idx = self.idx(row, col);
                let space = &self.spaces()[idx];
                let symbol = space.as_str(idx, None);
                print!("| {symbol} ");
            }
            println!("|");
            println!("{horizontal_line}");
        }
    }

    pub fn print_scores(
        &self,
        score_card: &ScoreCard,
        building_config: &BuildingConfig,
    ) {
        // generate vector
        let messages = make_messages(score_card, building_config);
        let flattened_score_card = score_card.flatten();
        // print
        let horizontal_line = "-".repeat(self.cols() * 4 + 1);

        let mut msg_idx = 0;
        if msg_idx < messages.len() {
            println!("{horizontal_line} {}", messages[msg_idx]);
            msg_idx += 1;
        } else {
            println!("{horizontal_line}");
        }

        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let idx = self.idx(row, col);
                let space = &self.spaces()[idx];
                let symbol = space.as_str(idx, Some(&flattened_score_card));
                let score = flattened_score_card[&idx];
                if score < 0 {
                    if score < -9 {
                        print!("|{symbol}");
                    } else {
                        print!("|{symbol} ");
                    }
                } else {
                    if score < 10 {
                        print!("| {symbol} ");
                    } else {
                        print!("| {symbol}");
                    }
                }
            }
            if msg_idx < messages.len() {
                println!("| {}", messages[msg_idx]);
                msg_idx += 1;
            } else {
                println!("|");
            }
            if msg_idx < messages.len() {
                println!("{horizontal_line} {}", messages[msg_idx]);
                msg_idx += 1;
            } else {
                println!("{horizontal_line}");
            }
        }
        println!("Total score: {}", score_card.score_all());
    }
}
