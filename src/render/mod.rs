use crate::prelude::*;

pub struct GameRender {
    geng: Geng,
    assets: Rc<Assets>,
}

impl GameRender {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
        }
    }

    pub fn draw(&mut self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        self.draw_background(model, framebuffer);

        for (_, (collider,)) in query!(model.triggers, (&collider)) {
            self.draw_sprite(
                &collider.clone(),
                &self.assets.sprites.spring,
                &model.camera,
                framebuffer,
            );
        }
        for (_, (collider,)) in query!(model.clouds, (&body.collider)) {
            self.draw_sprite(
                &collider.clone(),
                &self.assets.sprites.cloud,
                &model.camera,
                framebuffer,
            );
        }
        for (_, (collider,)) in query!(model.doodles, (&body.collider)) {
            self.draw_animation(
                &collider.clone(),
                &self.assets.sprites.doodle,
                model.time,
                &model.camera,
                framebuffer,
            );
        }
        for (_, (collider,)) in query!(model.birds, (&body.collider)) {
            self.draw_animation(
                &collider.clone(),
                &self.assets.sprites.bird,
                model.time,
                &model.camera,
                framebuffer,
            );
        }
        for (_, (collider,)) in query!(model.projectiles, (&body.collider)) {
            self.draw_sprite(
                &collider.clone(),
                &self.assets.sprites.bullet,
                &model.camera,
                framebuffer,
            );
        }
    }

    fn draw_background(&self, model: &Model, framebuffer: &mut ugli::Framebuffer) {
        let background = &self.assets.sprites.background;
        let mut background_size = background.size().as_f32();
        background_size *= model.world_width.as_f32() / background_size.x;

        let camera_pos = model.camera.center.shifted(model.camera.offset_center);
        // let camera_height = camera_pos.to_world().y;
        // let low = camera_height - model.camera.fov;
        // let high = camera_height + model.camera.fov;

        let parallax = vec2(1.0, 0.8).as_r32();
        let target = camera_pos.as_dir() * parallax;
        let target = Position::from_world(target, model.world_width);

        let delta = camera_pos.delta_to(target);
        let delta = (delta.as_f32() / background_size).map(f32::fract) * background_size;
        let target = camera_pos.shifted(delta.as_r32());

        let target = model.camera.project_f32(target);
        let target = Aabb2::point(target).extend_symmetric(background_size / 2.0);

        let translations = [
            vec2(0.0, 0.0),
            background_size * vec2::UNIT_Y,
            -background_size * vec2::UNIT_Y,
        ];
        for translation in translations {
            let target = target.translate(translation);
            self.geng.draw2d().draw2d(
                framebuffer,
                &model.camera,
                &draw2d::TexturedQuad::new(target, background),
            );
        }
    }

    fn draw_animation(
        &self,
        collider: &Collider,
        animation: &[GifFrame],
        mut time: Time,
        camera: &Camera,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let duration: Time = animation
            .iter()
            .map(|frame| frame.duration)
            .fold(Time::ZERO, |a, b| a + b.as_r32());
        time = (time / duration).fract() * duration;

        let mut i = 0;
        for (ix, frame) in animation.iter().enumerate() {
            i = ix;
            if time.as_f32() < frame.duration {
                break;
            }
            time -= frame.duration.as_r32();
        }
        let frame = &animation[i];

        self.draw_sprite(collider, &frame.texture, camera, framebuffer);
    }

    fn draw_sprite(
        &self,
        collider: &Collider,
        texture: &ugli::Texture,
        camera: &Camera,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let pos = camera.project_f32(collider.position);
        let target = collider.compute_aabb().map(Coord::as_f32);
        let target = target.translate(pos - target.center());

        let target = geng_utils::layout::fit_aabb_width(texture.size().as_f32(), target, 1.0);
        self.geng.draw2d().draw2d(
            framebuffer,
            camera,
            &draw2d::TexturedQuad::new(target, texture),
        );
    }

    fn draw_collider(
        &self,
        collider: &Collider,
        color: Color,
        camera: &Camera,
        framebuffer: &mut ugli::Framebuffer,
    ) {
        let pos = camera.project_f32(collider.position);
        let rotation = collider.rotation.map(R32::as_f32);

        match collider.shape {
            Shape::Circle { radius } => self.geng.draw2d().draw2d(
                framebuffer,
                camera,
                &draw2d::Ellipse::circle(pos, radius.as_f32(), color),
            ),
            Shape::Rectangle { width, height } => self.geng.draw2d().draw2d(
                framebuffer,
                camera,
                &draw2d::Quad::new(
                    Aabb2::ZERO.extend_symmetric(vec2(width, height).as_f32() / 2.0),
                    color,
                )
                .rotate(rotation)
                .translate(pos),
            ),
        }
    }
}
