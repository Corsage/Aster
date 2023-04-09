use bevy::prelude::*;

use self::systems::{assets, tiles};

pub const TILE_SIZE: f32 = 32.;

mod systems;

#[derive(Resource)]
pub struct GraphicsAssets {
    pub sprite_texture: Handle<TextureAtlas>,
}

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(tiles::spawn_tile_renderer);
    }
}
