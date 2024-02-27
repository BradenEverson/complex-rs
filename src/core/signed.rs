#[allow(non_camel_case_types)]
//I want it to look like the traditional prim types
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

impl c32 {
    pub fn new(real: i16, imaginary: i16) -> c32 {
        c32 { real, imaginary }
    }
    
}
