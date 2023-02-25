pub mod plugin;
pub mod configuration;
mod material;

pub mod prelude {
    pub use crate::configuration::{HealthBarOffset};
    pub use crate::plugin::{Percentage};
}


