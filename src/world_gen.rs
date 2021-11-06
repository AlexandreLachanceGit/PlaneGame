use bevy::prelude::*;

pub mod chunk_gen;
pub mod ui_controls;
pub mod gen;

pub struct WorldGenData {
    pub size: i32,
    pub chunk_size: i32,
    pub chunks: Vec<Entity>,
}
