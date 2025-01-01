use bevy::prelude::{Component, Handle, Mesh, Resource};
use ordered_float::OrderedFloat;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub(crate) struct MeshHandles(pub HashMap<(OrderedFloat<f32>, OrderedFloat<f32>), Handle<Mesh>>);

#[derive(Component, Clone, PartialEq)]
pub(crate) struct MeshHandle(pub Handle<Mesh>);

impl MeshHandles {
    pub fn get(&self, width: f32, height: f32) -> Option<MeshHandle> {
        self.0
            .get(&(OrderedFloat(width), OrderedFloat(height)))
            .cloned()
            .map(MeshHandle)
    }

    pub fn insert(&mut self, width: f32, height: f32, handle: Handle<Mesh>) -> MeshHandle {
        self.0
            .insert((OrderedFloat(width), OrderedFloat(height)), handle.clone());

        MeshHandle(handle)
    }
}
