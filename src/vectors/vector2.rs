use std::ops::{Add, Div, Mul, Sub};

/// A 2D vector with `x` and `y` components.
///
/// This struct is generic over the type `T`, which allows it to be used with
/// any numeric type that supports the required operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2<T> {
    /// The x-component of the vector.
    pub x: T,
    /// The y-component of the vector.
    pub y: T,
}

impl<T> Vector2<T> {
    /// Creates a new `Vector2` with the given `x` and `y` components.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v = Vector2::new(3.0, 4.0);
    /// assert_eq!(v.x, 3.0);
    /// assert_eq!(v.y, 4.0);
    /// ```
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Computes the length (magnitude) of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v = Vector2::new(3.0, 4.0);
    /// assert_eq!(v.length(), 5.0);
    /// ```
    pub fn length(&self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into().powi(2) + self.y.into().powi(2)).sqrt()
    }

    /// Computes the dot product of this vector and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(3.0, 4.0);
    /// let v2 = Vector2::new(5.0, 6.0);
    /// assert_eq!(v1.dot(&v2), 39.0);
    /// ```
    pub fn dot(&self, other: &Self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into() * other.x.into()) + (self.y.into() * other.y.into())
    }

    /// Computes the cross product of this vector and another.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(3.0, 4.0);
    /// let v2 = Vector2::new(5.0, 6.0);
    /// assert_eq!(v1.cross(&v2), -2.0);
    /// ```
    pub fn cross(&self, other: &Self) -> f64
    where
        T: Into<f64> + Copy,
    {
        (self.x.into() * other.y.into()) - (self.y.into() * other.x.into())
    }

    /// Normalizes the vector, making it a unit vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v = Vector2::new(3.0, 4.0);
    /// let normalized = v.normalize();
    /// let magnitude = v.length();
    ///
    /// let expected = Vector2::new(
    ///     3.0 / magnitude,
    ///     4.0 / magnitude,
    /// );
    ///
    /// assert!((normalized.x - expected.x).abs() < 1e-6);
    /// assert!((normalized.y - expected.y).abs() < 1e-6);
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
        }
    }

    /// Projects this vector onto another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(3.0, 4.0);
    /// let v2 = Vector2::new(1.0, 0.0);
    /// let projection = v1.project_onto(&v2);
    /// assert_eq!(projection.x, 3.0);
    /// assert_eq!(projection.y, 0.0);
    /// ```
    pub fn project_onto(&self, other: &Self) -> Self
    where
        T: Into<f64> + Copy + From<f64>,
    {
        let scalar = self.dot(other) / other.dot(other);
        Self {
            x: (scalar * other.x.into()).into(),
            y: (scalar * other.y.into()).into(),
        }
    }

    /// Rejects this vector from another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(3.0, 4.0);
    /// let v2 = Vector2::new(1.0, 0.0);
    /// let rejection = v1.reject_from(&v2);
    /// assert_eq!(rejection.x, 0.0);
    /// assert_eq!(rejection.y, 4.0);
    /// ```
    pub fn reject_from(&self, other: &Self) -> Self
    where
        T: Into<f64> + Copy + From<f64> + Sub<Output = T>,
    {
        let projection = self.project_onto(other);
        Self {
            x: self.x - projection.x,
            y: self.y - projection.y,
        }
    }

    /// Linearly interpolates between this vector and another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 2.0);
    /// let v2 = Vector2::new(3.0, 4.0);
    /// let interpolated = v1.lerp(&v2, 0.5);
    /// assert_eq!(interpolated.x, 2.0);
    /// assert_eq!(interpolated.y, 3.0);
    /// ```
    pub fn lerp(&self, other: &Self, t: f64) -> Self
    where
        T: Into<f64> + Copy + From<f64> + Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T>,
    {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }

    /// Computes the angle between this vector and another vector in radians.
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 0.0);
    /// let v2 = Vector2::new(0.0, 1.0);
    /// let angle = v1.angle_between(&v2);
    /// assert_eq!(angle, std::f64::consts::PI / 2.0); //FIXME: The PI constant should be available in the crate
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
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v = Vector2::new(1.0, 2.0);
    /// let swizzled = v.swizzle(1, 0);
    /// assert_eq!(swizzled.x, v.y);
    /// assert_eq!(swizzled.y, v.x);
    /// ```
    pub fn swizzle(&self, x: usize, y: usize) -> Self
    where
        T: Copy,
    {
        let components = [self.x, self.y];
        Self {
            x: components[x],
            y: components[y],
        }
    }
}

impl<T> Add for Vector2<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    /// Adds two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(1.0, 2.0);
    /// let v2 = Vector2::new(3.0, 4.0);
    /// let result = v1 + v2;
    /// assert_eq!(result.x, 4.0);
    /// assert_eq!(result.y, 6.0);
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector2<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    /// Subtracts one vector from another component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(3.0, 4.0);
    /// let v2 = Vector2::new(1.0, 2.0);
    /// let result = v1 - v2;
    /// assert_eq!(result.x, 2.0);
    /// assert_eq!(result.y, 2.0);
    /// ```
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul for Vector2<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;

    /// Multiplies two vectors component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(2.0, 3.0);
    /// let v2 = Vector2::new(4.0, 5.0);
    /// let result = v1 * v2;
    /// assert_eq!(result.x, 8.0);
    /// assert_eq!(result.y, 15.0);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T> Div for Vector2<T>
where
    T: Div<Output = T>,
{
    type Output = Self;

    /// Divides one vector by another component-wise.
    ///
    /// # Examples
    ///
    /// ```
    /// use vexel::vectors::vector2::Vector2;
    ///
    /// let v1 = Vector2::new(8.0, 15.0);
    /// let v2 = Vector2::new(2.0, 3.0);
    /// let result = v1 / v2;
    /// assert_eq!(result.x, 4.0);
    /// assert_eq!(result.y, 5.0);
    /// ```
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
