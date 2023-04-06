use bevy::prelude::*;

use crate::AppState;

use self::systems::*;

pub mod components;
mod systems;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(get_npcs.in_schedule(OnEnter(AppState::Menu)));
    }
}
