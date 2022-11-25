use glam::Vec3;
use rhachis::IdMap;

pub type ModelID = usize;
pub type MaterialID = usize;

pub struct Model {
    pub materials: IdMap<MaterialID>,
    pub vertices: Vec<Vec3>,
}

pub struct Material {}
