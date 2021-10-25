use bevy::{prelude::*};
mod world_gen;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(world_gen::WorldGen)
        .run();
}