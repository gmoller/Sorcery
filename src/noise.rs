use std::cmp;
use noise::{NoiseFn, Seedable};

pub fn generate_noise(width: u16, height: u16, octaves: Vec<(f64, u32)>) -> Vec<f64> {
    let simplex = noise::OpenSimplex::new();
    simplex.set_seed(1974);

    let mut noise = Vec::new();
    for row in 0..height {
        for column in 0..width {
            let x: f64 = (column as f64 / (width - 1) as f64) - 0.5; // a number between -0.5 and +0.5
            let y: f64 = (row as f64 / (height - 1) as f64) - 0.5; // a number between -0.5 and +0.5

            let mut values = Vec::new();
            for octave in &octaves {
                let frequency = octave.0;
                //let seed = octave.1;
                //simplex.set_seed(seed);
                let value = simplex.get([x * frequency, y * frequency]) * (1.0 / frequency);
                values.push(value);
            }
        
            let sum = sum_up(values);

            //let val = sum.powf(3.0);

            noise.push(sum);
        }
    }

    return noise;
}

fn sum_up(values: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for value in values
    {
        sum += value;
    }

    return sum;
}

pub fn normalize(noise: Vec<f64>) -> Vec<f64> {
    let min = get_min(&noise);
    let max = get_max(&noise);

    let mut noise_normalized = Vec::new();
    let diff = max - min;
    for item in noise {
        let foo = (item - min) / diff;
        noise_normalized.push(foo);
    }
    noise_normalized
}

fn get_min(list: &Vec<f64>) -> f64 {
    let mut min = f64::MAX;
    for item in list {
        if item < &min {
            min = *item;
        };
    }

    return min;
}

fn get_max(list: &Vec<f64>) -> f64 {
    let mut max = f64::MIN;
    for item in list {
        if item > &max {
            max = *item;
        };
    }

    return max;
}

pub fn add_two_vectors(list1: &Vec<f64>, list2: &Vec<f64>) -> Vec<f64> {
    let small = cmp::min(list1.len(), list2.len());

    let mut result = Vec::new();
    for i in 0..small {
        result.push(list1[i] + list2[i]);
    }

    return result;
}

pub fn scale_vector(list: &Vec<f64>, scalar: f64) -> Vec<f64> {
    let mut result = Vec::new();
    for item in list {
        result.push(item * scalar);
    }


    return result;
}
