
pub struct ResourcePile {
    pub HydrogenSulfide: u32,
    pub CarbonDioxide: u32,
    pub Oxygen: u32,
    pub Water: u32,
    pub Sugar: u32,
    pub Sulfur: u32,
    pub Ethanol: u32
}

impl ResourcePile {
    pub fn new() -> ResourcePile {
        ResourcePile {
            HydrogenSulfide: 0,
            CarbonDioxide: 0,
            Oxygen: 0,
            Water: 0,
            Sugar: 0,
            Sulfur: 0,
            Ethanol: 0,
        }
    }
}
