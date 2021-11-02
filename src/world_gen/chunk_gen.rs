use bevy::{prelude::*, render::mesh::*, render::pipeline::*};

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(spin.system());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Generating world...");
    let mesh = create_mesh(4, 2);
    commands.spawn_bundle(PbrBundle {
        //mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),//create_mesh()),
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.0, 0.3, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
    //.insert(Spin { speed: 4.0 });

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    println!("World generated!");
}

struct Spin {
    speed: f32,
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
    let mut positions: Vec<[f32; 3]> = Vec::new();

    for x in 0..width {
        for y in 0..length {
            positions.push([0.0 + x as f32, 0.0 + y as f32, 0.0]);
            positions.push([1.0 + x as f32, 0.0, 0.0]);
            positions.push([0.0 + x as f32, 1.0 + y as f32, 0.0]);
            positions.push([1.0 + x as f32, 1.0 + y as f32, 0.0]);
        }
    }

    positions
}

fn get_indices(length: i32, width: i32) -> Vec<u32> {
    let nb_squares = (length * width) as u32;
    let mut indices: Vec<u32> = Vec::new();

    for i in 0..nb_squares {
        indices.extend_from_slice(&[
            0 + 4 * i,
            1 + 4 * i,
            2 + 4 * i,
            1 + 4 * i,
            3 + 4 * i,
            2 + 4 * i,
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

fn spin(time: Res<Time>, mut query: Query<(&Spin, &mut Transform)>) {
    for (spin, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_x(spin.speed * time.delta_seconds()));
    }
}
