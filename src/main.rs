use bevy::{prelude::*};
mod world_gen;
mod camera;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(world_gen::WorldGenData {
            size: 1,
            chunk_size: 4,
            chunks: Vec::new(),
        })
        .add_plugin(world_gen::gen::WorldGen)
        .add_plugin(camera::Camera)
        //.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default()) 
        .run();
}