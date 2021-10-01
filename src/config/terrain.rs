use std::collections::HashMap;

struct TerrainType {
    id: u8,         // 0-255
    name: String,
    population: f32, // 0-255
    // production: u8, // 0-255
    // gold: u8,       // 0-255
    // movement_point_cost_walking: u8,
    // movement_point_cost_swimming: u8,
    // movement_point_cost_flying: u8,
    // movement_point_cost_sailing: u8,
    // movement_point_cost_forester: u8,
    // movement_point_cost_mountaineer: u8,
    // movement_point_cost_pathfinding: u8
}

pub(crate) struct TerrainTypes {
    map: HashMap<u8, TerrainType>
}

fn instantiate_terrain_type(id: u8, name: String, population: f32) -> TerrainType {
    let result = TerrainType {
        id,
        name,
        population
    };

    return result;
}

pub(crate) fn load_terrain_types() -> TerrainTypes {
    let mut terrain_types = HashMap::new();

    terrain_types.insert( 1, instantiate_terrain_type( 1, String::from("Ocean Iceberg"), 0.0));
    terrain_types.insert( 2, instantiate_terrain_type( 2, String::from("Ocean"), 0.0));

    terrain_types.insert( 3, instantiate_terrain_type( 3, String::from("Dirt Cold"), 0.25));
    terrain_types.insert( 4, instantiate_terrain_type( 4, String::from("Plains Cold"), 0.5));
    terrain_types.insert( 5, instantiate_terrain_type( 5, String::from("Plains Cold Snow Covered"), 0.0));
    terrain_types.insert( 6, instantiate_terrain_type( 6, String::from("Dirt"), 0.5));
    terrain_types.insert( 7, instantiate_terrain_type( 7, String::from("Plains"), 1.5));
    terrain_types.insert( 8, instantiate_terrain_type( 8, String::from("Forest Broadleaf"), 0.75));
    terrain_types.insert( 9, instantiate_terrain_type( 9, String::from("Desert Dunes"), 0.0));
    terrain_types.insert(10, instantiate_terrain_type(10, String::from("Desert Yellow Cacti Forest"), 0.5));
    terrain_types.insert(11, instantiate_terrain_type(11, String::from("Jungle"), 1.0));

    terrain_types.insert(12, instantiate_terrain_type(12, String::from("Hills Cold"), 0.0));
    terrain_types.insert(13, instantiate_terrain_type(13, String::from("Hills Cold Snow Transition"), 0.0));
    terrain_types.insert(14, instantiate_terrain_type(14, String::from("Hills Cold Snow Covered"), 0.0));
    terrain_types.insert(15, instantiate_terrain_type(15, String::from("Highlands"), 0.0));
    terrain_types.insert(16, instantiate_terrain_type(16, String::from("Hills"), 0.0));
    terrain_types.insert(17, instantiate_terrain_type(17, String::from("Forest Pine"), 0.0));
    terrain_types.insert(18, instantiate_terrain_type(18, String::from("Desert Yellow Hills"), 0.0));
    terrain_types.insert(19, instantiate_terrain_type(19, String::from("Desert Red Hills"), 0.0));
    terrain_types.insert(20, instantiate_terrain_type(20, String::from("Bog"), 0.0));

    terrain_types.insert(21, instantiate_terrain_type(21, String::from("Mountain Snow"), 0.0));
    terrain_types.insert(22, instantiate_terrain_type(22, String::from("Desert Yellow Mesas"), 0.0));
    terrain_types.insert(23, instantiate_terrain_type(23, String::from("Mountain"), 0.0));
    terrain_types.insert(24, instantiate_terrain_type(24, String::from("Desert Red Mountains"), 0.0));

    let result = TerrainTypes { map: terrain_types };

    return result;
}