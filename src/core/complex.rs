use std::cmp::Ordering;

#[allow(non_camel_case_types)]
//I want it to look like the traditional prim types
#[derive(Debug)]
pub struct c32 {
    real: i16,
    imaginary: i16
}

impl std::ops::Add<c32> for c32 {
    type Output = c32;
    fn add(self, rhs: c32) -> Self::Output {
        c32 {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl std::ops::Sub<c32> for c32 {
    type Output = c32;
    fn sub(self, rhs: c32) -> Self::Output {
        c32 {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary
        }
    }
}

impl std::ops::Mul<c32> for c32 {
    type Output = c32;
    fn mul(self, rhs: c32) -> Self::Output {
        c32 {
            real: self.real * rhs.real,
            imaginary: self.imaginary * rhs.imaginary
        }
    }
}

impl std::ops::Div<c32> for c32 {
    type Output = c32;
    fn div(self, rhs: c32) -> Self::Output {
        c32 {
            real: self.real / rhs.real,
            imaginary: self.imaginary / rhs.imaginary
        }
    }
}

impl std::ops::Rem<c32> for c32 {
    type Output = c32;
    fn rem(self, rhs: c32) -> Self::Output {
        c32 {
            real: self.real % rhs.real,
            imaginary: self.imaginary % rhs.imaginary
        }
    }
}

impl std::ops::AddAssign<c32> for c32 {
    fn add_assign(&mut self, rhs: c32) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}


impl std::ops::SubAssign<c32> for c32 {
    fn sub_assign(&mut self, rhs: c32) {
        self.real -= rhs.real;
        self.imaginary -= rhs.imaginary;
    }
}

impl std::ops::MulAssign<c32> for c32 {
    fn mul_assign(&mut self, rhs: c32) {
        self.real *= rhs.real;
        self.imaginary *= rhs.imaginary;
    }
}

impl std::ops::DivAssign<c32> for c32 {
    fn div_assign(&mut self, rhs: c32) {
        self.real /= rhs.real;
        self.imaginary /= rhs.imaginary;
    }
}

impl std::ops::RemAssign<c32> for c32 {
    fn rem_assign(&mut self, rhs: c32) {
        self.real %= rhs.real;
        self.imaginary %= rhs.imaginary;
    }
}
impl c32 {
    pub fn new(real: i16, imaginary: i16) -> c32 {
        c32 { real, imaginary }
    }
    pub fn modulus(&self) -> f32 {
        (self.real.pow(2) as f32 + self.imaginary.pow(2) as f32).sqrt()
    }
    
}

impl PartialEq for c32 {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

impl PartialOrd for c32 {
    fn ge(&self, other: &Self) -> bool {
        self.modulus() >= other.modulus()
    }
    fn gt(&self, other: &Self) -> bool {
        self.modulus() > other.modulus()
    }
    fn lt(&self, other: &Self) -> bool {
        self.modulus() < other.modulus()
    }
    fn le(&self, other: &Self) -> bool {
        self.modulus() <= other.modulus()
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self == other {
                return Some(Ordering::Equal);
            } else if self.modulus() > other.modulus() {
                return Some(Ordering::Greater);
            } else if self.modulus() < other.modulus() {
                return Some(Ordering::Less);
            }
            None
    }
}

impl From<(i16, i16)> for c32 {
    fn from(value: (i16, i16)) -> Self {
        c32::new(value.0, value.1)
    } 
}

#[allow(non_camel_case_types)]
//I want it to look like the traditional prim types
#[derive(Debug)]
pub struct c64 {
    real: i32,
    imaginary: i32
}

impl std::ops::Add<c64> for c64 {
    type Output = c64;
    fn add(self, rhs: c64) -> Self::Output {
        c64 {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl std::ops::Sub<c64> for c64 {
    type Output = c64;
    fn sub(self, rhs: c64) -> Self::Output {
        c64 {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary
        }
    }
}

impl std::ops::Mul<c64> for c64 {
    type Output = c64;
    fn mul(self, rhs: c64) -> Self::Output {
        c64 {
            real: self.real * rhs.real,
            imaginary: self.imaginary * rhs.imaginary
        }
    }
}

impl std::ops::Div<c64> for c64 {
    type Output = c64;
    fn div(self, rhs: c64) -> Self::Output {
        c64 {
            real: self.real / rhs.real,
            imaginary: self.imaginary / rhs.imaginary
        }
    }
}

impl std::ops::Rem<c64> for c64 {
    type Output = c64;
    fn rem(self, rhs: c64) -> Self::Output {
        c64 {
            real: self.real % rhs.real,
            imaginary: self.imaginary % rhs.imaginary
        }
    }
}

impl std::ops::AddAssign<c64> for c64 {
    fn add_assign(&mut self, rhs: c64) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    }
}


impl std::ops::SubAssign<c64> for c64 {
    fn sub_assign(&mut self, rhs: c64) {
        self.real -= rhs.real;
        self.imaginary -= rhs.imaginary;
    }
}

impl std::ops::MulAssign<c64> for c64 {
    fn mul_assign(&mut self, rhs: c64) {
        self.real *= rhs.real;
        self.imaginary *= rhs.imaginary;
    }
}

impl std::ops::DivAssign<c64> for c64 {
    fn div_assign(&mut self, rhs: c64) {
        self.real /= rhs.real;
        self.imaginary /= rhs.imaginary;
    }
}

impl std::ops::RemAssign<c64> for c64 {
    fn rem_assign(&mut self, rhs: c64) {
        self.real %= rhs.real;
        self.imaginary %= rhs.imaginary;
    }
}
impl c64 {
    pub fn new(real: i32, imaginary: i32) -> c64 {
        c64 { real, imaginary }
    }
    pub fn modulus(&self) -> f64 {
        (self.real.pow(2) as f64 + self.imaginary.pow(2) as f64).sqrt()
    }
    
}

impl PartialEq for c64 {
    fn eq(&self, other: &Self) -> bool {
        self.real == other.real && self.imaginary == other.imaginary
    }
}

impl PartialOrd for c64 {
    fn ge(&self, other: &Self) -> bool {
        self.modulus() >= other.modulus()
    }
    fn gt(&self, other: &Self) -> bool {
        self.modulus() > other.modulus()
    }
    fn lt(&self, other: &Self) -> bool {
        self.modulus() < other.modulus()
    }
    fn le(&self, other: &Self) -> bool {
        self.modulus() <= other.modulus()
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self == other {
                return Some(Ordering::Equal);
            } else if self.modulus() > other.modulus() {
                return Some(Ordering::Greater);
            } else if self.modulus() < other.modulus() {
                return Some(Ordering::Less);
            }
            None
    }
}

impl From<(i32, i32)> for c64 {
    fn from(value: (i32, i32)) -> Self {
        c64::new(value.0, value.1)
    } 
}
