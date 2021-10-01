use rand::Rng;

#[allow(dead_code)]
pub(crate) fn create_map_test() -> crate::resources::WorldMap {
    // 0-3: grassland, 4-7: ocean, 8-11: mountain; 12-15: desert; 16-19: forest; 20-23: swamp, 24-27: badlands, 28-31: black, 32-35: hills, 36-39: void
    let mut map = Vec::new();
    for i in 0..6 {
        for j in 0..24 {
            map.push(crate::resources::Cell::new(crate::resources::Image::new(i, j), 0));
        }
    }

    let world_map = crate::resources::WorldMap::new(12, 12, map);

    return world_map;
}

pub(crate) fn create_map(width: u16, height: u16) -> crate::resources::WorldMap {
    let temperature = generate_temperature(width, height);
    let elevation_noise = generate_elevation(width, height);
    let moisture_noise = generate_moisture(width, height, &temperature);

    let mut map = Vec::new();
    for row in 0..height {
        for column in 0..width {
            let index = ((row * width) + column) as i32;
            let elevation_item = elevation_noise[index as usize];
            let moisture_item = moisture_noise[index as usize];
            let temperature_item = temperature[index as usize];

            let cell = determine_terrain(elevation_item, moisture_item, temperature_item);

            map.push(cell);
        }
    }

    let world_map = crate::resources::WorldMap::new(width, height, map);

    return world_map;
}

fn generate_temperature(width: u16, height: u16) -> Vec<f64> {
    let temperature_octaves = vec![(1.0, 1657), (2.0, 2021)];

    let temperature_noise = crate::noise::generate_noise(width, height, temperature_octaves);
    let temperature_values = generate_temperature_values(width, height);

    let mut temperature = Vec::new();
    for i in 0..width * height {
        let val = (temperature_noise[i as usize] / 4.0) + temperature_values[i as usize];
        temperature.push(val);
    }

    let temperature_normalized = crate::noise::normalize(temperature);

    return temperature_normalized;
}

fn generate_temperature_values(width: u16, height: u16) -> Vec<f64> {
    let mut values = Vec::new();

    let mid_height = (height - 1) / 2;

    for row1 in 0..mid_height {
        let value1 = row1 as f64 / mid_height as f64;
        for _ in 0..width {
            values.push(value1);
        }
    }

    for _ in 0..width {
        values.push(1.0);
    }

    for row2 in (mid_height + 1)..height {
        let value2 = (height - 1 - row2) as f64 / mid_height as f64;
        for _ in 0..width {
            values.push(value2);
        }
    }

    return values;
}

fn generate_elevation(width: u16, height: u16) -> Vec<f64> {
    let elevation_octaves = vec![(8.0, 1942), (16.0, 1980), (32.0, 1974), (64.0, 1991)];
    let elevation_noise = crate::noise::generate_noise(width, height, elevation_octaves);
    let noise_normalized = crate::noise::normalize(elevation_noise);

    return noise_normalized;
}

fn generate_moisture(width: u16, height: u16, temperature: &Vec<f64>) -> Vec<f64> {
    let moisture_octaves = vec![(16.0, 1942), (32.0, 1980), (64.0, 1974), (128.0, 1991)];
    let moisture_noise = crate::noise::generate_noise(width, height, moisture_octaves);
    let temperature2 = crate::noise::scale_vector(&temperature, 0.1);
    let moisture_noise2 = crate::noise::add_two_vectors(&moisture_noise, &temperature2);
    let noise_normalized = crate::noise::normalize(moisture_noise2);

    return noise_normalized;
}

