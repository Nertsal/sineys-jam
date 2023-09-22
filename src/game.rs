use crate::{prelude::*, render::GameRender};

use geng::Key;

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    render: GameRender,
    model: Model,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            render: GameRender::new(geng, assets),
            model: Model::new(),
        }
    }

    fn player_input(&mut self) -> PlayerInput {
        let mut move_dir = vec2::<f32>::ZERO;
        let window = self.geng.window();
        if geng_utils::key::is_key_pressed(window, [Key::A, Key::ArrowLeft]) {
            move_dir.x -= 1.0;
        }
        if geng_utils::key::is_key_pressed(window, [Key::D, Key::ArrowRight]) {
            move_dir.x += 1.0;
        }
        if geng_utils::key::is_key_pressed(window, [Key::S, Key::ArrowDown]) {
            move_dir.y -= 1.0;
        }
        if geng_utils::key::is_key_pressed(window, [Key::W, Key::ArrowUp]) {
            move_dir.y += 1.0;
        }
        let move_dir = move_dir.as_r32();

        PlayerInput {
            input_dir: move_dir,
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, _event: geng::Event) {}

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);

        let input = self.player_input();
        self.model.update(input, delta_time);
    }
}
