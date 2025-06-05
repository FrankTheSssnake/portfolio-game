use macroquad::prelude::*;
use macroquad::audio::{stop_sound, load_sound, PlaySoundParams, play_sound};
use crate::{player::Player, utils, map::{load_tile_textures, draw_tilemap}};

struct Building {
    pos: Vec2,
    tex: Texture2D,
    link: &'static str,
}

pub async fn island_scene() -> super::super::GameScene {
    let music = load_sound("assets/audio/island.ogg").await.unwrap();
    play_sound(&music, PlaySoundParams { looped: true, volume: 0.7 });
    let tile_textures = load_tile_textures().await;
    let github_tex = load_texture("assets/buildings/github.png").await.unwrap();
    let linkedin_tex = load_texture("assets/buildings/linkedin.png").await.unwrap();
    let youtube_tex = load_texture("assets/buildings/youtube.png").await.unwrap();
    let buildings = [
        Building { pos: vec2(7.0*16.0, 2.0*16.0), tex: github_tex, link: "https://github.com/FrankTheSssnake" },
        Building { pos: vec2(12.0*16.0, 7.0*16.0), tex: linkedin_tex, link: "https://www.linkedin.com/in/aaradhya-bhardwaj-907485319/" },
        Building { pos: vec2(2.0*16.0, 7.0*16.0), tex: youtube_tex, link: "https://music.youtube.com/watch?v=xbnQ6pTROow&si=pJbo_xoJH1tUqk8f" },
    ];
    let mut player = Player::new(screen_width() / 2.0, screen_height() / 2.0).await;
    let mut sea_frame = 0;
    let next_scene;
    loop {
        clear_background(DARKGREEN);
        draw_tilemap(&tile_textures, sea_frame);
        for b in &buildings {
            draw_texture(&b.tex, b.pos.x, b.pos.y, WHITE);
        }
        player.update(get_frame_time());
        player.draw();
        // Check for proximity to buildings
        for b in &buildings {
            if player.position.distance(b.pos + vec2(31.5, 35.5)) < 40.0 {
                draw_text("Press E to enter", b.pos.x, b.pos.y - 10.0, 16.0, YELLOW);
                if is_key_pressed(KeyCode::E) {
                    utils::open_link(b.link);
                }
            }
        }
        // Glitch area (bottom center)
        let glitch_pos = vec2(7.0*16.0, 9.0*16.0);
        draw_text("GLITCH", glitch_pos.x, glitch_pos.y, 16.0, RED);
        if player.position.distance(glitch_pos + vec2(8.0, 8.0)) < 24.0 && is_key_pressed(KeyCode::E) {
            next_scene = super::super::GameScene::Glitch;
            break;
        }
        sea_frame = (sea_frame + 1) % 8;
        next_frame().await;
    }
    stop_sound(&music);
    next_scene
}

