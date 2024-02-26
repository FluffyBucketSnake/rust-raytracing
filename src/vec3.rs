use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { e: [x, y, z] }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn r(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn g(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    #[inline]
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Self::new(
            self.y() + other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    #[inline]
    pub fn lenght(&self) -> f32 {
        self.dot(self)
    }

    #[inline]
    pub fn unit(&self) -> Vec3 {
        self / self.lenght()
    }
}

impl Display for Vec3 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

macro_rules! impl_borrowed {
    ($lhs:ident, $rhs:ident, $trait:ident, $func:ident) => {
        impl $trait<&$rhs> for &$lhs {
            type Output = Vec3;

            fn $func(self, rhs: &$rhs) -> Self::Output {
                $trait::$func(*self, *rhs)
            }
        }

        impl $trait<&$rhs> for $lhs {
            type Output = Vec3;

            fn $func(self, rhs: &$rhs) -> Self::Output {
                $trait::$func(self, *rhs)
            }
        }

        impl $trait<$rhs> for &$lhs {
            type Output = Vec3;

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

            fn $func(self, rhs: Self) -> Self::Output {
                Self::new(
                    $trait::$func(self.x(), rhs.x()),
                    $trait::$func(self.y(), rhs.y()),
                    $trait::$func(self.z(), rhs.z()),
                )
            }
        }
        impl_borrowed!(Vec3, Vec3, $trait, $func);

        impl $trait<f32> for Vec3 {
            type Output = Self;

            fn $func(self, rhs: f32) -> Self::Output {
                Self::new(
                    $trait::$func(self.x(), rhs),
                    $trait::$func(self.y(), rhs),
                    $trait::$func(self.z(), rhs),
                )
            }
        }
        impl_borrowed!(Vec3, f32, $trait, $func);

        impl $trait<Vec3> for f32 {
            type Output = Vec3;

            fn $func(self, rhs: Vec3) -> Self::Output {
                $trait::$func(rhs, self)
            }
        }
        impl_borrowed!(f32, Vec3, $trait, $func);
    };
}

impl_element_wise_op!(Add, add);
impl_element_wise_op!(Sub, sub);
impl_element_wise_op!(Mul, mul);
impl_element_wise_op!(Div, div);

macro_rules! impl_element_wise_op_assign {
    ($trait:ident, $func:ident) => {
        impl $trait for Vec3 {
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
