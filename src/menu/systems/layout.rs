use bevy::prelude::*;

use crate::{
    menu::{components::Menu, styles::*},
    ui::styles::get_title_style,
};

pub fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_menu(&mut commands, &asset_server);
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Menu>>) {
    if let Ok(menu) = query.get_single() {
        commands.entity(menu).despawn_recursive();
    }
}

fn build_menu(commands: &mut Commands, server: &Res<AssetServer>) -> Entity {
    let entity = commands
        .spawn((
            NodeBundle {
                style: MENU_STYLE,
                background_color: Color::RED.into(),
                ..default()
            },
            Menu {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Aster", get_title_style(&server))],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    entity
}
