use std::{
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Not, Sub, SubAssign},
};

pub struct Complex(f32, f32);

impl Complex {
    pub const I: Self = Self::new(0.0, 1.0);
    pub const fn new(r: f32, im: f32) -> Self {
        Self(r, im)
    }
    pub const fn dot(&self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
    pub const fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn angle(&self) -> f32 {
        self.1.atan2(self.0)
    }
    pub const fn real(&self) -> f32 {
        self.0
    }
    pub const fn imag(&self) -> f32 {
        self.1
    }
}

impl Clone for Complex {
    fn clone(&self) -> Self {
        *self
    }
    fn clone_from(&mut self, source: &Self) {
        *self = *source;
    }
}
impl Copy for Complex {}

impl Debug for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == 0.0 {
            if self.1 == 0.0 {
                f.write_str("0")
            } else if self.1 == 1.0 {
                f.write_str("i")
            } else if self.1 == -1.0 {
                f.write_str("-i")
            } else {
                f.write_fmt(format_args!("{}i", self.1))
            }
        } else if self.1 < 0.0 {
            if self.1 == -1.0 {
                f.write_fmt(format_args!("{}-i", self.0))
            } else {
                f.write_fmt(format_args!("{}{}i", self.0, self.1))
            }
        } else {
            if self.1 == 1.0 {
                f.write_fmt(format_args!("{}+i", self.0))
            } else {
                f.write_fmt(format_args!("{}+{}i", self.0, self.1))
            }
        }
    }
}

impl Into<Complex> for f32 {
    fn into(self) -> Complex {
        Complex(self, 0.0)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0 || self.1 != other.1
    }
}

impl PartialEq<f32> for Complex {
    fn eq(&self, other: &f32) -> bool {
        self.0 == *other && self.1 == 0.0
    }
    fn ne(&self, other: &f32) -> bool {
        self.0 != *other || self.1 != 0.0
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.0 + rhs.0, self.1 + rhs.0)
    }
}

impl Add<f32> for Complex {
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.0 + rhs, self.1)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl AddAssign<f32> for Complex {
    fn add_assign(&mut self, rhs: f32) {
        self.0 += rhs;
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.0 - rhs.0, self.1 - rhs.0)
    }
}

impl Sub<f32> for Complex {
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self::new(self.0 - rhs, self.1)
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl SubAssign<f32> for Complex {
    fn sub_assign(&mut self, rhs: f32) {
        self.0 -= rhs;
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.0, -self.1)
    }
}

impl Not for Complex {
    type Output = Self;
    fn not(self) -> Self::Output {
        Self::new(self.1, self.0)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Mul<f32> for Complex {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs)
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let r = self.0 * rhs.0 - self.1 * rhs.1;
        let i = self.0 * rhs.1 + self.1 * rhs.0;
        self.0 = r;
        self.1 = i;
    }
}

impl MulAssign<f32> for Complex {
    fn mul_assign(&mut self, rhs: f32) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let d = rhs.0 * rhs.0 + rhs.1 * rhs.1;
        Complex::new(
            (self.0 * rhs.0 + self.1 * rhs.1) / d,
            (self.1 * rhs.0 - self.0 * rhs.1) / d,
        )
    }
}

impl Div<f32> for Complex {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Complex::new(self.0 / rhs, self.1 / rhs)
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, rhs: Self) {
        let d = rhs.0 * rhs.0 + rhs.1 * rhs.1;
        let r = (self.0 * rhs.0 + self.1 * rhs.1) / d;
        let i = (self.1 * rhs.0 - self.0 * rhs.1) / d;
        self.0 = r;
        self.1 = i;
    }
}

impl DivAssign<f32> for Complex {
    fn div_assign(&mut self, rhs: f32) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}
