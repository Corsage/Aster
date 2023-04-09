use bevy::prelude::*;
use std::collections::HashMap;

use crate::{utils::vectors::Vector2Int, AppState};

pub mod components;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentBoard>()
            .add_system(systems::spawn_map.in_schedule(OnEnter(AppState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct CurrentBoard {
    pub tiles: HashMap<Vector2Int, Entity>,
}
