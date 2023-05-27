use crate::polisim::environment::{gas::Gas, liquid::Liquid, solid::Solid};
use crate::polisim::entities::entity::Entity;


pub struct GridCell {
    ground: Solid,
    water: Liquid,
    air: Gas,

    ground_entity: Entity,
    water_entity: Entity,
    air_entity: Entity
}
