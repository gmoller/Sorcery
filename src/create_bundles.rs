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
