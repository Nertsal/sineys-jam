use super::*;

impl Model {
    pub fn update(&mut self, input: PlayerInput, delta_time: Time) {
        self.generate_level(delta_time);

        self.player_control(input, delta_time);
        self.gravity(delta_time);
        self.movement(delta_time);

        self.collide_clouds(delta_time);

        self.camera_control(delta_time);

        self.lifetime(delta_time);
    }

    fn player_control(&mut self, input: PlayerInput, delta_time: Time) {
        let (&position, velocity, &grounded, &mass) = get!(
            self.bodies,
            self.player.body,
            (&collider.position, &mut velocity, &grounded, &mass)
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
            }
        }

        if input.shoot {
            let target_pos = self.camera.cursor_pos_world();
            let delta = position.delta_to(target_pos);
            let dir = delta.normalize_or_zero();

            let speed = 10.0.as_r32();
            let mut proj = Projectile::new(
                Body::new(
                    Collider::new(
                        position,
                        Shape::Circle {
                            radius: 0.2.as_r32(),
                        },
                    ),
                    1.0,
                ),
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
            self.bodies,
            self.player.body,
            (&collider.position, &velocity)
        )
        .unwrap();

        self.camera.target_position = player_pos;
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

        for id in self.bodies.ids() {
            let (velocity,) = get!(self.bodies, id, (&mut velocity)).unwrap();
            *velocity += gravity;
        }
    }

    fn movement(&mut self, delta_time: Time) {
        // Bodies
        for id in self.bodies.ids() {
            let (position, &velocity) =
                get!(self.bodies, id, (&mut collider.position, &velocity)).unwrap();
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
            let elasticity = 1.0.as_r32();
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
    }

    fn collide_clouds(&mut self, _delta_time: Time) {
        for body_id in self.bodies.ids() {
            let (&body_mass, body_collider, body_vel, body_grounded) = get!(
                self.bodies,
                body_id,
                (&mass, &mut collider, &mut velocity, &mut grounded)
            )
            .unwrap();
            let body_col = body_collider.clone();
            *body_grounded = None;

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
                    if relative_vel.y > Coord::ZERO {
                        continue;
                    }

                    *body_grounded = Some(cloud_id);

                    let body_factor = cloud_mass / (body_mass + cloud_mass);
                    let cloud_factor = body_mass / (body_mass + cloud_mass);

                    // Move the cloud
                    let penetration = collision.normal * collision.penetration;
                    cloud_collider.position.shift(vec2::UNIT_Y * penetration.y);

                    // Fix horizontal velocity
                    cloud_vel.y += relative_vel.y * cloud_factor;
                    body_vel.y -= relative_vel.y * body_factor;
                }
            }
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

    fn generate_level(&mut self, _delta_time: Time) {
        if !self.clouds.ids().is_empty() {
            return;
        }

        let positions = [(0.0, -3.0), (-2.0, -1.0)];

        for (x, y) in positions {
            self.clouds.insert(Cloud::new(Body::new(
                Collider::new(
                    Position::from_world(vec2(x, y).as_r32(), self.world_width),
                    Shape::Rectangle {
                        width: 1.5.as_r32(),
                        height: 0.5.as_r32(),
                    },
                ),
                2.0,
            )));
        }
    }
}
