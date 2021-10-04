use bevy::prelude::*;

pub fn create_sprite_bundle(
    size: Vec2,
    position: Vec3,
    scale: Vec3,
    material_handle: Handle<ColorMaterial>
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
        material: material_handle,
        //visible: Visible { is_visible: false, is_transparent: false },
        ..Default::default()
    };

    return sprite_bundle;
}
