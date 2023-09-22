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

#[derive(SplitFields)]
pub struct Bird {
    #[split(nested)]
    pub body: Body,
}

#[derive(SplitFields)]
pub struct Trigger {
    pub kind: TriggerKind,
    #[split(nested)]
    pub collider: Collider,
    /// Cloud Id the trigger is attached to (moves together with).
    pub attached_to: Option<Attachment>,
}

#[derive(Debug, Clone)]
pub enum TriggerKind {
    Spring,
}

#[derive(Debug, Clone)]
pub struct Attachment {
    pub relative_pos: vec2<Coord>,
    pub cloud: Id,
}

impl Trigger {
    pub fn spring(cloud: Id, world_width: Coord) -> Self {
        let position = Position::zero(world_width);
        let collider = Collider::new(position, Shape::rectangle(0.4, 0.4));
        Self {
            kind: TriggerKind::Spring,
            collider,
            attached_to: Some(Attachment {
                relative_pos: vec2(0.0, 0.3).as_r32(),
                cloud,
            }),
        }
    }
}