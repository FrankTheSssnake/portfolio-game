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

#[macroquad::main("Rust Pokémon Portfolio")]
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

