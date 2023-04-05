use bevy::{prelude::*, window::PrimaryWindow};

use crate::AppState;

use self::systems::layout::spawn_menu;

mod components;
mod styles;
mod systems;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_menu.in_schedule(OnEnter(AppState::Menu)))
            .add_startup_system(spawn_camera);
    }
}

/// Menu Camera.
/// This spawns the camera specifically for the menu.
fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
