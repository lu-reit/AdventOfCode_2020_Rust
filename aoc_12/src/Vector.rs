use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt;

pub type Scalar = i32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: Scalar,
    pub y: Scalar
}

impl Vector {
    pub fn new(x: Scalar, y: Scalar) -> Vector {
        Vector { x, y }
    }

    pub fn dist(&self) -> Scalar {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Display for Vector { 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

//---------- Vector addition ---------- 


impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

// Need this for static arrays
impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl<'a, 'b> Sub <&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

//---------- Elementwise vector multiplication ----------

impl Mul for Vector {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }
}

// Need this for static arrays
impl<'a, 'b> Mul<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn mul(self, other: &'b Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

//---------- Scalar/Vector multiplication ----------

impl Mul<Scalar> for Vector {
    type Output = Vector;

    fn mul(self, rhs: Scalar) -> Vector {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Mul<Vector> for Scalar {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y
        }
    }
}
