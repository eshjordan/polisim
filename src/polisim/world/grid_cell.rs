use crate::polisim::environment::resources::{Resource, ResourcePile};
use crate::polisim::entities::entity::Entity;


pub struct GridCell {
    pub resource_pile: ResourcePile,

    entity: Entity,

    temperature: f32,
    reflectance: f32,
}

impl GridCell {

    pub fn new() -> GridCell {
        GridCell {
            resource_pile: ResourcePile::new(),

            entity: Entity::new(),

            temperature: 298.15, // 25 degrees Celsius
            reflectance: 0.3,
        }
    }

    /// Shine light on the grid cell, returning the amount of light that was reflected.
    pub fn shine_light(&mut self, light_intensity: f32) -> f32 {
        assert!(self.reflectance >= 0.0 && self.reflectance <= 1.0);
        const heat_factor: f32 = 0.1;

        // The entity absorbs some light, and reflects the rest.
        let light_intensity = self.entity.shine_light(light_intensity);
        let reflected_light = light_intensity * self.reflectance;
        self.temperature += (light_intensity - reflected_light)*heat_factor;
        reflected_light
    }
}
