use bevy::prelude::{Handle, Mesh, Mesh3d, Resource};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub(crate) struct MeshHandles(pub HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), Handle<Mesh>>);

impl MeshHandles {
    pub fn get(&self, width: f32, height: f32) -> Option<Mesh3d> {
        self.0
            .get(&(OrderedFloat(width), OrderedFloat(height)))
            .cloned()
            .map(Mesh3d)
    }

    pub fn insert(&mut self, width: f32, height: f32, handle: Handle<Mesh>) -> Mesh3d {
        self.0
            .insert((OrderedFloat(width), OrderedFloat(height)), handle.clone());

        Mesh3d(handle)
    }
}
