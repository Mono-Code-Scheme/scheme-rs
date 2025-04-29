use serde::{Deserialize, Serialize};

use crate::schemes::{get_lynx_json, get_panther_json};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorScheme {
    pub panther: Vec<Color>,
    pub lynx: Vec<Color>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Color {
    pub name: String,
    pub hex: String,
    pub rgb: RgbValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RgbValue {
    pub display: String,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub enum ColorId {
    N0,
    N1,
    N2,
    N3,
    N4,
    T0,
    T1,
    T2,
    T3,
    Red,
    Orange,
    Yellow,
    Green,
    NeonGreen,
    Cyan,
    Blue,
    Purple,
    Pink,
}

pub fn get_panther_color(color_id: ColorId) -> Color {
    let palette = get_panther_palette();

    match color_id {
        ColorId::N0 => palette.get(0).unwrap(),
        ColorId::N1 => palette.get(1).unwrap(),
        ColorId::N2 => palette.get(2).unwrap(),
        ColorId::N3 => palette.get(3).unwrap(),
        ColorId::N4 => palette.get(4).unwrap(),
        ColorId::T0 => palette.get(5).unwrap(),
        ColorId::T1 => palette.get(6).unwrap(),
        ColorId::T2 => palette.get(7).unwrap(),
        ColorId::T3 => palette.get(8).unwrap(),
        ColorId::Red => palette.get(9).unwrap(),
        ColorId::Orange => palette.get(10).unwrap(),
        ColorId::Yellow => palette.get(11).unwrap(),
        ColorId::Green => palette.get(12).unwrap(),
        ColorId::NeonGreen => palette.get(13).unwrap(),
        ColorId::Cyan => palette.get(14).unwrap(),
        ColorId::Blue => palette.get(15).unwrap(),
        ColorId::Purple => palette.get(16).unwrap(),
        ColorId::Pink => palette.get(17).unwrap(),
    }
    .to_owned()
}

pub fn get_lynx_color(color_id: ColorId) -> Color {
    let palette = get_lynx_palette();

    match color_id {
        ColorId::N0 => palette.get(0).unwrap(),
        ColorId::N1 => palette.get(1).unwrap(),
        ColorId::N2 => palette.get(2).unwrap(),
        ColorId::N3 => palette.get(3).unwrap(),
        ColorId::N4 => palette.get(4).unwrap(),
        ColorId::T0 => palette.get(5).unwrap(),
        ColorId::T1 => palette.get(6).unwrap(),
        ColorId::T2 => palette.get(7).unwrap(),
        ColorId::T3 => palette.get(8).unwrap(),
        ColorId::Red => palette.get(9).unwrap(),
        ColorId::Orange => palette.get(10).unwrap(),
        ColorId::Yellow => palette.get(11).unwrap(),
        ColorId::Green => palette.get(12).unwrap(),
        ColorId::NeonGreen => palette.get(13).unwrap(),
        ColorId::Cyan => palette.get(14).unwrap(),
        ColorId::Blue => palette.get(15).unwrap(),
        ColorId::Purple => palette.get(16).unwrap(),
        ColorId::Pink => palette.get(17).unwrap(),
    }
    .to_owned()
}

fn get_parsed_colors(json: &str) -> Vec<Color> {
    serde_json::from_str(json).unwrap()
}

pub fn get_panther_palette() -> Vec<Color> {
    get_parsed_colors(&get_panther_json())
}

pub fn get_lynx_palette() -> Vec<Color> {
    get_parsed_colors(&get_lynx_json())
}

pub fn get_monocode_scheme() -> ColorScheme {
    ColorScheme {
        panther: get_panther_palette(),
        lynx: get_lynx_palette(),
    }
}
