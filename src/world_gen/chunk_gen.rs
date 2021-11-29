use bevy::{prelude::*, render::mesh::*, render::pipeline::*};
use noise::{NoiseFn, Perlin};

pub fn create_square_mesh(size: i32) -> Mesh {
    create_mesh(size, size)
}

fn create_mesh(length: i32, width: i32) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let nb_verticies: usize = (length * width * 4) as usize;

    mesh.set_indices(Some(Indices::U32(get_indices(length, width))));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, get_positions(length, width));
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, get_normals(nb_verticies));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, get_uvs(nb_verticies));

    mesh
}

fn get_positions(length: i32, width: i32) -> Vec<[f32; 3]> {
    let perlin = &Perlin::new();
    let mut positions: Vec<[f32; 3]> = Vec::new();

    for x in 0..width {
        for y in 0..length {
            let pos_x = x as f32;
            let pos_y = y as f32;

            positions.push([pos_x, get_height(perlin, pos_x, pos_y), pos_y]);
            positions.push([1.0 + pos_x, get_height(perlin, 1.0 + pos_x, pos_y), pos_y]);
            positions.push([pos_x, get_height(perlin, pos_x, 1.0 + pos_y), 1.0 + pos_y]);
            positions.push([
                1.0 + pos_x,
                get_height(perlin, 1.0 + pos_x, 1.0 + pos_y),
                1.0 + pos_y,
            ]);
        }
    }

    positions
}

fn get_height(perlin: &Perlin, pos_x: f32, pos_y: f32) -> f32 {
    let amplitude = 1000.0;
    let frequency = 10.0;

    let height = perlin.get([
        (1.0 + pos_x as f64) * frequency,
        (0.0 + pos_y as f64) * frequency,
    ]) as f32
        * amplitude;

    println!("{} {},{}", height, pos_x, pos_y);
    height
}

fn get_indices(length: i32, width: i32) -> Vec<u32> {
    let nb_squares = (length * width) as u32;
    let mut indices: Vec<u32> = Vec::new();

    for i in 0..nb_squares {
        indices.extend_from_slice(&[
            2 + 4 * i,
            1 + 4 * i,
            0 + 4 * i,
            2 + 4 * i,
            3 + 4 * i,
            1 + 4 * i,
        ]);
    }

    indices
}

fn get_normals(nb_verticies: usize) -> Vec<[f32; 3]> {
    let mut normals: Vec<[f32; 3]> = Vec::new();

    for _i in 0..nb_verticies {
        normals.push([0.0, 1.0, 0.0]);
    }

    normals
}

fn get_uvs(nb_verticies: usize) -> Vec<[f32; 2]> {
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    for _i in 0..nb_verticies {
        uvs.push([1.0, 1.0]);
    }

    uvs
}
