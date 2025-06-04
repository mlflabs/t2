use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct DebugPrint {
    pub print: bool,
}
