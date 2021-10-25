use bevy::{prelude::*, render::mesh::*, render::pipeline::*};

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(movement_system.system());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Generating world...");
    let triangle_mesh = create_mesh();
    commands.spawn_bundle(PbrBundle {
        //mesh: meshes.add(Mesh::from(shape::Cube{size: 1.0})),//create_mesh()),
        mesh: meshes.add(triangle_mesh),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(-2.0, 0.5, 0.0),
        ..Default::default()
    }).insert(Spin { speed: 2.0 });

    // cube
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        }).insert(Spin { speed: 2.0 });
        
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(2.0, 0.5, 0.0),
            ..Default::default()
        }).insert(Spin { speed: 4.0 });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    println!("World generated!");
}

struct Spin {
    speed: f32,
}

fn create_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices = vec![[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]];

    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, vec![1.0, 1.0, 1.0]);
    mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[1.0, 0.0], [0.0, 1.0], [1.0, 1.0]],
    );
    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2])));
    mesh
}

fn movement_system(time: Res<Time>, mut query: Query<(&Spin, &mut Transform)>) {
    /*if let Ok((_controller, mut transform)) = query.for_each_mut() {
        transform.rotate(Quat::from_rotation_z(2.0 * time.delta_seconds()));
    }*/
    for (spin, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_z(spin.speed * time.delta_seconds()));
    }
}
/*
fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    if let Ok((paddle, mut transform)) = query.single_mut() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction * paddle.speed;
        // bound the paddle within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
    }
}*/
