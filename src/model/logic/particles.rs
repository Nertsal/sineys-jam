use super::*;

impl Model {
    pub fn spawn_particles(
        &mut self,
        intensity: R32,
        position: Position,
        velocity: vec2<Coord>,
        color: Color,
    ) {
        let mut rng = thread_rng();
        let position_radius = r32(0.2);

        let amount = if intensity.as_f32() < 1.0 {
            usize::from(rng.gen_bool(intensity.as_f32().into()))
        } else {
            intensity.as_f32().ceil().max(0.0) as usize
        };

        for _ in 0..amount {
            let pos = rng.gen_circle(vec2::ZERO, position_radius);
            let pos = position.shifted(pos);
            self.particles.insert(Particle::new(pos, velocity, color));
        }
    }

    pub fn update_particles(&mut self, delta_time: Time) {
        for id in self.particles.ids() {
            let (position, &velocity, lifetime) = get!(
                self.particles,
                id,
                (&mut body.collider.position, &body.velocity, &mut lifetime)
            )
            .unwrap();

            lifetime.change(-delta_time);
            if lifetime.is_min() {
                self.particles.remove(id);
                continue;
            }

            position.shift(velocity * delta_time);
        }
    }
}
