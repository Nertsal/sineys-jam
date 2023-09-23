use crate::{prelude::*, render::GameRender};

use geng::{Key, MouseButton};

#[allow(dead_code)]
pub struct Game {
    geng: Geng,
    render: GameRender,
    model: Model,
    jump: bool,
    shoot: bool,
    cursor_pos: vec2<f64>,
}

impl Game {
    pub fn new(geng: &Geng, assets: &Rc<Assets>) -> Self {
        Self {
            geng: geng.clone(),
            render: GameRender::new(geng, assets),
            model: Model::new(geng.clone(), assets.clone()),
            jump: false,
            shoot: false,
            cursor_pos: vec2::ZERO,
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
            jump: std::mem::take(&mut self.jump),
            shoot: std::mem::take(&mut self.shoot),
        }
    }
}

impl geng::State for Game {
    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::BLACK), None, None);
        self.model.camera.framebuffer_size = framebuffer.size();
        self.render.draw(&self.model, framebuffer);
    }

    fn handle_event(&mut self, event: geng::Event) {
        if geng_utils::key::is_event_press(&event, [Key::Space]) {
            self.jump = true;
        }
        if geng_utils::key::is_event_press(&event, [MouseButton::Left]) {
            self.shoot = true;
        }

        if let geng::Event::CursorMove { position } = event {
            self.cursor_pos = position;
        }
    }

    fn transition(&mut self) -> Option<geng::state::Transition> {
        self.model.transition.take()
    }

    fn update(&mut self, delta_time: f64) {
        let delta_time = Time::new(delta_time as _);

        self.model.camera.cursor_pos = self.cursor_pos;

        let input = self.player_input();
        self.model.update(input, delta_time);
    }
}
