use super::*;

impl Model {
    pub fn generate_level(&mut self, delta_time: Time) {
        let mut rng = thread_rng();
        let (&player_pos, &player_vel) = get!(
            self.doodles,
            self.player.body,
            (&body.collider.position, &body.velocity)
        )
        .unwrap();

        if self.clouds.ids().is_empty() {
            // Initial stuff
            let positions = [(0.0, -3.0), (-2.0, -1.0), (1.0, 2.5)];
            let clouds: Vec<Id> = positions
                .into_iter()
                .map(|(x, y)| {
                    self.clouds.insert(Cloud::new(Position::from_world(
                        vec2(x, y).as_r32(),
                        self.world_width,
                    )))
                })
                .collect();

            self.triggers
                .insert(Trigger::spring(*clouds.last().unwrap(), self.world_width));

            self.birds.insert(Bird::new(
                Position::from_world(vec2(-3.0, 3.0).as_r32(), self.world_width),
                5.0,
            ));
        }

        // Birds
        if player_pos.to_world().y > r32(30.0) {
            self.next_bird -= delta_time;
            while self.next_bird < Time::ZERO {
                self.next_bird += rng.gen_range(0.7..=2.0).as_r32();

                let predict_time = rng.gen_range(1.0..=2.0).as_r32();
                let height = predict_time * player_vel.y;

                let position = player_pos.shifted(vec2(self.world_width / r32(2.0), height));

                let dir = if rng.gen() { 1.0 } else { -1.0 };
                let speed = rng.gen_range(4.0..=6.0);

                self.birds.insert(Bird::new(position, dir * speed));
            }
        }

        // Clouds
        let gen_ahead = 20.0.as_r32();
        while player_pos.to_world().y + gen_ahead > self.generated_height {
            let height = rng.gen_range(0.5..=2.0).as_r32();
            let y = self.generated_height + height;
            self.generated_height = y;
            let x = rng.gen_range(0.0..=self.world_width.as_f32()).as_r32();
            let position = Position::from_world(vec2(x, y), self.world_width);

            let mut cloud = Cloud::new(position);

            if rng.gen_bool(0.3) {
                // Moving cloud
                let dir = if rng.gen() { 1.0 } else { -1.0 }.as_r32();
                let speed = rng.gen_range(2.0..=4.0).as_r32();
                cloud.anchor_velocity = vec2::UNIT_X * dir * speed;
            }

            let cloud = self.clouds.insert(cloud);

            if rng.gen_bool(0.1) {
                // With a spring
                self.triggers
                    .insert(Trigger::spring(cloud, self.world_width));
            } else if rng.gen_bool(0.75) {
                // With a coin
                self.triggers
                    .insert(Trigger::coin(Some(cloud), self.world_width));
            }
        }
    }

    pub fn despawn_below(&mut self) {
        let low = self.camera.center.to_world().y - self.camera.fov;
        for id in self.clouds.ids() {
            let (&pos,) = get!(self.clouds, id, (&body.collider.position)).unwrap();
            if pos.to_world().y < low {
                self.clouds.remove(id);
            }
        }
        for id in self.triggers.ids() {
            let (&pos,) = get!(self.triggers, id, (&collider.position)).unwrap();
            if pos.to_world().y < low {
                self.triggers.remove(id);
            }
        }
    }
}
