use mint::Vector3;
use std::ops::{Add, Sub, Mul, Div};

/// Trait to extend the `Vector3` struct from the `mint` crate.
pub trait Vector3Ext<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn magnitude(&self) -> T;
    fn normalize(&self) -> Vector3<T>;
    fn dot(&self, other: &Vector3<T>) -> T;
    fn cross(&self, other: &Vector3<T>) -> Vector3<T>;
}

// Extend Vector3 with T = f32
impl Vector3Ext<f32> for Vector3<f32> {
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Vector3<f32> {
        let mag = self.magnitude();
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn dot(&self, other: &Vector3<f32>) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vector3<f32>) -> Vector3<f32> {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// Extend Vector3 with T = u32
impl Vector3Ext<u32> for Vector3<u32> {
    fn magnitude(&self) -> u32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt() as u32
    }

    fn normalize(&self) -> Vector3<u32> {
        let mag = self.magnitude();
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    fn dot(&self, other: &Vector3<u32>) -> u32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vector3<u32>) -> Vector3<u32> {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
