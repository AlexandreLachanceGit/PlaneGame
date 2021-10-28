use bevy::{prelude::*};
mod world_gen;
mod camera;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(world_gen::WorldGen)
        .add_plugin(camera::Camera)
        //.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default()) 
        .run();
}