use bevy::prelude::*;

pub(crate) fn create_sprite_bundle(
    size: Vec2,
    position: Vec3,
    scale: Vec3,
    image: Handle<ColorMaterial>
) -> SpriteBundle {
    // instantiates a sprite_bundle

    let sprite = Sprite::new(size);
    let transform = Transform {
        translation: position,
        scale,
        ..Default::default()
    };

    let sprite_bundle = SpriteBundle {
        transform,
        sprite,
        material: image,
        //visible: Visible { is_visible: false, is_transparent: false },
        ..Default::default()
    };

    return sprite_bundle;
}

// pub(crate) fn create_spritesheet_bundle(
//     position: Vec3,
//     scale: Vec3,
//     texture_atlas_handle: &Handle<TextureAtlas>,
//     sprite_index: u32
// ) -> SpriteSheetBundle {
//     //let sprite = TextureAtlasSprite::new(sprite_index);
//     let transform = Transform {
//         translation: position,
//         scale: scale,
//         ..Default::default()
//     };

//     let mut spritesheet_bundle = SpriteSheetBundle {
//         transform: transform,
//         //sprite: sprite,
//         texture_atlas: texture_atlas_handle.clone(),
//         ..Default::default()
//     };

//     spritesheet_bundle.sprite.index = sprite_index;

//     return spritesheet_bundle;
// }
