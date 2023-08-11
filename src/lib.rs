/// Contains structs for game control and windowing.
pub mod game;
/// Contains math functions to manipulate `mint` types, like [Vector2] or [Vector3].
pub mod math;
/// Contains structs for the [game] module to use. Mostly rendering and geometry stuff.
pub mod render;
/// Contains the [scene::Scene] trait.
pub mod scene;
/// Contains some functions for creating structs simpler and more readable.
pub mod simplified;
/// Contains common types of qilin.
pub mod types;
/// Contains plugin development features.
pub mod plugin;

/// Contains utils for converting images from the `image` crate to a qilin [types::Image].\
/// Requires `image` feature.
#[cfg(feature = "image")]
pub mod image;

/// Contains audio playing functionality.
/// Requires `audio` feature.
#[cfg(feature = "audio")]
pub mod audio;

/// Contains text rendering functions.
/// Requires `text` feature.
#[cfg(feature = "text")]
pub mod text;

pub use minifb::Key;
pub use minifb::KeyRepeat;
pub use minifb::MouseButton;
pub use minifb::Scale;
pub use minifb::ScaleMode;
pub use minifb::WindowOptions;
pub use mint::EulerAngles;
pub use mint::Quaternion;
pub use mint::Vector2;
pub use mint::Vector3;
pub use mint::Vector4;

#[cfg(feature = "minifb")]
pub use minifb::*;
