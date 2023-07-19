use mint::Vector4;
use std::ops::{Add, Div, Mul, Sub};

/// Trait to extend the `Vector4` struct from the `mint` crate.
pub trait Vector4Ext<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn magnitude(&self) -> T;
    fn normalize(&self) -> Vector4<T>;
    fn dot(&self, other: &Vector4<T>) -> T;
}

// Extend Vector4 with T = f32
impl Vector4Ext<f32> for Vector4<f32> {
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    fn normalize(&self) -> Vector4<f32> {
        let mag = self.magnitude();
        Vector4 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    fn dot(&self, other: &Vector4<f32>) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

// Extend Vector4 with T = f32
impl Vector4Ext<u32> for Vector4<u32> {
    fn magnitude(&self) -> u32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w) as f64).sqrt()
            as u32
    }

    fn normalize(&self) -> Vector4<u32> {
        let mag = self.magnitude();
        Vector4 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag,
        }
    }

    fn dot(&self, other: &Vector4<u32>) -> u32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}
