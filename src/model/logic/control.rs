use super::*;

impl Model {
    pub fn player_control(&mut self, input: PlayerInput, delta_time: Time) {
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
        let acceleration = 50.0.as_r32();
        let ground_vel = match grounded {
            Some(cloud) => {
                let (&vel, &anchor_vel) =
                    get!(self.clouds, cloud, (&body.velocity, &anchor_velocity)).unwrap();
                (vel + anchor_vel).x
            }
            None => Coord::ZERO,
        };
        let target_vel = input.input_dir.x.clamp_abs(Coord::ONE) * speed + ground_vel;
        let change = (target_vel - velocity.x).clamp_abs(acceleration * delta_time);
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

        if input.shoot && shoot_timer.elapsed().as_secs_f64() > 0.5 {
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

            self.spawn_particles(
                r32(3.0),
                position,
                dir * speed * r32(0.3),
                Rgba::try_from("#4B071A").unwrap(),
            );
        }
    }

    pub fn camera_control(&mut self, delta_time: Time) {
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
        let time = 0.2.as_r32();
        self.camera
            .center
            .shift(delta * (delta_time / time).min(R32::ONE));

        // Look ahead
        // let target_offset = (player_vel * 1.5.as_r32()).clamp_len(..=5.0.as_r32());
        // let delta = target_offset - self.camera.offset_center;
        // let time = 0.5.as_r32();
        // self.camera.offset_center = delta * (delta_time / time).min(R32::ONE);
    }
}
