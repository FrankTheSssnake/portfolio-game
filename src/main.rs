mod player;
mod map;
mod utils;
mod scenes;

use macroquad::prelude::*;
use scenes::{island::island_scene, glitch::glitch_scene};

#[derive(PartialEq)]
enum GameScene {
    Island,
    Glitch,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene = GameScene::Island;

    loop {
        clear_background(BLACK);

        scene = match scene {
            GameScene::Island => island_scene().await,
            GameScene::Glitch => glitch_scene().await,
        };

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Portfolio".to_string(),
        fullscreen: true,
        window_width: 1920,
        window_height: 1200,
        ..Default::default()
    }
}
