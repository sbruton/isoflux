mod reexport;

pub mod domain;
pub mod event;
pub mod generation;

pub mod prelude {
    pub use crate::reexport::*;

    pub use crate::domain::FluxDomain;
    pub use crate::event::FluxEvent;
    pub use crate::generation::FluxGeneration;
}
