use crate::world_gen;
use bevy::prelude::*;

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut AppBuilder) {
        let now = std::time::Instant::now();

        app.add_startup_system(setup.system())
            .add_plugin(world_gen::ui_controls::UIControls)
            .add_system(world_gen_data_changed.system());

        let elapsed_time = now.elapsed();
        println!("World generated in {} seconds.", elapsed_time.as_secs_f32());
    }
}

struct ChunkData {}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut world_gen_data: ResMut<world_gen::WorldGenData>,
) {
    // Spawn all chunks
    for _i in 0..world_gen_data.size * world_gen_data.size {
        let chunk_size = world_gen_data.chunk_size;
        world_gen_data.chunks.push(
            commands
                .spawn()
                .insert(ChunkData {})
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(world_gen::chunk_gen::create_square_mesh(chunk_size)),
                    material: materials.add(Color::rgb(0.0, 0.3, 0.6).into()),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                })
                .id(),
        )
    }

    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
}

fn world_gen_data_changed(
    mut chunk_size_query: Query<(&mut ChunkData, &Handle<Mesh>)>,
    //mut world_size_query: Query<(&mut ChunkData, &Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    world_gen_data: Res<world_gen::WorldGenData>,
) {
    if world_gen_data.is_changed() {
        for (_chunk_data, mesh) in chunk_size_query.iter_mut() {
            meshes.set_untracked(
                mesh,
                world_gen::chunk_gen::create_square_mesh(world_gen_data.chunk_size),
            );
        }
    }
}
