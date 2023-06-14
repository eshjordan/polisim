use std::cmp::min;
use std::marker::PhantomData;

use crate::polisim::world::grid_cell::GridCell;
use crate::polisim::entities::metabolism::{AnabolicType, CatabolicType};

use super::metabolism::{Chemoorganoheterotroph, Anaerobe, Photolithoautotroph, Chemoorganoautotroph, Aerobe, Fermenter};


pub struct Cell<T: AnabolicType, U: CatabolicType> {
    pub(crate) water: u32,
    pub(crate) sugar: u32,
    pub(crate) sulfur: u32,

    pub(crate) max_water: u32,
    pub(crate) max_sugar: u32,
    pub(crate) max_sulfur: u32,

    pub dead: bool,

    anabolic_type: PhantomData<T>,
    catabolic_type: PhantomData<U>,
}

impl<T: AnabolicType, U: CatabolicType> Cell<T, U> {
    pub fn new() -> Self {
        Self {
            water: 60,
            sugar: 10,
            sulfur: 36,

            max_water: 100,
            max_sugar: 100,
            max_sulfur: 100,
            dead: false,

            anabolic_type: PhantomData,
            catabolic_type: PhantomData,
        }
    }

    pub fn drink(&mut self, grid_cell: &mut GridCell) {
        let mut grid_water = &grid_cell.resource_pile.Water;

        if self.dead {
            return;
        }

        let drunk_water = min(*grid_water, self.max_water - self.water);
        self.water += drunk_water;
        *grid_water -= drunk_water;
    }

    pub fn kill(&mut self) {
        self.dead = true;
    }
}


impl<T: Chemoorganoheterotroph, U: CatabolicType> Cell<T,U> {
    fn consume(&mut self, grid_cell: &mut GridCell) {
        let mut grid_sugar = &grid_cell.resource_pile.Sugar;
        let mut grid_sulfur = &grid_cell.resource_pile.Sulfur;

        if self.dead {
            return;
        }

        let eaten_sugar = min(*grid_sugar, self.max_sugar - self.sugar);
        self.sugar += eaten_sugar;
        *grid_sugar -= eaten_sugar;

        let eaten_sulfur = min(*grid_sulfur, self.max_sulfur - self.sulfur);
        self.sulfur += eaten_sulfur;
        *grid_sulfur -= eaten_sulfur;
    }
}

impl<T: Photolithoautotroph, U: CatabolicType> Cell<T,U> {
    fn photosynthesise(&mut self, grid_cell: &mut GridCell) {
        let mut grid_water = &grid_cell.resource_pile.Water;
        let mut grid_sugar = &grid_cell.resource_pile.Sugar;
        let mut grid_oxygen = &grid_cell.resource_pile.Oxygen;
    }
}

impl<T: Chemoorganoautotroph, U: CatabolicType> Cell<T,U> {
    fn chemosynthesise(&mut self, grid_cell: &mut GridCell) {
        let mut grid_water = &grid_cell.resource_pile.Water;
        let mut grid_sugar = &grid_cell.resource_pile.Sugar;
        let mut grid_oxygen = &grid_cell.resource_pile.Oxygen;
    }
}

impl<T: AnabolicType, U: Aerobe> Cell<T, U> {
    fn aerobically_respire(&mut self, grid_cell: &mut GridCell) {
        let mut grid_water = &grid_cell.resource_pile.Water;
        let mut grid_carbon_dioxide = &grid_cell.resource_pile.CarbonDioxide;
        let mut grid_oxygen = &grid_cell.resource_pile.Oxygen;
    }
}

impl<T: AnabolicType, U: Anaerobe> Cell<T, U> {
    fn anaerobically_respire(&mut self, grid_cell: &mut GridCell) {
        let mut grid_oxygen = &grid_cell.resource_pile.Oxygen;
        let mut grid_carbon_dioxide = &grid_cell.resource_pile.CarbonDioxide;
        let mut grid_hydrogen_sulfide = &grid_cell.resource_pile.HydrogenSulfide;

        if self.dead {
            return;
        } else if self.water < 12 || self.sulfur < 18 {
            self.kill();
            return;
        }

        self.sulfur -= 18;
        self.water -= 12;
        self.sugar -= 1;

        *grid_hydrogen_sulfide += 18;
        *grid_carbon_dioxide += 6;
        *grid_oxygen += 3;
    }
}

impl<T: AnabolicType, U: Fermenter> Cell<T, U> {
    fn ferment(&mut self, grid_cell: &mut GridCell) {
        let mut grid_water = &grid_cell.resource_pile.Water;
        let mut grid_carbon_dioxide = &grid_cell.resource_pile.CarbonDioxide;
        let mut grid_ethanol = &grid_cell.resource_pile.Ethanol;
    }
}
