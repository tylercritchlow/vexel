use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T> {
    /// Creates a new `Vector4` with the given `x`, `y`, `z`, and `w` components.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(v.x, 1.0);
    /// assert_eq!(v.y, 2.0);
    /// assert_eq!(v.z, 3.0);
    /// assert_eq!(v.w, 4.0);
    /// ```
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    /// Computes the length (magnitude) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v = Vector4::new(1.0, 2.0, 2.0, 2.0);
    /// assert_eq!(v.length(), 3.605551275463989);
    /// ```
    pub fn length(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into().powi(2)
            + self.y.into().powi(2)
            + self.z.into().powi(2)
            + self.w.into().powi(2))
        .sqrt()
    }

    /// Computes the dot product of this vector and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
    /// assert_eq!(v1.dot(&v2), 70.0);
    /// ```
    pub fn dot(&self, other: &Self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into() * other.x.into())
            + (self.y.into() * other.y.into())
            + (self.z.into() * other.z.into())
            + (self.w.into() * other.w.into())
    }

    /// Computes the cross product of this vector and another.
    /// Note: The cross product is only defined for 3D vectors, so the `w` component is ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
    /// let cross = v1.cross(&v2);
    /// assert_eq!(cross.x, -4.0);
    /// assert_eq!(cross.y, 8.0);
    /// assert_eq!(cross.z, -4.0);
    /// assert_eq!(cross.w, 0.0);
    /// ```
    pub fn cross(&self, other: &Self) -> Vector4<f64>
    where
        T: Into<f64> + Copy,
    {
        Vector4::new(
            self.y.into() * other.z.into() - self.z.into() * other.y.into(),
            self.z.into() * other.x.into() - self.x.into() * other.z.into(),
            self.x.into() * other.y.into() - self.y.into() * other.x.into(),
            0.0,
        )
    }

    /// Normalizes the vector, making it a unit vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v = Vector4::new(1.0, 2.0, 2.0, 2.0);
    /// let normalized = v.normalize();
    /// let magnitude = v.length();
    ///
    /// let expected = Vector4::new(
    ///     1.0 / magnitude,
    ///     2.0 / magnitude,
    ///     2.0 / magnitude,
    ///     2.0 / magnitude,
    /// );
    ///
    /// assert!((normalized.x - expected.x).abs() < 1e-6);
    /// assert!((normalized.y - expected.y).abs() < 1e-6);
    /// assert!((normalized.z - expected.z).abs() < 1e-6);
    /// assert!((normalized.w - expected.w).abs() < 1e-6);
    /// ```

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
            w: (self.w.into() / len).into(),
        }
    }

    /// Projects this vector onto another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(1.0, 0.0, 0.0, 0.0);
    /// let projection = v1.project_onto(&v2);
    /// assert_eq!(projection.x, 1.0);
    /// assert_eq!(projection.y, 0.0);
    /// assert_eq!(projection.z, 0.0);
    /// assert_eq!(projection.w, 0.0);
    /// ```
    pub fn project_onto(&self, other: &Self) -> Self
    where
        T: Into<f64> + Copy + From<f64> + Mul<Output = T> + Add<Output = T> + Div<Output = T>,
    {
        let scalar = (self.x.into() * other.x.into()
            + self.y.into() * other.y.into()
            + self.z.into() * other.z.into()
            + self.w.into() * other.w.into())
            / (other.x.into() * other.x.into()
                + other.y.into() * other.y.into()
                + other.z.into() * other.z.into()
                + other.w.into() * other.w.into());
        Self {
            x: (scalar * other.x.into()).into(),
            y: (scalar * other.y.into()).into(),
            z: (scalar * other.z.into()).into(),
            w: (scalar * other.w.into()).into(),
        }
    }

    /// Rejects this vector from another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(1.0, 0.0, 0.0, 0.0);
    /// let rejection = v1.reject_from(&v2);
    /// assert_eq!(rejection.x, 0.0);
    /// assert_eq!(rejection.y, 2.0);
    /// assert_eq!(rejection.z, 3.0);
    /// assert_eq!(rejection.w, 4.0);
    /// ```
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
            w: self.w - projection.w,
        }
    }

    /// Linearly interpolates between this vector and another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
    /// let interpolated = v1.lerp(&v2, 0.5);
    /// assert_eq!(interpolated.x, 3.0);
    /// assert_eq!(interpolated.y, 4.0);
    /// assert_eq!(interpolated.z, 5.0);
    /// assert_eq!(interpolated.w, 6.0);
    /// ```
    pub fn lerp(&self, other: &Self, t: f64) -> Self
    where
        T: Into<f64> + Copy + From<f64> + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T>,
    {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
            w: self.w + (other.w - self.w) * t,
        }
    }

    /// Computes the angle between this vector and another vector in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 0.0, 0.0, 0.0);
    /// let v2 = Vector4::new(0.0, 1.0, 0.0, 0.0);
    /// let angle = v1.angle_between(&v2);
    /// assert_eq!(angle, std::f64::consts::FRAC_PI_2);
    /// ```
    pub fn angle_between(&self, other: &Self) -> f64
    where
        T: Into<f64> + Copy,
    {
        let dot_product = self.dot(other);
        let magnitude_product = self.length() * other.length();
        (dot_product / magnitude_product).acos()
    }

    /// Swizzles the components of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let swizzled = v.swizzle(1, 2, 3, 0);
    /// assert_eq!(swizzled.x, v.y);
    /// assert_eq!(swizzled.y, v.z);
    /// assert_eq!(swizzled.z, v.w);
    /// assert_eq!(swizzled.w, v.x);
    /// ```
    pub fn swizzle(&self, x: usize, y: usize, z: usize, w: usize) -> Self
    where
        T: Copy,
    {
        let components = [self.x, self.y, self.z, self.w];
        Self {
            x: components[x],
            y: components[y],
            z: components[z],
            w: components[w],
        }
    }
}

impl<T> Add for Vector4<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    /// Adds two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
    /// let result = v1 + v2;
    /// assert_eq!(result.x, 6.0);
    /// assert_eq!(result.y, 8.0);
    /// assert_eq!(result.z, 10.0);
    /// assert_eq!(result.w, 12.0);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T> Sub for Vector4<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    /// Subtracts one vector from another component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(5.0, 6.0, 7.0, 8.0);
    /// let v2 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let result = v1 - v2;
    /// assert_eq!(result.x, 4.0);
    /// assert_eq!(result.y, 4.0);
    /// assert_eq!(result.z, 4.0);
    /// assert_eq!(result.w, 4.0);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T> Mul for Vector4<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    /// Multiplies two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(1.0, 2.0, 3.0, 4.0);
    /// let v2 = Vector4::new(2.0, 3.0, 4.0, 5.0);
    /// let result = v1 * v2;
    /// assert_eq!(result.x, 2.0);
    /// assert_eq!(result.y, 6.0);
    /// assert_eq!(result.z, 12.0);
    /// assert_eq!(result.w, 20.0);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}

impl<T> Div for Vector4<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    /// Divides one vector by another component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector4::Vector4;
    ///
    /// let v1 = Vector4::new(2.0, 6.0, 12.0, 20.0);
    /// let v2 = Vector4::new(2.0, 3.0, 4.0, 5.0);
    /// let result = v1 / v2;
    /// assert_eq!(result.x, 1.0);
    /// assert_eq!(result.y, 2.0);
    /// assert_eq!(result.z, 3.0);
    /// assert_eq!(result.w, 4.0);
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
