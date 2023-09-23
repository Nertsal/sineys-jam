use super::*;

pub struct EndScreen {
    geng: Geng,
    assets: Rc<assets::Assets>,
    transition: Option<geng::state::Transition>,
    score: i32,
}

impl EndScreen {
    pub fn new(geng: &Geng, assets: &Rc<assets::Assets>, score: i32) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            transition: None,
            score,
        }
    }
}

impl geng::State for EndScreen {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        let camera = geng::Camera2d {
            center: vec2::ZERO,
            rotation: Angle::ZERO,
            fov: 10.0,
        };
        ugli::clear(framebuffer, Some("#e3e9f8".try_into().unwrap()), None, None);
        self.geng.draw2d().draw2d(
            framebuffer,
            &camera,
            &draw2d::TexturedQuad::unit(&self.assets.end).scale(
                vec2(self.assets.end.size().map(|x| x as f32).aspect(), 1.0) * camera.fov / 2.0,
            ),
        );
        self.geng.default_font().draw(
            framebuffer,
            &camera,
            &self.score.to_string(),
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(vec2(0.0, -3.0)),
            "#90455a".try_into().unwrap(),
        )
    }
    fn handle_event(&mut self, event: geng::Event) {
        if let geng::Event::KeyPress { key: geng::Key::R } = event {
            self.transition = Some(geng::state::Transition::Switch(Box::new(game::Game::new(
                &self.geng,
                &self.assets,
            ))));
        }
    }
    fn transition(&mut self) -> Option<geng::state::Transition> {
        self.transition.take()
    }
}
