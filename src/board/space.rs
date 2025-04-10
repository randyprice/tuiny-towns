use strum_macros::EnumIter;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Resource {
    Brick,
    Glass,
    Stone,
    Wheat,
    Wood,
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Space {
    Building(BuildingType),
    BuildingWithOptResource(BuildingType, Option<Resource>),
    BuildingWithResource(BuildingType, Resource),
    BuildingWithResources(BuildingType, Vec<Resource>),
    Resource(Resource),
    Empty,
}

impl Space {
    // -------------------------------------------------------------------------
    pub fn building_type(&self) -> Option<BuildingType> {
        let building_type_opt = match self {
            Space::Building(building_type)
            | Space::BuildingWithOptResource(building_type, _)
            | Space::BuildingWithResource(building_type, _)
            | Space::BuildingWithResources(building_type, _) => {
                Some(*building_type)
            }
            _ => None,
        };

        building_type_opt
    }

    // -------------------------------------------------------------------------
    pub fn building_type_eq(&self, building_type: BuildingType) -> bool {
        let eq = if let Some(my_building_type) = self.building_type() {
            my_building_type == building_type
        } else {
            false
        };

        eq
    }

    // -------------------------------------------------------------------------
    pub fn is_unused(&self) -> bool {
        match self {
            Space::Resource(_) | Space::Empty => true,
            _ => false,
        }
    }

    // -------------------------------------------------------------------------
    pub fn resources(&self) -> Option<&Vec<Resource>> {
        match self {
            Space::BuildingWithResources(_, resources) => Some(resources),
            _ => None,
        }
    }
}

// =============================================================================
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
        Space::BuildingWithResources(self.0, self.1)
    }
}
