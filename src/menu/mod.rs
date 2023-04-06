use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppState;

use self::systems::layout::spawn_menu;

mod components;
mod styles;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(AppState::Menu)));
    }
}
