use mint::Quaternion;
use std::ops::{Add, Div, Mul, Sub};

/// Trait to extend the `Vector2` struct from the `mint` crate.
pub trait QuatExt<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn magnitude(&self) -> T;
    fn normalize(&self) -> Quaternion<T>;
    fn conjugate(&self) -> Quaternion<T>;
    fn dot(&self, other: &Quaternion<T>) -> T;
}

// Extend Vector2 with T = f32
impl QuatExt<f32> for Quaternion<f32> {
    fn magnitude(&self) -> f32 {
        (self.s * self.s + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z).sqrt()
    }

    fn normalize(&self) -> Quaternion<f32> {
        let mag = self.magnitude();
        Quaternion {
            s: self.s / mag,
            v: mint::Vector3 {
                x: self.v.x / mag,
                y: self.v.y / mag,
                z: self.v.z / mag,
            },
        }
    }

    fn conjugate(&self) -> Quaternion<f32> {
        Quaternion {
            s: self.s,
            v: mint::Vector3 {
                x: -self.v.x,
                y: -self.v.y,
                z: -self.v.z,
            },
        }
    }

    fn dot(&self, other: &Quaternion<f32>) -> f32 {
        self.s * other.s + self.v.x * other.v.x + self.v.y * other.v.y + self.v.z * other.v.z
    }
}

// Extend Vector2 with T = u32
impl QuatExt<u32> for Quaternion<u32> {
    fn magnitude(&self) -> u32 {
        ((self.s * self.s + self.v.x * self.v.x + self.v.y * self.v.y + self.v.z * self.v.z) as f64)
            .sqrt() as u32
    }

    fn normalize(&self) -> Quaternion<u32> {
        let mag = self.magnitude();
        Quaternion {
            s: self.s / mag,
            v: mint::Vector3 {
                x: self.v.x / mag,
                y: self.v.y / mag,
                z: self.v.z / mag,
            },
        }
    }

    fn conjugate(&self) -> Quaternion<u32> {
        Quaternion {
            s: self.s,
            v: mint::Vector3 {
                x: self.v.x,
                y: self.v.y,
                z: self.v.z,
            },
        }
    }

    fn dot(&self, other: &Quaternion<u32>) -> u32 {
        self.s * other.s + self.v.x * other.v.x + self.v.y * other.v.y + self.v.z * other.v.z
    }
}
