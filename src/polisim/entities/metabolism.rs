use crate::polisim::world::grid_cell::GridCell;


// Anabolic reactions, energy generation

pub trait AnabolicType {
    fn anabolise(&mut self, grid_cell: &mut GridCell);
}

pub trait Chemoorganoheterotroph: AnabolicType {
    fn anabolise(&mut self, grid_cell: &mut GridCell) {
        self.consume(grid_cell);
    }

    fn consume(&mut self, grid_cell: &mut GridCell);
}

pub trait Photolithoautotroph: AnabolicType {
    fn anabolise(&mut self, grid_cell: &mut GridCell) {
        self.photosynthesise(grid_cell);
    }

    fn photosynthesise(&mut self, grid_cell: &mut GridCell);
}

pub trait Chemoorganoautotroph: AnabolicType {
    fn anabolise(&mut self, grid_cell: &mut GridCell) {
        self.chemosynthesise(grid_cell);
    }

    fn chemosynthesise(&mut self, grid_cell: &mut GridCell);
}


// Catabolic reactions, energy utilisation

pub trait CatabolicType {
    fn respire(&mut self, grid_cell: &mut GridCell);
}

pub trait Aerobe: CatabolicType {
    fn respire(&mut self, grid_cell: &mut GridCell) {
        self.aerobically_respire(grid_cell);
    }

    fn aerobically_respire(&mut self, grid_cell: &mut GridCell);
}

pub trait Anaerobe: CatabolicType {
    fn respire(&mut self, grid_cell: &mut GridCell) {
        self.anaerobically_respire(grid_cell);
    }

    fn anaerobically_respire(&mut self, grid_cell: &mut GridCell);
}

pub trait Fermenter: CatabolicType {
    fn respire(&mut self, grid_cell: &mut GridCell) {
        self.ferment(grid_cell);
    }

    fn ferment(&mut self, grid_cell: &mut GridCell);
}
