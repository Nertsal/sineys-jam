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
        self.collide_triggers(delta_time);

        self.camera_control(delta_time);

        self.lifetime(delta_time);
    }

    fn player_control(&mut self, input: PlayerInput, delta_time: Time) {
        let (&position, velocity, &grounded, shoot_timer, &mass) = get!(
            self.doodles,
            self.player.body,
            (
                &body.collider.position,
                &mut body.velocity,
                &grounded,
                &mut shoot_timer,
                &body.mass
            )
        )
        .unwrap();
        let speed = 5.0.as_r32();
        let acceleration = 10.0.as_r32();
        let target_vel = input.input_dir.x.clamp_abs(Coord::ONE) * speed;
        let change = (target_vel - velocity.x) * acceleration * delta_time;
        velocity.x += change;

        if input.jump {
            if let Some(cloud) = grounded {
                let jump = 5.0.as_r32();
                let jump = vec2::UNIT_Y * jump;
                *velocity += jump;

                // Push off the cloud
                let (cloud_vel, &cloud_mass) =
                    get!(self.clouds, cloud, (&mut body.velocity, &body.mass)).unwrap();
                let cloud_factor = mass / (mass + cloud_mass);
                *cloud_vel -= jump * cloud_factor;

                self.assets.sfx.jump.play();
            }
        }

        if input.shoot && shoot_timer.elapsed().as_secs_f64() > 0.3 {
            self.assets.sfx.shoot.play();
            *shoot_timer = Instant::now();
            let target_pos = self.camera.cursor_pos_world();
            let delta = position.delta_to(target_pos);
            let dir = delta.normalize_or_zero();

            let speed = 10.0.as_r32();
            let mut proj = Projectile::new(
                Body::new(Collider::new(position, Shape::circle(0.2)), 1.0),
                1.0,
            );
            proj.body.velocity = dir * speed;
            self.projectiles.insert(proj);

            // Recoil
            let recoil = 5.0.as_r32();
            *velocity -= dir * recoil;
        }
    }

    fn camera_control(&mut self, delta_time: Time) {
        let (&player_pos, &_player_vel) = get!(
            self.doodles,
            self.player.body,
            (&body.collider.position, &body.velocity)
        )
        .unwrap();

        self.camera.target_position.shift({
            let mut delta = self.camera.target_position.delta_to(player_pos);
            delta.y = delta.y.max(R32::ZERO);
            delta
        });
        let delta = self.camera.center.delta_to(self.camera.target_position);
        let time = 0.3.as_r32();
        self.camera
            .center
            .shift(delta * (delta_time / time).min(R32::ONE));

        // Look ahead
        // let target_offset = (player_vel * 1.5.as_r32()).clamp_len(..=5.0.as_r32());
        // let delta = target_offset - self.camera.offset_center;
        // let time = 0.5.as_r32();
        // self.camera.offset_center = delta * (delta_time / time).min(R32::ONE);
    }

    fn gravity(&mut self, delta_time: Time) {
        let gravity = vec2(0.0, -9.8).as_r32() * delta_time;

        for id in self.doodles.ids() {
            let (velocity,) = get!(self.doodles, id, (&mut body.velocity)).unwrap();
            *velocity += gravity;
        }
    }

    fn movement(&mut self, delta_time: Time) {
        // Bodies
        for id in self.doodles.ids() {
            let (position, velocity) = get!(
                self.doodles,
                id,
                (&mut body.collider.position, &mut body.velocity)
            )
            .unwrap();
            *velocity = velocity.clamp_len(..=40.0.as_r32());
            position.shift(*velocity * delta_time);
        }

        // Birds
        for id in self.birds.ids() {
            let (position, &velocity) = get!(
                self.birds,
                id,
                (&mut body.collider.position, &body.velocity)
            )
            .unwrap();
            position.shift((velocity) * delta_time);
        }

        // Clouds
        for id in self.clouds.ids() {
            let (position, velocity, &anchor) = get!(
                self.clouds,
                id,
                (&mut body.collider.position, &mut body.velocity, &anchor)
            )
            .unwrap();

            // Dampen
            let damp = 10.0.as_r32();
            *velocity -= velocity.clamp_len(..=Coord::ONE) * damp * delta_time;

            // Move towards the anchor
            let direction = position.delta_to(anchor);
            let elasticity = 30.0.as_r32();
            *velocity += direction * direction.len().sqr() * elasticity;

            position.shift(*velocity * delta_time);
        }

        // Projectiles
        for id in self.projectiles.ids() {
            let (position, &velocity) = get!(
                self.projectiles,
                id,
                (&mut body.collider.position, &body.velocity)
            )
            .unwrap();
            position.shift(velocity * delta_time);
        }

        // Triggers that are attached
        for id in self.triggers.ids() {
            let Some((position, attachment)) = get!(
                self.triggers,
                id,
                (&mut collider.position, &attached_to.Get.Some)
            ) else {
                continue;
            };

            let Some((&cloud_pos,)) =
                get!(self.clouds, attachment.cloud, (&body.collider.position))
            else {
                continue;
            };

            *position = cloud_pos.shifted(attachment.relative_pos);
        }
    }

    fn collide_clouds(&mut self, delta_time: Time) {
        for body_id in self.doodles.ids() {
            let (&body_mass, body_collider, body_vel, body_grounded, coyote_timer) = get!(
                self.doodles,
                body_id,
                (
                    &body.mass,
                    &mut body.collider,
                    &mut body.velocity,
                    &mut grounded,
                    &mut coyote_timer,
                )
            )
            .unwrap();
            let body_col = body_collider.clone();
            if coyote_timer.elapsed().as_secs_f64() > 0.2 {
                *body_grounded = None;
            }

            for cloud_id in self.clouds.ids() {
                let (&cloud_mass, cloud_collider, cloud_vel) = get!(
                    self.clouds,
                    cloud_id,
                    (&body.mass, &mut body.collider, &mut body.velocity)
                )
                .unwrap();

                if let Some(collision) = body_col.collide(&cloud_collider.clone()) {
                    let relative_vel = *body_vel - *cloud_vel;
                    // Collide only when moving down
                    if relative_vel.y > Coord::ZERO || collision.normal.y > Coord::ZERO {
                        continue;
                    }

                    *body_grounded = Some(cloud_id);
                    *coyote_timer = Instant::now();

                    let body_factor = cloud_mass / (body_mass + cloud_mass);
                    let cloud_factor = body_mass / (body_mass + cloud_mass);

                    // Move the cloud
                    let penetration = collision.normal * collision.penetration;
                    let cloud_shift_speed = 5.0.as_r32();
                    cloud_collider.position.shift(
                        vec2::UNIT_Y * penetration.y.clamp_abs(cloud_shift_speed * delta_time),
                    );

                    // Fix horizontal velocity
                    cloud_vel.y += relative_vel.y * cloud_factor;
                    body_vel.y -= relative_vel.y * body_factor;
                }
            }
        }
    }

    fn collide_birds(&mut self, _delta_time: Time) {
        'bird: for bird_id in self.birds.ids() {
            let (&bird_mass, bird_collider, &bird_vel) = get!(
                self.birds,
                bird_id,
                (&body.mass, &mut body.collider, &body.velocity)
            )
            .unwrap();
            let bird_col = bird_collider.clone();

            for body_id in self.doodles.ids() {
                let (&body_mass, body_collider, body_vel) = get!(
                    self.doodles,
                    body_id,
                    (&body.mass, &mut body.collider, &mut body.velocity)
                )
                .unwrap();
                let body_col = body_collider.clone();

                if let Some(_collision) = body_col.collide(&bird_col) {
                    let body_factor = bird_mass / body_mass;
                    *body_vel += bird_vel * body_factor;
                    self.birds.remove(bird_id);
                    self.assets.sfx.oi.play();
                    continue 'bird;
                }
            }

            for proj_id in self.projectiles.ids() {
                let (proj_collider,) = get!(self.projectiles, proj_id, (&body.collider)).unwrap();
                let proj_col = proj_collider.clone();

                if let Some(_collision) = bird_col.collide(&proj_col) {
                    self.projectiles.remove(proj_id);
                    self.birds.remove(bird_id);
                    self.assets.sfx.kill_bird.play();
                    continue 'bird;
                }
            }
        }
    }

    fn collide_triggers(&mut self, _delta_time: Time) {
        for body_id in self.doodles.ids() {
            let (body_collider, body_vel, &body_mass, active_triggers) = get!(
                self.doodles,
                body_id,
                (
                    &body.collider,
                    &mut body.velocity,
                    &body.mass,
                    &mut active_triggers
                )
            )
            .unwrap();
            let body_col = body_collider.clone();

            let mut triggers = Vec::new();
            for trigger_id in self.triggers.ids() {
                let (trigger_kind, trigger_collider, attachment) =
                    get!(self.triggers, trigger_id, (&kind, &collider, &attached_to)).unwrap();
                let trigger_col = trigger_collider.clone();

                if let Some(_collision) = body_col.collide(&trigger_col) {
                    triggers.push(trigger_id);
                    if active_triggers.contains(&trigger_id) {
                        continue;
                    }

                    match trigger_kind {
                        TriggerKind::Spring => {
                            let jump = 10.0.as_r32();
                            let dir = vec2::UNIT_Y;
                            let jump = dir * jump;
                            *body_vel += jump;

                            // Make sure we jump up with a minimum speed
                            let min_jump_speed = 15.0.as_r32();
                            let proj = vec2::dot(*body_vel, dir);
                            *body_vel += dir * (min_jump_speed - proj).max(R32::ZERO);
                            self.assets.sfx.spring.play();

                            if let Some(attachment) = attachment {
                                if let Some((cloud_velocity, &cloud_mass)) = get!(
                                    self.clouds,
                                    attachment.cloud,
                                    (&mut body.velocity, &body.mass)
                                ) {
                                    let cloud_factor = body_mass / cloud_mass;
                                    *cloud_velocity -= jump * r32(0.3) * cloud_factor;
                                }
                            }
                        }
                    }
                }
            }
            *active_triggers = triggers;
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

    fn generate_level(&mut self, delta_time: Time) {
        let mut rng = thread_rng();
        let player_pos = self
            .doodles
            .body
            .collider
            .position
            .get(self.player.body)
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
        self.next_bird -= delta_time;
        while self.next_bird < Time::ZERO {
            self.next_bird += rng.gen_range(0.7..=2.0).as_r32();

            let height = rng.gen_range(1.0..=3.0).as_r32();
            let position = player_pos.shifted(vec2(self.world_width / r32(2.0), height));

            let dir = if rng.gen() { 1.0 } else { -1.0 };
            let speed = rng.gen_range(4.0..=6.0);

            self.birds.insert(Bird::new(position, dir * speed));
        }

        // Clouds
        let gen_ahead = 20.0.as_r32();
        while player_pos.to_world().y + gen_ahead > self.generated_height {
            let height = rng.gen_range(0.5..=2.0).as_r32();
            let y = self.generated_height + height;
            self.generated_height = y;
            let x = rng.gen_range(0.0..=self.world_width.as_f32()).as_r32();
            let position = Position::from_world(vec2(x, y), self.world_width);

            let cloud = self.clouds.insert(Cloud::new(position));
            if rng.gen_bool(0.1) {
                // With a spring
                self.triggers
                    .insert(Trigger::spring(cloud, self.world_width));
            }
        }
    }
}
