use macroquad::prelude::*;
use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams};

use crate::{map::{GameMap, TileTextures}, player::Player, GameScene};

pub async fn island_scene() -> super::super::GameScene {

    let tile_textures: TileTextures = TileTextures::new().await;

    let map: GameMap = GameMap::new();
    let mut player = Player::new(550.0, 550.0).await;

    let mut sea_timer = 0.0;
    let mut sea_frame = 0;

    let next_scene: GameScene = super::super::GameScene::Glitch;

    let music: macroquad::audio::Sound = load_sound("assets/audio/island.ogg").await.unwrap();

    play_sound(&music, PlaySoundParams { looped: true, volume: 0.7 });

    let mut camera = Camera2D {
        target: vec2(player.position.x, player.position.y),
        zoom: vec2(10.0 / screen_width(), 10.0 / screen_height()),
        ..Default::default()
    };

    loop {

        camera.target = vec2(player.position.x, player.position.y);
    
        set_camera(&camera);
    
        clear_background(BLACK);

        let dt = get_frame_time();
        
        sea_timer += dt;
    
        if sea_timer > 0.2 {
            sea_frame = (sea_frame + 1) % 8;
            sea_timer = 0.0;
        }

        map.draw(&tile_textures, sea_frame);
        player.update(&map, get_frame_time());
        player.draw();

        set_default_camera();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }

    stop_sound(&music);

    next_scene
}
