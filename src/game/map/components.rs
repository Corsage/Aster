use bevy::prelude::Component;

use crate::utils::vectors::Vector2Int;

#[derive(Component)]
pub struct Position {
    pub v: Vector2Int,
}

#[derive(Component)]
pub struct Tile;
