use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Add for Vector3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> Sub for Vector3<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> Mul for Vector3<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T> Div for Vector3<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into().powi(2) + self.y.into().powi(2) + self.z.into().powi(2)).sqrt()
    }
    pub fn dot(&self, other: &Self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into() * other.x.into())
            + (self.y.into() * other.y.into())
            + (self.z.into() * other.z.into())
    }
    pub fn cross(&self, other: &Self) -> Self
    where
        T: Copy + Into<f64> + From<f64>,
    {
        Self {
            x: (self.y.into() * other.z.into() - self.z.into() * other.y.into()).into(),
            y: (self.z.into() * other.x.into() - self.x.into() * other.z.into()).into(),
            z: (self.x.into() * other.y.into() - self.y.into() * other.x.into()).into(),
        }
    }

    pub fn normalize(&self) -> Self
    where
        T: Into<f64> + Copy + From<f64>,
    {
        let len = self.length();
        if len == 0.0 {
            return *self;
        }
        Self {
            x: (self.x.into() / len).into(),
            y: (self.y.into() / len).into(),
            z: (self.z.into() / len).into(),
        }
    }

    pub fn project_onto(&self, other: &Self) -> Self
    where
        T: Into<f64> + Copy + From<f64> + Mul<Output = T> + Add<Output = T> + Div<Output = T>,
    {
        let scalar = (self.x.into() * other.x.into()
            + self.y.into() * other.y.into()
            + self.z.into() * other.z.into())
            / (other.x.into() * other.x.into()
                + other.y.into() * other.y.into()
                + other.z.into() * other.z.into());
        Self {
            x: (scalar * other.x.into()).into(),
            y: (scalar * other.y.into()).into(),
            z: (scalar * other.z.into()).into(),
        }
    }

    pub fn reject_from(&self, other: &Self) -> Self
    where
        T: Into<f64>
            + Copy
            + From<f64>
            + Mul<Output = T>
            + Add<Output = T>
            + Div<Output = T>
            + Sub<Output = T>,
    {
        let projection = self.project_onto(other);
        Self {
            x: self.x - projection.x,
            y: self.y - projection.y,
            z: self.z - projection.z,
        }
    }

    pub fn lerp(&self, other: &Self, t: f64) -> Self
    where
        T: Into<f64> + Copy + From<f64> + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T>,
    {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        }
    }
    pub fn angle_between(&self, other: &Self) -> f64
    where
        T: Into<f64> + Copy,
    {
        let dot_product = self.dot(other);
        let magnitude_product = self.length() * other.length();
        (dot_product / magnitude_product).acos()
    }

    pub fn swizzle(&self, x: usize, y: usize, z: usize) -> Self
    where
        T: Copy,
    {
        let components = [self.x, self.y, self.z];
        Self {
            x: components[x],
            y: components[y],
            z: components[z],
        }
    }
}
