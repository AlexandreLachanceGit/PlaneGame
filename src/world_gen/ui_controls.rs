use crate::world_gen;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub struct UIControls;

impl Plugin for UIControls {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(world_gen_ui.system());
    }
}

fn world_gen_ui(egui_context: ResMut<EguiContext>, mut world_gen_data: ResMut<world_gen::WorldGenData>) {
    egui::Window::new("World gen params").show(egui_context.ctx(), |ui| {
        let mut chunk_size = world_gen_data.chunk_size;
        ui.add(egui::Slider::new(&mut chunk_size, 1..=200).text("Chunk size"));
        if world_gen_data.chunk_size != chunk_size {
            world_gen_data.chunk_size = chunk_size;
        }
    });
}
