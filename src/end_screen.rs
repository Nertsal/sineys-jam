use super::*;

pub struct EndScreen {
    geng: Geng,
    assets: Rc<assets::Assets>,
    transition: Option<geng::state::Transition>,
}

impl EndScreen {
    pub fn new(geng: &Geng, assets: &Rc<assets::Assets>) -> Self {
        Self {
            geng: geng.clone(),
            assets: assets.clone(),
            transition: None,
        }
    }
}

impl geng::State for EndScreen {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
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
