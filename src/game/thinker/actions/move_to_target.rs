use bevy::prelude::*;
use std::fmt::Debug;

use crate::game::{ActionStage, ThinkAction};

use super::ThinkBoard;



#[derive(Clone, Default, Debug, Reflect)]
pub struct MoveToTargetAction {
    pub speed: f32,
    pub index: usize,
    // Other state specific to this action
}

impl ThinkAction for MoveToTargetAction {
    fn on_enter(&mut self, _commands: &mut Commands, _board: &super::ThinkBoard, debug:bool) {
        if debug {
            println!("Entering MoveToTargetAction for target: on_enter");
        }
        self.index = 0;
    }

    fn on_exit(&mut self, _commands: &mut Commands, _board: &ThinkBoard, debug:bool) {
        if debug {
            println!("Exiting MoveToTargetAction.");
        }
    }

    fn on_update(&mut self, _delta:f32, _commands: &mut Commands, _board: &ThinkBoard, debug:bool) -> ActionStage {
        if debug {
            println!("Updating MoveToTargetAction, moving towards update");
        }
        self.index += 1;
        return ActionStage::Running;
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn clone_box(&self) -> Box<dyn ThinkAction> {
        Box::new(self.clone()) // Requires MoveToTargetAction to be Clone
    }
    
    fn get_stage(&self) -> super::ActionStage {
        if self.index >= 10 {
            return super::ActionStage::Finished;
        }
        return super::ActionStage::Running;
    }
}


