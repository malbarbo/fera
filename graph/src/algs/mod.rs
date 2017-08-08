//! Collection of algorithms.

pub mod components;
pub mod cycles;
pub mod degrees;
pub mod kruskal;
pub mod paths;
pub mod prim;
pub mod sets;
pub mod trees;

pub use self::components::Components;
pub use self::cycles::Cycles;
pub use self::degrees::Degrees;
pub use self::kruskal::Kruskal;
pub use self::paths::Paths;
pub use self::prim::Prim;
pub use self::sets::Sets;
pub use self::trees::Trees;
