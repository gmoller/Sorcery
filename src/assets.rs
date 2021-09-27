use std::collections::HashMap;
use bevy::prelude::*;

pub(crate) fn load_images(asset_server: &Res<AssetServer>, materials: &mut ResMut<Assets<ColorMaterial>>) -> HashMap<i32, Vec<Handle<ColorMaterial>>> {
    // loads assets using the Bevy asset_server

    let folders = [ ( "TerrainTiles/Basic", 0 ),
                                   ( "TerrainTiles/Cold", 1 ),
                                   ( "TerrainTiles/Desert", 2 ),
                                   ( "TerrainTiles/Medieval-Fantasy", 3 ),
                                   ( "TerrainTiles/Tropics-Wetlands", 4 ),
                                   ( "TerrainTiles/Volcanic Wastes", 5 ),
                                   ( "Units", 6 ),
                                 ];

    let mut texture_atlases = HashMap::new();

    for (folder, key) in folders {
        let handles = load_images_from_asset_server(folder, asset_server, materials);
        texture_atlases.insert(key, handles);
    }

    return texture_atlases;
}

fn load_images_from_asset_server(folder: &str, asset_server: &Res<AssetServer>, materials: &mut ResMut<Assets<ColorMaterial>>) -> Vec<Handle<ColorMaterial>> {
    // reads a manifest file in folder and loads all images described in the manifest file

    let path = format!("assets/images/{}/manifest.txt", folder);
    let msg = format!("Unable to read file: {}", path);
    let data = std::fs::read_to_string(path).expect(&msg);
    let lines = data.lines();
    let filenames: Vec<&str> = lines.collect();

    let mut handles: Vec<Handle<ColorMaterial>> = Vec::new();
    for filename in filenames {
        let path = format!("images/{}/{}", folder, filename);
        let texture_handle = asset_server.load(&path[..]);
        let material = materials.add(texture_handle.into());
        handles.push(material);
    }
    
    return handles;
}
