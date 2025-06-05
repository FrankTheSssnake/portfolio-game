use macroquad::prelude::*;

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

    pub fn update(&mut self, delta: f32) {
        let mut direction = vec2(0.0, 0.0);
        if is_key_down(KeyCode::W) {
            direction.y -= 1.0;
            self.direction = PlayerDirection::Up;
        }
        if is_key_down(KeyCode::S) {
            direction.y += 1.0;
            self.direction = PlayerDirection::Down;
        }
        if is_key_down(KeyCode::A) {
            direction.x -= 1.0;
            self.direction = PlayerDirection::Left;
        }
        if is_key_down(KeyCode::D) {
            direction.x += 1.0;
            self.direction = PlayerDirection::Right;
        }
        self.position += direction.normalize_or_zero() * self.speed * delta;
    }

    pub fn draw(&self) {
        let tex = match self.direction {
            PlayerDirection::Down => &self.tex_down,
            PlayerDirection::Up => &self.tex_up,
            PlayerDirection::Left => &self.tex_left,
            PlayerDirection::Right => &self.tex_right,
        };
        draw_texture(tex, self.position.x - 28.0, self.position.y - 28.0, WHITE);
    }
}

