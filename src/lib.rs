pub mod plugin;
pub mod configuration;
mod material;
mod mesh;
mod constants;

pub mod prelude {
    pub use crate::configuration::*;
    pub use crate::plugin::*;
}


