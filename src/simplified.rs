use mint::{Vector2, Vector3, Vector4};

/// Util function to create a 2D Vector.\
/// `T` is the type the vector contains.
#[inline]
pub fn vec2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::from([x, y])
}

/// Util function to create a 3D Vector.\
/// `T` is the type the vector contains.
#[inline]
pub fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::from([x, y, z])
}

/// Util function to create a 4D Vector.\
/// `T` is the type the vector contains.
#[inline]
pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> {
    Vector4::from([x, y, z, w])
}
