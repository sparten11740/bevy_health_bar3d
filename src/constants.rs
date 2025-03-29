use bevy::{asset::weak_handle, prelude::*};

pub const DEFAULT_BACKGROUND_COLOR: Color = Color::srgba(0., 0., 0., 0.75);
pub const DEFAULT_BORDER_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.95);
pub const DEFAULT_HIGH_COLOR: Color = Color::srgba(0., 1., 0., 0.95);
pub const DEFAULT_MODERATE_COLOR: Color = Color::srgba(1., 1., 0., 0.95);
pub const DEFAULT_LOW_COLOR: Color = Color::srgba(1., 0., 0., 0.95);

pub const DEFAULT_WIDTH: f32 = 1.2;
pub const DEFAULT_RELATIVE_HEIGHT: f32 = 0.1666;

pub(crate) const BAR_SHADER_HANDLE: Handle<Shader> = weak_handle!("3eb36ca5-6c32-4eb2-b900-b91bcacbb7a6");
