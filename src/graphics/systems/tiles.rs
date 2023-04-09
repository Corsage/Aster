use bevy::prelude::*;

use crate::{
    game::map::components::{Position, Tile},
    graphics::{get_world_position, GraphicsAssets, TILE_SIZE, TILE_Z},
};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>,
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(25);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

        let v = get_world_position(&position, TILE_Z);

        commands.entity(entity).insert(SpriteSheetBundle {
            sprite,
            texture_atlas: assets.sprite_texture.clone(),
            transform: Transform::from_translation(v),
            ..Default::default()
        });
    }
}
