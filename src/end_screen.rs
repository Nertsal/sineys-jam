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
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.geng.default_font().draw(
            framebuffer,
            &geng::Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: 10.0,
            },
            &format!("GG\n\nyour score:\n{}", self.score),
            vec2::splat(geng::TextAlign::CENTER),
            mat3::translate(vec2(0.0, 3.0)),
            Rgba::WHITE,
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
