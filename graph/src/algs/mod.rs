//! Collection of algorithms.

pub mod components;
pub mod cycles;
pub mod kruskal;
pub mod prim;
pub mod paths;
pub mod trees;

pub use self::components::Components;
pub use self::cycles::Cycles;
pub use self::kruskal::Kruskal;
pub use self::prim::Prim;
pub use self::paths::Paths;
pub use self::trees::Trees;
