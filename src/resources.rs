pub(crate) struct ScreenSize {
    pub(crate) width: f32,
    pub(crate) height: f32
}
impl ScreenSize {
    pub(crate) fn new(width: f32, height: f32) -> ScreenSize {
        return ScreenSize { width, height };
    }
}

pub(crate) struct WorldMap {
    pub(crate) width: u16,  // 0-65535
    pub(crate) height: u16, // 0-65535
    pub(crate) map: Vec<Cell>
}
impl WorldMap {
    pub(crate) fn new(width: u16, height: u16, map: Vec<Cell>) -> WorldMap {
        return WorldMap { width, height, map };
    }
}

pub(crate) struct Cell {
    pub(crate) image: Image,
    pub(crate) terrain_type_id: u8 // 0-255
}
impl Cell {
    pub(crate) fn new(image: Image, terrain_type_id: u8) -> Cell {
        return Cell { image, terrain_type_id };
    }
}

pub(crate) struct Image {
    pub(crate) texture_atlas_index: u8, // 0-255
    pub(crate) image_index: u8          // 0-255
}
impl Image {
    pub(crate) fn new(texture_atlas_index: u8, image_index: u8) -> Image {
        return Image { texture_atlas_index, image_index };
    }
}