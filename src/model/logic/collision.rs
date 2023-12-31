use super::*;

impl Model {
    pub fn collide_clouds(&mut self, delta_time: Time) {
        let mut particles = Vec::new();
        let mut target_shhh_volume = 0.0_f64;
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

                    if body_grounded.is_none() {
                        particles.push((
                            r32(5.0),
                            collision.point,
                            -vec2::UNIT_Y * r32(0.1),
                            Color::try_from("#5772B5").unwrap(),
                        ));
                    }

                    *body_grounded = Some(cloud_id);
                    *coyote_timer = Instant::now();

                    target_shhh_volume = target_shhh_volume
                        .max((relative_vel.y.abs().as_f32() as f64 / 5.0).clamp(0.3, 1.0));
                    if self.shhh.is_none() {
                        self.shhh = Some(self.assets.sfx.shhh.effect());
                        let sfx = self.shhh.as_mut().unwrap();
                        self.shhh_volume = target_shhh_volume;
                        sfx.set_volume(self.shhh_volume);
                        sfx.play();
                    }

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

        for (intensity, position, velocity, color) in particles {
            self.spawn_particles(intensity, position, velocity, color);
        }

        let fade_time = 0.3;
        self.shhh_volume += (target_shhh_volume - self.shhh_volume)
            .clamp_abs(delta_time.as_f32() as f64 / fade_time);
        if let Some(sfx) = &mut self.shhh {
            sfx.set_volume(self.shhh_volume);
            if self.shhh_volume <= 1e-5 {
                sfx.stop();
                self.shhh = None;
            }
        }
    }

    pub fn collide_birds(&mut self, _delta_time: Time) {
        let mut particles = Vec::new();

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
                    *body_vel -= vec2::UNIT_Y * body_vel.y * r32(0.5);
                    self.birds.remove(bird_id);
                    self.assets.sfx.oi.play();
                    self.score -= 50;
                    particles.push((
                        r32(5.0),
                        *body_collider.position,
                        bird_vel * r32(0.3),
                        Color::try_from("#B16B7E").unwrap(),
                    ));
                    continue 'bird;
                }
            }

            for proj_id in self.projectiles.ids() {
                let (proj_collider, &proj_vel) =
                    get!(self.projectiles, proj_id, (&body.collider, &body.velocity)).unwrap();
                let proj_col = proj_collider.clone();

                if let Some(_collision) = bird_col.collide(&proj_col) {
                    self.projectiles.remove(proj_id);
                    self.birds.remove(bird_id);
                    self.assets.sfx.kill_bird.play();
                    self.score += 100;
                    particles.push((
                        r32(3.0),
                        bird_col.position,
                        proj_vel * r32(0.3),
                        Color::try_from("#4B071A").unwrap(),
                    ));
                    continue 'bird;
                }
            }
        }

        for (intensity, position, velocity, color) in particles {
            self.spawn_particles(intensity, position, velocity, color);
        }
    }

    pub fn collide_triggers(&mut self, _delta_time: Time) {
        let mut particles = Vec::new();
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

                            particles.push((
                                r32(5.0),
                                trigger_col.position,
                                -vec2::UNIT_Y * r32(0.2),
                                Color::try_from("#2148AB").unwrap(),
                            ));
                        }
                        TriggerKind::Coin => {
                            self.triggers.remove(trigger_id);
                            self.score += 100;
                            let mut sfx = self.assets.sfx.coin.effect();
                            sfx.set_volume(0.2);
                            sfx.play();
                            particles.push((
                                r32(5.0),
                                trigger_col.position,
                                vec2::ZERO,
                                Color::try_from("#E6AC4C").unwrap(),
                            ));
                        }
                    }
                }
            }
            *active_triggers = triggers;
        }

        for (intensity, position, velocity, color) in particles {
            self.spawn_particles(intensity, position, velocity, color);
        }
    }
}
