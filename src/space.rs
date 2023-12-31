use crate::building::{BuildingType, Resource};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Space {
    Building(BuildingType),
    BuildingWithOptResource(BuildingType, Option<Resource>),
    BuildingWithResource(BuildingType, Resource),
    BuildingWithResources(BuildingType, Vec<Resource>, usize),
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
            | Space::BuildingWithResources(building_type, _, _) =>
                Some(*building_type),
            _ => None
        };

        building_type_opt
    }

    // -------------------------------------------------------------------------
    pub fn building_type_eq(&self, building_type: BuildingType) -> bool {
        let eq =
            if let Some(my_building_type) = self.building_type() {
                my_building_type == building_type
            } else {
                false
            };

        eq
    }
}