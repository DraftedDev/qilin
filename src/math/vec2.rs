use mint::Vector2;
use std::ops::{Add, Sub, Mul, Div};

/// Trait to extend the `Vector2` struct from the `mint` crate.
pub trait Vector2Ext<T>
    where
        T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Copy,
{
    fn magnitude(&self) -> T;
    fn normalize(&self) -> Vector2<T>;
    fn dot(&self, other: &Vector2<T>) -> T;
    fn cross(&self, other: &Vector2<T>) -> T;
    fn distance(&self, other: &Vector2<T>) -> T;
    fn angle(&self, other: &Vector2<T>) -> T;
    fn rotate(&self, angle: T) -> Vector2<T>;
}

// Extend Vector2 with T = f32
impl Vector2Ext<f32> for Vector2<f32> {
    fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(&self) -> Vector2<f32> {
        let mag = self.magnitude();
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn dot(&self, other: &Vector2<f32>) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Vector2<f32>) -> f32 {
        self.x * other.y - self.y * other.x
    }

    fn distance(&self, other: &Vector2<f32>) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn angle(&self, other: &Vector2<f32>) -> f32 {
        let dot_product = self.dot(other);
        let self_magnitude = self.magnitude();
        let other_magnitude = other.magnitude();
        (dot_product / (self_magnitude * other_magnitude)).acos()
    }

    fn rotate(&self, angle: f32) -> Vector2<f32> {
        let cos_angle = angle.cos();
        let sin_angle = angle.sin();
        Vector2 {
            x: self.x * cos_angle - self.y * sin_angle,
            y: self.x * sin_angle + self.y * cos_angle,
        }
    }
}

// Extend Vector2 with T = u32
impl Vector2Ext<u32> for Vector2<u32> {
    fn magnitude(&self) -> u32 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt() as u32
    }

    fn normalize(&self) -> Vector2<u32> {
        let mag = self.magnitude();
        Vector2 {
            x: self.x / mag,
            y: self.y / mag,
        }
    }

    fn dot(&self, other: &Vector2<u32>) -> u32 {
        self.x * other.x + self.y * other.y
    }

    fn cross(&self, other: &Vector2<u32>) -> u32 {
        self.x * other.y - self.y * other.x
    }

    fn distance(&self, other: &Vector2<u32>) -> u32 {
        let dx = (self.x as i32 - other.x as i32).abs();
        let dy = (self.y as i32 - other.y as i32).abs();
        ((dx * dx + dy * dy) as f64).sqrt() as u32
    }

    fn angle(&self, other: &Vector2<u32>) -> u32 {
        let dot_product = self.dot(other) as f64;
        let self_magnitude = self.magnitude() as f64;
        let other_magnitude = other.magnitude() as f64;

        // Calculate the angle in radians
        let angle_radians = (dot_product / (self_magnitude * other_magnitude)).acos();

        // Convert the angle to degrees
        let angle_degrees = angle_radians.to_degrees() as u32;
        angle_degrees
    }

    fn rotate(&self, angle: u32) -> Vector2<u32> {
        let cos_angle = (angle as f32).cos();
        let sin_angle = (angle as f32).sin();
        Vector2 {
            x: ((self.x as f32 * cos_angle - self.y as f32 * sin_angle) as u32),
            y: ((self.x as f32 * sin_angle + self.y as f32 * cos_angle) as u32),
        }
    }
}