mod collision;
mod control;
mod generation;
mod movement;

use super::*;

impl Model {
    pub fn update(&mut self, input: PlayerInput, delta_time: Time) {
        self.time += delta_time;

        self.generate_level(delta_time);

        self.player_control(input, delta_time);
        self.gravity(delta_time);
        self.movement(delta_time);

        self.collide_clouds(delta_time);
        self.collide_birds(delta_time);

        self.attached_triggers(delta_time);
        self.collide_triggers(delta_time);

        self.camera_control(delta_time);

        self.lifetime(delta_time);
    }

    fn gravity(&mut self, delta_time: Time) {
        let gravity = vec2(0.0, -9.8).as_r32() * delta_time;

        for id in self.doodles.ids() {
            let (velocity,) = get!(self.doodles, id, (&mut body.velocity)).unwrap();
            *velocity += gravity;
        }
    }

    fn lifetime(&mut self, delta_time: Time) {
        for id in self.projectiles.ids() {
            let (lifetime,) = get!(self.projectiles, id, (&mut lifetime)).unwrap();
            lifetime.change(-delta_time);
            if lifetime.is_min() {
                self.projectiles.remove(id);
            }
        }
    }
}
