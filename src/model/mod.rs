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
    pub time: Time,
    pub world_width: Coord,
    /// The height up to which the world has been generated so far.
    pub generated_height: Coord,
    /// The time until the next bird spawns.
    pub next_bird: Time,
    pub camera: Camera,
    pub player: Player,
    pub doodles: StructOf<Arena<Doodle>>,
    pub birds: StructOf<Arena<Bird>>,
    pub clouds: StructOf<Arena<Cloud>>,
    pub projectiles: StructOf<Arena<Projectile>>,
    pub triggers: StructOf<Arena<Trigger>>,
    pub assets: Rc<Assets>,
}

impl Model {
    pub fn new(assets: Rc<Assets>) -> Self {
        let world_width = (35.0 * 0.55 ).as_r32();

        let mut doodles: StructOf<Arena<Doodle>> = default();
        let player_body = doodles.insert(Doodle::new(Body::new(
            Collider::new(Position::zero(world_width), Shape::rectangle(1.0, 1.0)),
            10.0,
        )));
        Self {
            assets,
            time: Time::ZERO,
            world_width,
            generated_height: Coord::ZERO,
            next_bird: Time::ZERO,
            // -3 so the clouds dont teleport (visibly) from one edge of the screen to the other
            // but disappear behind the edge instead
            // // * 0.55 because it was too wide Kapp
            camera: Camera::new(
                (world_width.as_f32() - 3.0) 
                    // * 0.55
                    * 9.0 / 16.0,
                world_width,
            ),
            player: Player { body: player_body },
            doodles,
            birds: default(),
            clouds: default(),
            projectiles: default(),
            triggers: default(),
        }
    }
}
