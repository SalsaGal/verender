use glam::{Vec3, vec3};
use rhachis::IdMap;

pub type ModelID = usize;
pub type MaterialID = usize;

pub struct Model {
    pub materials: IdMap<MaterialID>,
    pub vertices: Vec<Vec3>,
}

pub struct Material {}

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec3,
}

impl From<&easy_gltf::model::Vertex> for Vertex {
    fn from(vertex: &easy_gltf::model::Vertex) -> Self {
        Self {
            pos: vec3(vertex.position.x, vertex.position.y, vertex.position.z),
        }
    }
}
