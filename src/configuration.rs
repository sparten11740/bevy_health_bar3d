use bevy::prelude::Component;

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