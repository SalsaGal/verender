use rhachis::IdMap;

pub type ModelID = usize;
pub type MaterialID = usize;

pub struct Model {
    pub materials: IdMap<MaterialID>,
}

pub struct Material {}
