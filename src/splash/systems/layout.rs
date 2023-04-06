use bevy::prelude::*;

use crate::{
    splash::{components::Splash, styles::SPLASH_STYLE},
    ui::styles::get_title_style,
};

pub fn spawn_splash(mut commands: Commands, asset_server: Res<AssetServer>) {
    build_splash(&mut commands, &asset_server);
}

pub fn despawn_splash(mut commands: Commands, query: Query<Entity, With<Splash>>) {
    if let Ok(splash) = query.get_single() {
        commands.entity(splash).despawn_recursive();
    }
}

fn build_splash(commands: &mut Commands, server: &Res<AssetServer>) -> Entity {
    let entity = commands
        .spawn((
            NodeBundle {
                style: SPLASH_STYLE,
                background_color: Color::BLACK.into(),
                ..default()
            },
            Splash {},
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection::new("Loading...", get_title_style(&server))],
                    alignment: TextAlignment::Center,
                    ..default()
                },
                ..default()
            });
        })
        .id();

    entity
}
