/// Contains [crate::game::context::GameContext] struct
pub mod context;

/// Contains [game::Game] struct
pub mod game;

#[cfg(feature = "store")]
/// Contains common types for storing game data
pub mod store;
