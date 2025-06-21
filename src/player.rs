use macroquad::prelude::*;
use crate::map::GameMap;

pub enum PlayerDirection {
    Down,
    Up,
    Left,
    Right,
}

pub struct Player {
    pub position: Vec2,
    pub speed: f32,
    pub direction: PlayerDirection,
    pub tex_down: Texture2D,
    pub tex_up: Texture2D,
    pub tex_left: Texture2D,
    pub tex_right: Texture2D,
}

impl Player {
    pub async fn new(x: f32, y: f32) -> Self {
        let tex_down = load_texture("assets/characters/player_down.png").await.unwrap();
        let tex_up = load_texture("assets/characters/player_up.png").await.unwrap();
        let tex_left = load_texture("assets/characters/player_left.png").await.unwrap();
        let tex_right = load_texture("assets/characters/player_right.png").await.unwrap();

        Self {
            position: vec2(x, y),
            speed: 100.0,
            direction: PlayerDirection::Down,
            tex_down,
            tex_up,
            tex_left,
            tex_right,
        }
    }

    pub fn update(&mut self, map: &GameMap, delta: f32) {
        let mut input_dir = vec2(0.0, 0.0);

        if is_key_down(KeyCode::W) {
            input_dir.y -= 1.0;
            self.direction = PlayerDirection::Up;
        }
        if is_key_down(KeyCode::S) {
            input_dir.y += 1.0;
            self.direction = PlayerDirection::Down;
        }
        if is_key_down(KeyCode::A) {
            input_dir.x -= 1.0;
            self.direction = PlayerDirection::Left;
        }
        if is_key_down(KeyCode::D) {
            input_dir.x += 1.0;
            self.direction = PlayerDirection::Right;
        }

        if input_dir.length() > 0.0 {
            let movement = input_dir.normalize() * self.speed * delta;
            let next_pos = self.position + movement;

            // Only update position if walkable
            if map.is_walkable(next_pos.x, next_pos.y) {
                self.position = next_pos;
            }
        }
    }

    pub fn draw(&self) {
        let texture = match self.direction {
            PlayerDirection::Down => &self.tex_down,
            PlayerDirection::Up => &self.tex_up,
            PlayerDirection::Left => &self.tex_left,
            PlayerDirection::Right => &self.tex_right,
        };

        draw_texture(texture, self.position.x - 28.0, self.position.y - 28.0, WHITE);
    }
}
