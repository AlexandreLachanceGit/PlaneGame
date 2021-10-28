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
    let mesh = create_mesh();
    commands.spawn_bundle(PbrBundle {
        //mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),//create_mesh()),
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    }).insert(Spin {speed: 4.0});

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

fn create_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let indices = vec![0, 1, 2, 1, 3, 2, 2,1,0, 2,3,1];
    let positions = vec![
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    ];

    let nb_verticies: usize = positions.len();
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, get_normals(nb_verticies));
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, get_uvs(nb_verticies));
    mesh
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
