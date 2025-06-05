use macroquad::prelude::*;
use macroquad::audio::{play_sound, stop_sound, load_sound, PlaySoundParams};

pub async fn glitch_scene() -> super::super::GameScene {
    let music = load_sound("assets/audio/glitch.ogg").await.unwrap();
    play_sound(&music, PlaySoundParams { looped: true, volume: 0.7 });
    clear_background(RED);
    draw_text("Errno: unexpected NULL in dream zone", 100.0, 200.0, 30.0, WHITE);
    draw_text("Returning to island...", 100.0, 240.0, 20.0, LIGHTGRAY);

    for _ in 0..120 {
        next_frame().await;
    }
    stop_sound(&music);
    super::super::GameScene::Island
}

