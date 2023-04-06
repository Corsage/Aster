use bevy::prelude::*;

use crate::game::components::FacingDirection;

use super::components::{Action, Job, Npc};

pub fn spawn_npc(mut commands: Commands) {
    let test = Npc {
        name: String::from("NPC 1"),
        speed: 1,
        level: 10,
        job: Job::Normal,
        action: Action::Idle,
    };
    commands.spawn(test);
}

pub fn get_npcs(query: Query<&Npc>) {
    for npc in query.iter() {
        println!("NPC: {:?}", npc);
    }
}
