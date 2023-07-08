use mint::{EulerAngles, Vector2, Vector3, Vector4};

pub fn vec2<T>(x: T, y: T) -> Vector2<T> {
    Vector2::from([x, y])
}

pub fn vec3<T>(x: T, y: T, z: T) -> Vector3<T> {
    Vector3::from([x, y, z])
}

pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vector4<T> {
    Vector4::from([x, y, z, w])
}
