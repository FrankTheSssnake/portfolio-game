use std::fs::File;
use std::io::Read;

use std::collections::HashMap;

use macroquad::prelude::*;
use serde_json::Value;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Building {
    GitHub,
    YouTube,
    LinkedIn,
}

impl Building {
    pub fn get_texture<'a>(&self, tile_textures: &'a TileTextures) -> &'a Texture2D {
        match self {
            Building::GitHub => &tile_textures.github,
            Building::YouTube => &tile_textures.youtube,
            Building::LinkedIn => &tile_textures.linkedin,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TileType {
    Building(Building),
    Shore,
    Sea,
    Path,
    Grass,
}

impl TileType {
    pub fn from_tile(img: &String) -> Self {
        if img.contains("grass") {
            TileType::Grass
        } else if img.contains("shore") {
            TileType::Shore
        } else if img.contains("sea") {
            TileType::Sea
        } else if img.contains("path") {
            TileType::Path
        } else if img.contains("github") {
            TileType::Building(Building::GitHub)
        } else if img.contains("youtube") {
            TileType::Building(Building::YouTube)
        } else if img.contains("linkedin") {
            TileType::Building(Building::LinkedIn)
        } else {
            TileType::Sea // fallback default
        }
    }
}

#[derive(Debug)]
struct Map {
    pub height: usize,
    pub width: usize,
    pub data: Vec<usize>,
    pub tileset: (usize, Vec<(usize, String)>),
}

impl Map {
    fn new(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let v: Value = serde_json::from_str(&buffer).unwrap();

        let height = v["height"].as_u64().unwrap() as usize;
        let width = v["width"].as_u64().unwrap() as usize;

        let data = v["layers"][0]["data"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| x.as_u64().unwrap() as usize)
            .collect();

        let firstgid = v["tilesets"][0]["firstgid"].as_u64().unwrap() as usize;
        let tileset_array = v["tilesets"][0]["tiles"].as_array().unwrap();

        let tileset: Vec<(usize, String)> = tileset_array
            .iter()
            .map(|x|
                (
                    x["id"].as_u64().unwrap() as usize,
                    x["image"].as_str().unwrap().to_string(),
                )
            )
            .collect();

        Map {
            height,
            width,
            data,
            tileset: (firstgid, tileset),
        }
    }
}


#[derive(Debug)]
pub struct TileTextures {
    pub grass: Texture2D,
    pub path: Texture2D,
    pub shore: Texture2D,
    pub sea: [Texture2D; 8],
    pub github: Texture2D,
    pub youtube: Texture2D,
    pub linkedin: Texture2D,
}

impl TileTextures {
    pub async fn new() -> Self {
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

        let github = load_texture("assets/buildings/github.png").await.unwrap();
        let youtube = load_texture("assets/buildings/youtube.png").await.unwrap();
        let linkedin = load_texture("assets/buildings/linkedin.png").await.unwrap();

        Self { grass, path, shore, sea, github, youtube, linkedin }
    }

    pub fn get_texture(&self, tile_type: &TileType) -> &Texture2D {
        match tile_type {
            TileType::Grass => &self.grass,
            TileType::Path => &self.path,
            TileType::Shore => &self.shore,
            TileType::Sea => &self.sea[0],
            TileType::Building(building) => building.get_texture(self),
        }
    }
}


#[derive(Debug)]
pub struct GameMap {
    pub height: usize,
    pub width: usize,
    pub data: Vec<TileType>,
}

impl GameMap {

    pub fn new() -> Self {
        let path = "assets/portfolio.tmj";
        let map: Map = Map::new(path);

        let (firstgid, tileset) = &map.tileset;

        let id_to_type: HashMap<usize, TileType> = tileset
            .iter()
            .map(|tile| (tile.0, TileType::from_tile(&tile.1)))
            .collect();

        let mut data = Vec::with_capacity(map.data.len());

        for raw_gid in map.data.iter() {
            if *raw_gid < *firstgid {
                data.push(TileType::Sea); // Empty/default tile
                continue;
            }

            let tile_id = raw_gid - firstgid;
            let tile_type = id_to_type.get(&tile_id).cloned().unwrap_or(TileType::Grass);
            data.push(tile_type);
        }

        GameMap {
            height: map.height,
            width: map.width,
            data,
        }
    }

    pub fn is_walkable(&self, x: f32, y: f32) -> bool {
        let tile_size = 16.0;
        let tile_x = (x / tile_size).floor() as isize;
        let tile_y = (y / tile_size).floor() as isize;

        let index = tile_y as usize * self.width + tile_x as usize;

        match self.data.get(index) {
            Some(TileType::Sea) => {
                println!("Not walkable! Tile: Sea");
                false
            },
            Some(TileType::Building(_)) => {
                println!("Not walkable! Tile: Building");
                false
            },
            Some(t) => {
                println!("Walkable! Tile: {:?}", t);
                true
            },
            None => {
                println!("Not walkable! Tile: None");
                false
            }
        }
    }

    pub fn draw(&self, tile_textures: &TileTextures, sea_frame: usize) {
        let tile_size = 16.0; // Adjust based on your texture size

        for y in 0..self.height {
            for x in 0..self.width {
                let index = y * self.width + x;
                let tile_type = &self.data[index];

                let texture = match tile_type {
                    TileType::Sea => {
                        &tile_textures.sea[sea_frame % tile_textures.sea.len()]
                    }
                    _ => tile_textures.get_texture(tile_type),
                };

                draw_texture(texture, x as f32 * tile_size, y as f32 * tile_size, WHITE);
            }
        }
    }
}
