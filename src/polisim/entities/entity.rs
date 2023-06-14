pub struct Entity {

}

impl Entity {

    pub fn new() -> Entity {
        Entity {}
    }

    /// Shine light on the entity, returning the amount of light that was reflected.
    pub fn shine_light(&mut self, light_intensity: f32) -> f32 {
        light_intensity
    }
}
