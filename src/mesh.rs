use std::collections::HashMap;
use bevy::prelude::{Handle, Mesh, Resource};
use ordered_float::OrderedFloat;

#[derive(Resource, Default)]
pub(crate) struct MeshHandles(pub HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), Handle<Mesh>>);

impl MeshHandles {
    pub fn get(&self, width: f32, height: f32) -> Option<&Handle<Mesh>> {
        self.0.get(&(OrderedFloat(width), OrderedFloat(height)))
    }

    pub fn insert(&mut self, width: f32, height: f32, handle: Handle<Mesh>) -> Handle<Mesh> {
        self.0.insert((OrderedFloat(width), OrderedFloat(height)), handle.clone());
        handle
    }
}