use bevy::asset::uuid_handle;
use bevy::prelude::*;

pub const DEFAULT_BACKGROUND_COLOR: Color = Color::srgba(0., 0., 0., 0.75);
pub const DEFAULT_BORDER_COLOR: Color = Color::srgba(0.02, 0.02, 0.02, 0.95);
pub const DEFAULT_HIGH_COLOR: Color = Color::srgba(0., 1., 0., 0.95);
pub const DEFAULT_MODERATE_COLOR: Color = Color::srgba(1., 1., 0., 0.95);
pub const DEFAULT_LOW_COLOR: Color = Color::srgba(1., 0., 0., 0.95);

pub const DEFAULT_WIDTH: f32 = 1.2;
pub const DEFAULT_RELATIVE_HEIGHT: f32 = 0.1666;

#[cfg(feature = "3d")]
pub(crate) const BAR_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("c41a3557-a08d-4e56-b2aa-708e27acaeaa");

#[cfg(feature = "2d")]
pub(crate) const BAR_SHADER_2D_HANDLE: Handle<Shader> =
    uuid_handle!("d52b4668-b19e-5f67-c3bb-819f38bcbfbb");
