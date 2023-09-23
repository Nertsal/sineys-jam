mod assets;
mod game;
mod model;
mod prelude;
mod render;

use geng::prelude::*;

#[derive(clap::Parser)]
struct Opts {
    #[clap(flatten)]
    geng: geng::CliArgs,
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let opts: Opts = clap::Parser::parse();

    let mut geng_opts = geng::ContextOptions::default();
    geng_opts.window.title = "Doodle Shoot".to_string();
    geng_opts.with_cli(&opts.geng);

    Geng::run_with(&geng_opts, |geng| async move {
        let manager = geng.asset_manager();
        let assets = assets::Assets::load(manager).await.unwrap();
        let mut music = assets.music.effect();
        music.set_volume(0.5);
        music.play();
        let game = game::Game::new(&geng, &Rc::new(assets));
        geng.run_state(game).await;
    });
}
