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

// Nertsal: why do all compoennts need to be Clone + Debug???????????
#[derive(Copy, Clone)]
pub struct Instant {
    inner: time::Instant,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            inner: time::Instant::now(),
        }
    }
    pub fn elapsed(&self) -> time::Duration {
        self.inner.elapsed()
    }
}

impl std::fmt::Debug for Instant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "instant :)")
    }
}

#[derive(SplitFields)]
pub struct Doodle {
    #[split(nested)]
    pub body: Body,
    pub grounded: Option<Id>,
    pub active_triggers: Vec<Id>,
    pub coyote_timer: Instant,
    pub shoot_timer: Instant,
}

impl Doodle {
    pub fn new(body: Body) -> Self {
        Self {
            body,
            grounded: None,
            active_triggers: Vec::new(),
            coyote_timer: Instant::now(),
            shoot_timer: Instant::now(),
        }
    }
}

#[derive(SplitFields)]
pub struct Cloud {
    #[split(nested)]
    pub body: Body,
    pub anchor: Position,
    pub anchor_velocity: vec2<Coord>,
}

impl Cloud {
    pub fn new(position: Position) -> Self {
        let body = Body::new(Collider::new(position, Shape::rectangle(1.5, 0.5)), 5.0);
        Self {
            anchor: body.collider.position,
            anchor_velocity: vec2::ZERO,
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
    pub lifetime: Lifetime,
}

impl Bird {
    pub fn new(position: Position, speed: impl Float) -> Self {
        let mut body = Body::new(Collider::new(position, Shape::circle(0.5)), 30.0);
        body.velocity = vec2::UNIT_X * speed.as_r32();
        Self {
            body,
            lifetime: Lifetime::new_max(r32(5.0)),
        }
    }
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
    Coin,
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

    pub fn coin(cloud: Option<Id>, world_width: Coord) -> Self {
        let position = Position::zero(world_width);
        let collider = Collider::new(position, Shape::circle(0.3));
        Self {
            kind: TriggerKind::Coin,
            collider,
            attached_to: cloud.map(|cloud| Attachment {
                relative_pos: vec2(0.0, 0.5).as_r32(),
                cloud,
            }),
        }
    }
}

#[derive(SplitFields)]
pub struct Particle {
    #[split(nested)]
    pub body: Body,
    pub color: Color,
    pub lifetime: Lifetime,
}

impl Particle {
    pub fn new(position: Position, velocity: vec2<Coord>, color: Color) -> Self {
        let mut body = Body::new(
            Collider::new(position, Shape::Circle { radius: r32(0.1) }),
            1.0,
        );
        body.velocity = velocity;
        Self {
            body,
            color,
            lifetime: Lifetime::new_max(r32(0.6)),
        }
    }
}
