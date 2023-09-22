use super::*;

pub struct Player {
    pub body: Id,
}

pub struct PlayerInput {
    pub input_dir: vec2<Coord>,
    pub jump: bool,
}
