use crate::prelude::*;

#[allow(dead_code)]
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
        for (_, (collider,)) in query!(model.triggers, (&collider)) {
            self.draw_collider(&collider.clone(), Color::GREEN, &model.camera, framebuffer);
        }
        for (_, (collider,)) in query!(model.clouds, (&body.collider)) {
            self.draw_collider(&collider.clone(), Color::WHITE, &model.camera, framebuffer);
        }
        for (_, (collider,)) in query!(model.bodies, (&collider)) {
            self.draw_collider(&collider.clone(), Color::WHITE, &model.camera, framebuffer);
        }
        for (_, (collider,)) in query!(model.birds, (&body.collider)) {
            self.draw_collider(&collider.clone(), Color::BLUE, &model.camera, framebuffer);
        }
        for (_, (collider,)) in query!(model.projectiles, (&body.collider)) {
            self.draw_collider(&collider.clone(), Color::RED, &model.camera, framebuffer);
        }
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
