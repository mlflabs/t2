use bevy::prelude::*;
use std::fmt::Debug;

use crate::game::ActionStage;

use super::ThinkBoard;

mod print;
mod wait;
mod move_to_target;

pub trait ThinkAction: Send + Sync + 'static {
    // These methods take `&mut self` so they can modify the concrete action instance if needed.
    // They also need the World and EntityCommands to interact with the ECS.
    fn on_enter(&mut self, commands: &mut Commands, board: &ThinkBoard, debug:bool);
    fn on_exit(&mut self,  commands: &mut Commands, board: &ThinkBoard, debug:bool);
    fn on_update(&mut self, delta:f32, commands: &mut Commands, board: &ThinkBoard, debug:bool) -> ActionStage;

    fn get_stage(&self) -> super::ActionStage;


    // You also need a way to debug print this trait object.
    // `Debug` is not automatically implemented for `dyn Trait`, so you need to provide it.
    fn as_debug(&self) -> &dyn Debug;

    // If you need to clone actions (e.g., for `Vec::clone()` on `actions`),
    // you'll need a mechanism for cloning trait objects.
    // This typically involves a `clone_box` method.
    fn clone_box(&self) -> Box<dyn ThinkAction>;
}

impl Clone for Box<dyn ThinkAction> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl Debug for Box<dyn ThinkAction> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_debug().fmt(f)
    }
}
