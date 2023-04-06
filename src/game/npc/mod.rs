use bevy::prelude::*;

use self::systems::*;

pub mod components;
mod systems;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_npc).add_system(get_npcs);
    }
}
