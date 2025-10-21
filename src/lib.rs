pub mod configuration;
pub mod constants;
#[cfg(feature = "3d")]
mod material;
#[cfg(feature = "2d")]
mod material2d;
mod mesh;
pub mod plugin;

pub mod prelude {
    pub use crate::configuration::*;
    pub use crate::plugin::HealthBarPlugin;
}
