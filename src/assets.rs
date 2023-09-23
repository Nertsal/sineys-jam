use geng::prelude::*;
use geng_utils::gif::GifFrame;

#[derive(geng::asset::Load)]
pub struct Sfx {
    pub jump: geng::Sound,
    pub spring: geng::Sound,
    pub oi: geng::Sound,
    pub kill_bird: geng::Sound,
    pub shoot: geng::Sound,
}

#[derive(geng::asset::Load)]
pub struct Sprites {
    pub background: ugli::Texture,
    #[load(load_with = "load_gif(&manager, &base_path.join(\"doodle.gif\"))")]
    pub doodle: Vec<GifFrame>,
    #[load(load_with = "load_gif(&manager, &base_path.join(\"bird.gif\"))")]
    pub bird: Vec<GifFrame>,
    pub bullet: ugli::Texture,
    pub cloud: ugli::Texture,
    pub spring: ugli::Texture,
}

#[derive(geng::asset::Load)]
pub struct Assets {
    pub sprites: Sprites,
    #[load(ext = "mp3", options(looped = "true"))]
    pub music: geng::Sound,
    pub sfx: Sfx,
}

impl Assets {
    pub async fn load(manager: &geng::asset::Manager) -> anyhow::Result<Self> {
        geng::asset::Load::load(manager, &run_dir().join("assets"), &())
            .await
            .context("failed to load assets")
    }
}

fn load_gif(
    manager: &geng::asset::Manager,
    path: &std::path::Path,
) -> geng::asset::Future<Vec<GifFrame>> {
    let manager = manager.clone();
    let path = path.to_owned();
    async move {
        geng_utils::gif::load_gif(
            &manager,
            &path,
            geng_utils::gif::GifOptions {
                frame: geng::asset::TextureOptions {
                    // filter: ugli::Filter::Nearest,
                    ..Default::default()
                },
            },
        )
        .await
    }
    .boxed_local()
}
