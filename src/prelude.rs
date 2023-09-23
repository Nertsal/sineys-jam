pub use crate::{assets::Assets, model::*};

pub use ecs::{
    prelude::*,
    storage::arena::{Arena, Index as Id},
};
pub use geng::prelude::*;
pub use geng_utils::{conversions::*, gif::GifFrame};

pub type Color = Rgba<f32>;
