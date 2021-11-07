use bevy::{prelude::*};
use bevy_egui::EguiPlugin;
mod world_gen;
mod camera;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(world_gen::WorldGenData {
            size: 4,
            chunk_size: 4,
        })
        .add_plugin(world_gen::gen::WorldGen)
        .add_plugin(camera::Camera)
        //.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default()) 
        .run();
}