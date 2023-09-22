use super::*;

impl Model {
    pub fn update(&mut self, delta_time: Time) {
        self.gravity(delta_time);
        self.movement(delta_time);
        self.collisions(delta_time);
    }

    fn gravity(&mut self, delta_time: Time) {
        let gravity = vec2(0.0, -9.8).as_r32() * delta_time;

        for id in self.bodies.ids() {
            let (velocity,) = get!(self.bodies, id, (&mut velocity)).unwrap();
            *velocity += gravity;
        }
    }

    fn movement(&mut self, delta_time: Time) {
        for id in self.bodies.ids() {
            let (position, &velocity) =
                get!(self.bodies, id, (&mut collider.position, &velocity)).unwrap();
            position.shift(velocity * delta_time);
        }
    }

    fn collisions(&mut self, _delta_time: Time) {
        // TODO
    }
}
