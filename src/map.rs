use macroquad::prelude::*;

const MAP_WIDTH: usize = 15;
const MAP_HEIGHT: usize = 10;

// 0: grass, 1: path, 2: shore, 3+: sea frames
const MAP: [[u8; MAP_WIDTH]; MAP_HEIGHT] = [
    [3,3,3,3,2,2,2,2,2,2,2,3,3,3,3],
    [3,3,3,2,0,0,0,0,0,0,0,2,3,3,3],
    [3,3,2,0,0,0,1,1,1,0,0,0,2,3,3],
    [3,2,0,0,0,1,1,1,1,1,0,0,0,2,3],
    [2,0,0,0,1,1,1,1,1,1,1,0,0,0,2],
    [2,0,0,0,1,1,1,1,1,1,1,0,0,0,2],
    [3,2,0,0,0,1,1,1,1,1,0,0,0,2,3],
    [3,3,2,0,0,0,1,1,1,0,0,0,2,3,3],
    [3,3,3,2,0,0,0,0,0,0,0,2,3,3,3],
    [3,3,3,3,2,2,2,2,2,2,2,3,3,3,3],
];

pub struct TileTextures {
    pub grass: Texture2D,
    pub path: Texture2D,
    pub shore: Texture2D,
    pub sea: [Texture2D; 8],
}

pub async fn load_tile_textures() -> TileTextures {
    let grass = load_texture("assets/tiles/grass.png").await.unwrap();
    let path = load_texture("assets/tiles/path.png").await.unwrap();
    let shore = load_texture("assets/tiles/shore.png").await.unwrap();
    let sea = [
        load_texture("assets/tiles/sea_1.png").await.unwrap(),
        load_texture("assets/tiles/sea_2.png").await.unwrap(),
        load_texture("assets/tiles/sea_3.png").await.unwrap(),
        load_texture("assets/tiles/sea_4.png").await.unwrap(),
        load_texture("assets/tiles/sea_5.png").await.unwrap(),
        load_texture("assets/tiles/sea_6.png").await.unwrap(),
        load_texture("assets/tiles/sea_7.png").await.unwrap(),
        load_texture("assets/tiles/sea_8.png").await.unwrap(),
    ];
    TileTextures { grass, path, shore, sea }
}

pub fn draw_tilemap(tile_textures: &TileTextures, sea_frame: usize) {
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let tile = MAP[y][x];
            let pos_x = x as f32 * 16.0;
            let pos_y = y as f32 * 16.0;
            match tile {
                0 => draw_texture(&tile_textures.grass, pos_x, pos_y, WHITE),
                1 => draw_texture(&tile_textures.path, pos_x, pos_y, WHITE),
                2 => draw_texture(&tile_textures.shore, pos_x, pos_y, WHITE),
                3..=10 => draw_texture(&tile_textures.sea[sea_frame % 8], pos_x, pos_y, WHITE),
                _ => {},
            }
        }
    }
}

