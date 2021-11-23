use crate::world_gen;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct UIControls;

impl Plugin for UIControls {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(world_gen_ui.system());
    }
}

fn world_gen_ui(
    commands: Commands,
    chunk_query: Query<(
        &mut world_gen::gen::ChunkData,
        &mut Transform,
        &Handle<Mesh>,
        Entity,
    )>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    egui_context: ResMut<EguiContext>,
    mut world_gen_data: ResMut<world_gen::WorldGenData>,
) {
    egui::Window::new("World gen params").show(egui_context.ctx(), |ui| {
        let mut size = world_gen_data.size;
        let mut chunk_size = world_gen_data.chunk_size;

        ui.add(egui::Slider::new(&mut size, 1..=200).text("Size"));
        ui.add(egui::Slider::new(&mut chunk_size, 1..=200).text("Chunk size"));

        if ui.button("Regenerate").clicked() {
            world_gen::gen::regenerate(commands, chunk_query, meshes, materials, &world_gen_data);
        }

        if world_gen_data.chunk_size != chunk_size || world_gen_data.size != size {
            world_gen_data.chunk_size = chunk_size;
            world_gen_data.size = size;
        }
    });
}
