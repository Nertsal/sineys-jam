mod collider;
mod position;

pub use self::{collider::*, position::*};

use super::*;

#[derive(SplitFields)]
pub struct Body {
    #[split(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
    pub mass: R32,
}

impl Body {
    pub fn new(collider: Collider, mass: impl Float) -> Self {
        Self {
            collider,
            velocity: vec2::ZERO,
            mass: mass.as_r32(),
        }
    }
}
