mod camera;
mod components;
mod logic;
mod player;

pub use self::{camera::*, components::*, player::*};

use crate::prelude::*;

pub type Time = R32;
pub type Coord = R32;
pub type Position = PositionCylinder<Coord>;

pub struct Model {
    pub world_width: Coord,
    pub camera: Camera,
    pub player: Player,
    pub bodies: StructOf<Arena<Body>>,
    pub birds: StructOf<Arena<Bird>>,
    pub clouds: StructOf<Arena<Cloud>>,
    pub projectiles: StructOf<Arena<Projectile>>,
}

impl Model {
    pub fn new() -> Self {
        let world_width = 16.0.as_r32();

        let mut bodies: StructOf<Arena<Body>> = default();
        let player_body = bodies.insert(Body::new(
            Collider::new(
                Position::zero(world_width),
                Shape::Rectangle {
                    width: 1.0.as_r32(),
                    height: 1.0.as_r32(),
                },
            ),
            10.0,
        ));
        Self {
            world_width,
            camera: Camera::new(9.0, world_width),
            player: Player { body: player_body },
            bodies,
            birds: default(),
            clouds: default(),
            projectiles: default(),
        }
    }
}
