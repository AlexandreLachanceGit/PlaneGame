use bevy::{prelude::*};
mod plugin_test;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(plugin_test::HelloPlugin)
        .run();
}