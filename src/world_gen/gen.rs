use crate::world_gen;
use bevy::prelude::*;

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut AppBuilder) {
        let now = std::time::Instant::now();

        app.add_startup_system(setup.system())
            .add_plugin(world_gen::ui_controls::UIControls);

        let elapsed_time = now.elapsed();
        println!("World generated in {} seconds.", elapsed_time.as_secs_f32());
    }
}

pub struct ChunkData {
    pos_x: i32,
    pos_y: i32,
    size: i32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_gen_data: ResMut<world_gen::WorldGenData>,
) {
    // Spawn all chunks
    for x in 0..world_gen_data.size {
        for y in 0..world_gen_data.size {
            let chunk_size = world_gen_data.chunk_size;
            commands
                .spawn()
                .insert(ChunkData {
                    pos_x: x,
                    pos_y: y,
                    size: chunk_size,
                })
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(world_gen::chunk_gen::create_square_mesh(chunk_size)),
                    material: if (y + x) % 2 == 0 {
                        materials.add(Color::WHITE.into())
                    } else {
                        materials.add(Color::BLACK.into())
                    },
                    transform: Transform::from_xyz(
                        (x * chunk_size) as f32,
                        0.0,
                        (y * chunk_size) as f32,
                    ),
                    ..Default::default()
                });
        }
    }

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 10.0, 4.0),
        ..Default::default()
    });
}

pub fn regenerate(
    mut commands: Commands,
    mut chunk_query: Query<(&mut ChunkData, &mut Transform, &Handle<Mesh>, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_gen_data: &ResMut<world_gen::WorldGenData>,
) {
    for (_chunk_data, _transform, _mesh, entity) in chunk_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    for x in 0..world_gen_data.size {
        for y in 0..world_gen_data.size {
            let chunk_size = world_gen_data.chunk_size;
            commands
                .spawn()
                .insert(ChunkData {
                    pos_x: x,
                    pos_y: y,
                    size: chunk_size,
                })
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(world_gen::chunk_gen::create_square_mesh(chunk_size)),
                    material: if (y + x) % 2 == 0 {
                        materials.add(Color::WHITE.into())
                    } else {
                        materials.add(Color::BLACK.into())
                    },
                    transform: Transform::from_xyz(
                        (x * chunk_size) as f32,
                        0.0,
                        (y * chunk_size) as f32,
                    ),
                    ..Default::default()
                });
        }
    }
}
