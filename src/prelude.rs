pub use crate::{assets::Assets, model::*};

pub use ecs::{
    prelude::*,
    storage::arena::{Arena, Index as Id},
};
pub use geng::prelude::*;
pub use geng_utils::conversions::*;

pub type Color = Rgba<f32>;
