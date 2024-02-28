use std::{
    fmt::{Debug, Display},
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct Vec3 {
    pub e: [float; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::uniform(0.0);

    pub const X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const NEG_X: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
    pub const Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const NEG_Y: Vec3 = Vec3::new(0.0, -1.0, 0.0);
    pub const Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub const NEG_Z: Vec3 = Vec3::new(0.0, 0.0, -1.0);

    #[inline]
    pub const fn from_array(e: [float; 3]) -> Self {
        Self { e }
    }

    #[inline]
    pub const fn new(x: float, y: float, z: float) -> Self {
        Self::from_array([x, y, z])
    }

    #[inline]
    pub const fn uniform(value: float) -> Self {
        Self::new(value, value, value)
    }

    #[inline]
    pub fn random_norm() -> Self {
        Self::new(rand_norm(), rand_norm(), rand_norm())
    }

    #[inline]
    pub fn random(min: float, max: float) -> Self {
        Self::new(rand(min, max), rand(min, max), rand(min, max))
    }

    #[inline]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.lenght_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(rand(-1.0, 1.0), rand(-1.0, 1.0), 0.0);
            if p.lenght_squared() < 1.0 {
                return p;
            }
        }
    }

    #[inline]
    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    #[inline]
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let vector = Self::random_unit_vector();
        if vector.dot(&normal) > 0.0 {
            vector
        } else {
            -vector
        }
    }

    #[inline]
    pub fn x(&self) -> float {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> float {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> float {
        self.e[2]
    }

    #[inline]
    pub fn r(&self) -> float {
        self.e[0]
    }

    #[inline]
    pub fn g(&self) -> float {
        self.e[1]
    }

    #[inline]
    pub fn b(&self) -> float {
        self.e[2]
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> float {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    #[inline]
    pub fn lenght_squared(&self) -> float {
        self.dot(self)
    }

    #[inline]
    pub fn lenght(&self) -> float {
        self.dot(self).sqrt()
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        self / self.lenght()
    }

    #[inline]
    pub fn near_zero(&self) -> bool {
        const EPSILON: float = 1e-8;
        self.e[0].abs() < EPSILON && self.e[1].abs() < EPSILON && self.e[2].abs() < EPSILON
    }

    #[inline]
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    #[inline]
    pub fn refract(&self, normal: &Vec3, eta_ratio: float) -> Vec3 {
        let cos_theta = (-self).dot(normal).min(1.0);
        let out_ray_perp = eta_ratio * (self + cos_theta * normal);
        let out_ray_parallel = -((1.0 - out_ray_perp.lenght_squared()).sqrt()) * normal;
        return out_ray_perp + out_ray_parallel;
    }

    #[inline]
    pub fn map<F: FnMut(float) -> float>(self, f: F) -> Self {
        Self::from_array(self.e.map(f))
    }

    #[inline]
    pub fn gamma_corrected(self) -> Self {
        self.map(|c| c.sqrt())
    }
}

impl Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl Debug for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.e, f)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    #[inline]
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

macro_rules! impl_borrowed_bop {
    ($lhs:ident, $rhs:ident, $trait:ident, $func:ident) => {
        impl $trait<&$rhs> for &$lhs {
            type Output = Vec3;

            #[inline]
            fn $func(self, rhs: &$rhs) -> Self::Output {
                $trait::$func(*self, *rhs)
            }
        }

        impl $trait<&$rhs> for $lhs {
            type Output = Vec3;

            #[inline]
            fn $func(self, rhs: &$rhs) -> Self::Output {
                $trait::$func(self, *rhs)
            }
        }

        impl $trait<$rhs> for &$lhs {
            type Output = Vec3;

            #[inline]
            fn $func(self, rhs: $rhs) -> Self::Output {
                $trait::$func(*self, rhs)
            }
        }
    };
}

macro_rules! impl_element_wise_op {
    ($trait:ident, $func:ident) => {
        impl $trait for Vec3 {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: Self) -> Self::Output {
                Self::new(
                    $trait::$func(self.x(), rhs.x()),
                    $trait::$func(self.y(), rhs.y()),
                    $trait::$func(self.z(), rhs.z()),
                )
            }
        }
        impl_borrowed_bop!(Vec3, Vec3, $trait, $func);

        impl $trait<float> for Vec3 {
            type Output = Self;

            #[inline]
            fn $func(self, rhs: float) -> Self::Output {
                Self::new(
                    $trait::$func(self.x(), rhs),
                    $trait::$func(self.y(), rhs),
                    $trait::$func(self.z(), rhs),
                )
            }
        }
        impl_borrowed_bop!(Vec3, float, $trait, $func);

        impl $trait<Vec3> for float {
            type Output = Vec3;

            #[inline]
            fn $func(self, rhs: Vec3) -> Self::Output {
                $trait::$func(rhs, self)
            }
        }
        impl_borrowed_bop!(float, Vec3, $trait, $func);
    };
}

impl_element_wise_op!(Add, add);
impl_element_wise_op!(Sub, sub);
impl_element_wise_op!(Mul, mul);
impl_element_wise_op!(Div, div);

macro_rules! impl_element_wise_op_assign {
    ($trait:ident, $func:ident) => {
        impl $trait for Vec3 {
            #[inline]
            fn $func(&mut self, rhs: Self) {
                $trait::$func(&mut self.e[0], rhs.e[0]);
                $trait::$func(&mut self.e[1], rhs.e[1]);
                $trait::$func(&mut self.e[2], rhs.e[2]);
            }
        }
    };
}

impl_element_wise_op_assign!(AddAssign, add_assign);
impl_element_wise_op_assign!(SubAssign, sub_assign);
impl_element_wise_op_assign!(MulAssign, mul_assign);
impl_element_wise_op_assign!(DivAssign, div_assign);

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3::ZERO, |acc, current| acc + current)
    }
}
