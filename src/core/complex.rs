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

