use bevy::prelude::*;

use self::npc::NpcPlugin;

pub mod components;
pub mod npc;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NpcPlugin);
    }
}
