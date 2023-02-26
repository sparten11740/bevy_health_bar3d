use std::marker::PhantomData;
use bevy::prelude::{Color, Component, Resource};
use crate::constants::DEFAULT_BACKGROUND_COLOR;

/// Component to configure the Y-offset of the bar relative to the entity its attached to
#[derive(Component)]
pub struct HealthBarOffset(pub f32);

impl HealthBarOffset {
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Component to configure the width of the bar
#[derive(Component)]
pub struct HealthBarWidth(pub f32);

impl HealthBarWidth {
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Component to configure the width of the bar
#[derive(Component)]
pub struct HealthBarHeight(pub f32);

impl HealthBarHeight {
    pub fn get(&self) -> f32 {
        self.0
    }
}

/// Trait implemented by the component to be tracked by the health bar
pub trait Percentage {
    /// Value between 0 and 1
    fn value(&self) -> f32;
}

#[derive(Resource)]
pub struct ColorScheme<T: Percentage + Component> {
    pub background_color: Color,
    phantom_data: PhantomData<T>
}

impl<T: Percentage + Component> Default for ColorScheme<T> {
    fn default() -> Self {
       Self {
           background_color: DEFAULT_BACKGROUND_COLOR,
           phantom_data: PhantomData,
       }
    }
}