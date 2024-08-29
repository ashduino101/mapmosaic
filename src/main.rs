mod colors;

use std::fs;
use std::io::Read;
use serde_derive::{Serialize, Deserialize};
use flate2::read::GzDecoder;
use image::RgbaImage;
use crate::colors::COLORS;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Vector3i {
    #[serde(rename = "X")]
    x: i32,
    #[serde(rename = "Y")]
    y: i32,
    #[serde(rename = "Z")]
    z: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Banner {
    #[serde(rename = "Color")]
    color: String,
    #[serde(rename = "Name")]
    name: Option<String>,
    #[serde(rename = "Pos")]
    pos: Vector3i
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MapFrame {
    #[serde(rename = "EntityId")]
    entity_id: Option<i32>,
    #[serde(rename = "Pos")]
    pos: Option<Vector3i>,
    #[serde(rename = "Rotation")]
    rotation: Option<i32>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MapData {
    #[serde(rename = "UUIDMost")]
    uuid_most: i64,
    #[serde(rename = "UUIDLeast")]
    uuid_least: i64,
    banners: Vec<Banner>,
    colors: fastnbt::ByteArray,
    dimension: String,
    frames: Vec<MapFrame>,
    locked: bool,
    scale: i8,
    #[serde(rename = "trackingPosition")]
    tracking_position: i8,
    #[serde(rename = "unlimitedTracking")]
    unlimited_tracking: bool,
    #[serde(rename = "xCenter")]
    x_center: i32,
    #[serde(rename = "zCenter")]
    z_center: i32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Map {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    data: MapData,
}

fn main() {
    for f in fs::read_dir("maps").unwrap() {
        let entry = f.unwrap();
        let data = fs::read(entry.path()).unwrap();
        let mut d = GzDecoder::new(&data[..]);
        let mut s = Vec::new();
        d.read_to_end(&mut s).unwrap();

        let nbt: Map = fastnbt::from_bytes(&s[..]).unwrap();

        let mut im = RgbaImage::new(128, 128);
        for (idx, pix) in im.pixels_mut().enumerate() {
            let color_id = nbt.data.colors[idx] as u8;
            let (base_id, shade_id) = if color_id == 0 {
                (0, 0)
            } else {
                (color_id / 4, color_id % 4)
            };
            // println!("{} {} {}", color_id, base_id, shade_id);
            let base_color = COLORS[base_id as usize];
            let shade_mul = match shade_id {
                0 => 180,
                1 => 220,
                2 => 255,
                3 => 135,
                _ => 255
            };
            let [r, g, b, a] = base_color;
            let [r, g, b, a] = [
                ((r as i32) * shade_mul / 255) as u8,
                ((g as i32) * shade_mul / 255) as u8,
                ((b as i32) * shade_mul / 255) as u8,
                a
            ];
            pix.0 = [r, g, b, a];
        }
        im.save(format!("out/{}.png", entry.file_name().to_str().unwrap())).unwrap();
    }
}
