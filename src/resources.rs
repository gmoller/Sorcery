pub struct ScreenSize {
    pub width: f32,
    pub height: f32
}
impl ScreenSize {
    pub fn new(width: f32, height: f32) -> ScreenSize {
        return ScreenSize { width, height };
    }
}

pub struct WorldMap {
    pub width: u16,  // 0-65535
    pub height: u16, // 0-65535
    pub map: Vec<Cell>
}
impl WorldMap {
    pub fn new(width: u16, height: u16, map: Vec<Cell>) -> WorldMap {
        return WorldMap { width, height, map };
    }
}

pub struct Cell {
    pub image: Image,
    pub terrain_type_id: u8 // 0-255
}
impl Cell {
    pub fn new(image: Image, terrain_type_id: u8) -> Cell {
        return Cell { image, terrain_type_id };
    }
}

pub struct Image {
    pub texture_atlas_index: u8, // 0-255
    pub image_index: u8          // 0-255
}
impl Image {
    pub fn new(texture_atlas_index: u8, image_index: u8) -> Image {
        return Image { texture_atlas_index, image_index };
    }
}
