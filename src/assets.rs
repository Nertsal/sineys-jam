use geng::prelude::*;

#[derive(geng::asset::Load)]
pub struct Sfx {
    pub jump: geng::Sound,
    pub spring: geng::Sound,
    pub oi: geng::Sound,
    pub kill_bird: geng::Sound,
    pub shoot: geng::Sound,
}

#[derive(geng::asset::Load)]
pub struct Assets {
    // #[load(options(looped = true))]
    // music: geng::Sound,
    pub sfx: Sfx,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}
