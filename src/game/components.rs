use bevy::prelude::*;

pub enum FacingDirection {
    Left,
    Right,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: u8,
}
