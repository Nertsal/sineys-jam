mod collider;
mod position;

pub use self::{collider::*, position::*};

use super::*;

pub type Lifetime = geng_utils::bounded::Bounded<Time>;

#[derive(SplitFields)]
pub struct Body {
    #[split(nested)]
    pub collider: Collider,
    pub velocity: vec2<Coord>,
    pub mass: R32,
    pub grounded: Option<Id>,
}

impl Body {
    pub fn new(collider: Collider, mass: impl Float) -> Self {
        Self {
            collider,
            velocity: vec2::ZERO,
            mass: mass.as_r32(),
            grounded: None,
        }
    }
}

#[derive(SplitFields)]
pub struct Cloud {
    #[split(nested)]
    pub body: Body,
    pub anchor: Position,
}

impl Cloud {
    pub fn new(body: Body) -> Self {
        Self {
            anchor: body.collider.position,
            body,
        }
    }
}

#[derive(SplitFields)]
pub struct Projectile {
    #[split(nested)]
    pub body: Body,
    pub lifetime: Lifetime,
}

impl Projectile {
    pub fn new(body: Body, lifetime: impl Float) -> Self {
        Self {
            body,
            lifetime: Lifetime::new_max(lifetime.as_r32()),
        }
    }
}
