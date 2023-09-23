use super::*;

impl Model {
    pub fn movement(&mut self, delta_time: Time) {
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
            let (position, velocity, anchor, &anchor_velocity) = get!(
                self.clouds,
                id,
                (
                    &mut body.collider.position,
                    &mut body.velocity,
                    &mut anchor,
                    &anchor_velocity
                )
            )
            .unwrap();

            // Move the anchor
            anchor.shift(anchor_velocity * delta_time);

            // Dampen
            let damp = 10.0.as_r32();
            let damp = velocity.clamp_len(..=Coord::ONE) * damp * delta_time;
            *velocity -= damp;

            // Move towards the anchor
            let direction = position.delta_to(*anchor);
            let elasticity = 200.0.as_r32();
            *velocity += direction.normalize_or_zero()
                * direction.len().sqr().min(10.0.as_r32())
                * elasticity
                * delta_time;

            position.shift((*velocity + anchor_velocity) * delta_time);
            *position = anchor.shifted(anchor.delta_to(*position).clamp_len(..=5.0.as_r32()))
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

    pub fn attached_triggers(&mut self, _delta_time: Time) {
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
}
