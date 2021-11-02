use bevy::prelude::*;
mod chunk_gen;
mod ui_controls;

pub struct WorldGen;

impl Plugin for WorldGen {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(chunk_gen::ChunkGen)
        .add_plugin(ui_controls::UIControls);
    }
}