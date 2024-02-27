use crate::prelude::*;

pub fn rand_norm() -> float {
    rand::random()
}

pub fn rand(min: float, max: float) -> float {
    min + (max - min) * rand_norm()
}
