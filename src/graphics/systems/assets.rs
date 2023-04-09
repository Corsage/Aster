use bevy::prelude::*;

use crate::{graphics::GraphicsAssets, splash::AssetList};

const TILE_PATH: &str = "sprites\\map\\tilemap.png";

pub fn load_assets(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut atlas: ResMut<Assets<TextureAtlas>>,
    mut assets: ResMut<AssetList>,
) {
    let texture = server.load(TILE_PATH);
    assets.0.push(texture.clone_untyped());

    let map = TextureAtlas::from_grid(
        texture,
        Vec2::splat(16.),
        17,
        8,
        Some(Vec2::splat(1.)),
        None, // Some(Vec2::splat(1.)),
    );

    let handle = atlas.add(map);
    commands.insert_resource(GraphicsAssets {
        sprite_texture: handle,
    });
}
