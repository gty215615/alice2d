use std::{ops::{
    Add, AddAssign, Div, DivAssign,  Mul, MulAssign, Sub, SubAssign, Neg,
}};

#[derive(Debug,Default, Clone, Copy, PartialEq)]
pub struct Vector2<T>
where
    T: Copy + Default  
{
    pub x: T,
    pub y: T,
}


// Addition
impl<T> Add for Vector2<T>
where
    T: Copy + Default  + Add<Output = T>
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// AddAssign
impl<T> AddAssign for Vector2<T>
where
    T: Copy
        + Default
        + Add<Output = T>

{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// Addition scalar
impl<T> Add<T> for Vector2<T>
where
    T: Copy
        + Default
        + Add<Output = T>
{
    type Output = Self;

    fn add(self, other: T) -> Self::Output {
        Self {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

// AddAssign scalar
impl<T> AddAssign<T> for Vector2<T>
where
    T: Copy
        + Default
        + Add<Output = T>
{
    fn add_assign(&mut self, other: T) {
        *self = Self {
            x: self.x + other,
            y: self.y + other,
        };
    }
}

// Subtraction
impl<T> Sub for Vector2<T>
where
    T: Copy
        + Default
        + Sub<Output = T>
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// SubAssign
impl<T> SubAssign for Vector2<T>
where
    T: Copy
        + Default
        + Sub<Output = T>

        
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

// Subtraction scalar
impl<T> Sub<T> for Vector2<T>
where
    T: Copy
        + Default
        + Sub<Output = T>

        
{
    type Output = Self;

    fn sub(self, other: T) -> Self::Output {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

// SubAssign scalar
impl<T> SubAssign<T> for Vector2<T>
where
    T: Copy
        + Default
        + Sub<Output = T>

        
{
    fn sub_assign(&mut self, other: T) {
        *self = Self {
            x: self.x - other,
            y: self.y - other,
        };
    }
}

// Multiplication
impl<T> Mul for Vector2<T>
where
    T: Copy
        + Default

        + Mul<Output = T>

        
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

// MultiplicationAssign
impl<T> MulAssign for Vector2<T>
where
    T: Copy
        + Default

        + Mul<Output = T>

        
{
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

// Multiplication scalar
impl<T> Mul<T> for Vector2<T>
where
    T: Copy
        + Default

        + Mul<Output = T>

        
{
    type Output = Self;

    fn mul(self, other: T) -> Self::Output {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

// MultiplicationAssign scalar
impl<T> MulAssign<T> for Vector2<T>
where
    T: Copy
        + Default
        + Mul<Output = T>
        
{
    fn mul_assign(&mut self, other: T) {
        *self = Self {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

// Division
impl<T> Div for Vector2<T>
where
    T: Copy
        + Default

        + Div<Output = T>
        
{
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// DivisionAssign
impl<T> DivAssign for Vector2<T>
where
    T: Copy
        + Default
        + Div<Output = T>
        
{
    fn div_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

// Division scalar
impl<T> Div<T> for Vector2<T>
where
    T: Copy
        + Default
        + Div<Output = T>
        
{
    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

// DivisionAssign scalar
impl<T> DivAssign<T> for Vector2<T>
where
    T: Copy
        + Default
        + Div<Output = T>
        + Neg<Output = T>
{
    fn div_assign(&mut self, other: T) {
        *self = Self {
            x: self.x / other,
            y: self.y / other,
        };
    }
}


// DivisionAssign scalar
impl<T> Neg for Vector2<T>
where
    T: Copy
        + Default

        + Neg<Output = T>
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Vector2<T>
where
    T: Copy
        + Default
        + Div<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
{
    
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
    // LengthSquared
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }



    pub const ZERO:Vector2<f32> = Vector2{x:0.0,y:0.0};
}

impl<T> Vector2<T> 
where
    T: Copy
        + Default
        + Div<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
{

    pub fn rot_90(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}

impl<T> Vector2<T> 
where
    T: Copy
        + Default
        + Div<Output = T>
        + Neg<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Into<f32>
        + From<f32>
{
        // Length
        pub fn length(&self) -> f32 {
            let value = self.x * self.x + self.y * self.y;
            f32::sqrt(value.into())
        }
        // Normalize
        pub fn normalize(self) -> Vector2<f32> {
            let length = self.length();
            let x:f32 = self.x.into() / length;
            let y = self.y.into() / length;
            Vector2::<f32>::new(x,y)
        }
}

pub type Vector2f = Vector2<f32>;
pub type Vector2u = Vector2<u32>;
pub type Vector2i = Vector2<i32>;


unsafe impl bytemuck::Pod for Vector2f {}
unsafe impl bytemuck::Zeroable for Vector2f {}