fn determine_terrain(elevation: f64, moisture: f64, temperature: f64) -> crate::resources::Cell {
    if  elevation < 0.0 { panic!("elevation is less than zero!"); }
    if  elevation > 1.0 { panic!("elevation is greater than one!"); }
    if  moisture < 0.0 { panic!("moisture is less than zero!"); }
    if  moisture > 1.0 { panic!("moisture is greater than one!"); }
    if  temperature < 0.0 { panic!("temperature is less than zero!"); }
    if  temperature > 1.0 { panic!("temperature is greater than one!"); }

    let mut texture_atlas_index = u8::MAX;
    let mut image_index_min = u8::MAX;
    let mut image_index_max = u8::MAX;
    let mut terrain_type_id = u8::MAX;

    //      elevation     temperature     moisture
    //      (0.35, 0.6) - (0.0, 0.3) - (0.0, 0.3) => Cold::DirtCold (1;0-3)

    let list = vec![
        (0.0,  0.35, 0.0, 0.1, 0.0, 1.0, 1, 42, 45,  1),    // Water - Cold                      => Cold::OceanIceberg
        (0.0,  0.35, 0.1, 1.0, 0.0, 1.0, 0, 32, 35,  2),    // Water - the rest                  => Basic::Ocean

        (0.35, 0.6,  0.0, 0.3, 0.0, 0.3, 1,  0,  3,  3),    // Flat - Cold - Dry                 => Cold::DirtCold
        (0.35, 0.6,  0.0, 0.3, 0.3, 0.7, 1, 42, 46,  4),    // Flat - Cold - Goldilocks          => Cold::PlainsCold
        (0.35, 0.6,  0.0, 0.3, 0.7, 1.0, 1, 47, 51,  5),    // Flat - Cold - Rainy               => Cold::PlainsColdSnowCovered
        (0.35, 0.6,  0.3, 0.7, 0.0, 0.3, 0,  8, 11,  6),    // Flat - Temperate - Dry            => Basic::Dirt
        (0.35, 0.6,  0.3, 0.7, 0.3, 0.7, 0, 36, 39,  7),    // Flat - Temperate - Goldilocks     => Basic::Plains
        (0.35, 0.6,  0.3, 0.7, 0.7, 1.0, 0, 12, 15,  8),    // Flat - Temperate - Rainy          => Basic::ForestBroadleaf
        (0.35, 0.6,  0.7, 1.0, 0.0, 0.3, 0,  4,  7,  9),    // Flat - Hot - Dry                  => Basic::DesertDunes
        (0.35, 0.6,  0.7, 1.0, 0.3, 0.7, 2, 44, 47, 10),    // Flat - Hot - Goldilocks           => Desert::DesertYellowCactiForest
        (0.35, 0.6,  0.7, 1.0, 0.7, 1.0, 4, 12, 15, 11),    // Flat - Hot - Rainy                => Tropics::Jungle

        (0.6,  0.75, 0.0, 0.3, 0.0, 0.3, 1, 19, 23, 12),    // Hills - Cold - Dry                => Cold::HillsCold
        (0.6,  0.75, 0.0, 0.3, 0.3, 0.7, 1, 29, 33, 13),    // Hills - Cold - Goldilocks         => Cold::HillsColdSnowTransition
        (0.6,  0.75, 0.0, 0.3, 0.7, 1.0, 1, 24, 28, 14),    // Hills - Cold - Rainy              => Basic::HillsColdSnowCovered
        (0.6,  0.75, 0.3, 0.7, 0.0, 0.3, 0, 16, 19, 15),    // Hills - Temperate - Dry           => Basic::Highlands
        (0.6,  0.75, 0.3, 0.7, 0.3, 0.7, 0, 20, 23, 16),    // Hills - Temperate - Goldilocks    => Basic::Hills
        (0.6,  0.75, 0.3, 0.7, 0.7, 1.0, 1,  4,  8, 17),    // Hills - Temperate - Rainy         => Basic::ForestPine
        (0.6,  0.75, 0.7, 1.0, 0.0, 0.3, 2, 57, 64, 18),    // Hills - Hot - Dry                 => Desert::DesertYellowHills
        (0.6,  0.75, 0.7, 1.0, 0.3, 0.7, 2, 24, 49, 19),    // Hills - Hot - Goldilocks          => Desert:DesertRedHills
        (0.6,  0.75, 0.7, 1.0, 0.7, 1.0, 4,  0,  3, 20),    // Hills - Hot - Rainy               => Tropics::Bog

        (0.75, 1.0,  0.0, 0.3, 0.0, 0.3, 1, 34, 41, 21),    // Mountain - Cold - Dry             => Cold::MountainSnow
        (0.75, 1.0,  0.0, 0.3, 0.3, 0.7, 1, 34, 41, 21),    // Mountain - Cold - Goldilocks      => Cold::MountainSnow
        (0.75, 1.0,  0.0, 0.3, 0.7, 1.0, 1, 34, 41, 21),    // Mountain - Cold - Rainy           => Cold::MountainSnow
        (0.75, 1.0,  0.3, 0.7, 0.0, 0.3, 2, 71, 78, 22),    // Mountain - Temperate - Dry        => Desert::DesertYellowMesas
        (0.75, 1.0,  0.3, 0.7, 0.3, 0.7, 0, 28, 31, 23),    // Mountain - Temperate - Goldilocks => Basic::Mountain
        (0.75, 1.0,  0.3, 0.7, 0.7, 1.0, 0, 28, 31, 23),    // Mountain - Temperate - Rainy      => Basic::Mountain
        (0.75, 1.0,  0.7, 1.0, 0.0, 0.3, 2, 32, 39, 24),    // Mountain - Hot - Dry              => Desert::DesertRedMountains
        (0.75, 1.0,  0.7, 1.0, 0.3, 0.7, 2, 32, 39, 24),    // Mountain - Hot - Goldilocks       => Desert::DesertRedMountains
        (0.75, 1.0,  0.7, 1.0, 0.7, 1.0, 2, 32, 39, 24),    // Mountain - Hot - Rainy            => Desert::DesertRedMountains
    ];

    for item in list {
        if elevation >= item.0 && elevation <= item.1 &&
           temperature >= item.2 && temperature <= item.3 &&
           moisture >= item.4 && moisture <= item.5 {
            texture_atlas_index = item.6;
            image_index_min = item.7;
            image_index_max = item.8;
            terrain_type_id = item.9;
        }
    }

    if texture_atlas_index == u8::MAX && image_index_min == u8::MAX && image_index_max == u8::MAX {
        panic!("terrain not found for elevation [{}], temperature [{}], moisture [{}]", elevation, temperature, moisture);
    }

    let image_index = rand::thread_rng().gen_range(image_index_min..image_index_max + 1);

    let image = crate::resources::Image::new(texture_atlas_index, image_index);

    return crate::resources::Cell::new(image, terrain_type_id);
}
