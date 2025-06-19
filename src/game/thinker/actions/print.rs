use bevy::prelude::*;
use std::fmt::Debug;
use crate::game::{ActionStage, BoardValueTypes, ThinkAction};
use super::ThinkBoard;

use crate::game::BoardKeys;



#[derive(Clone, Default, Debug, Reflect)]
pub struct PrintAction {
    pub msg: String,
    pub key: BoardKeys,
    pub value_type: BoardValueTypes,
}

impl PrintAction {
    fn new(msg: String, key: BoardKeys, value_type: BoardValueTypes) -> Self {
        PrintAction {
            msg,
            key,
            value_type,
        }
    }
}

impl ThinkAction for PrintAction {

    fn on_enter(&mut self, _commands: &mut Commands, board: &ThinkBoard, debug:bool) {
        if self.msg != String::from("") {
            println!("Print Action: {}", self.msg);
        }

        // if self.key != BoardKeys::Unknown {
        //     println!("Print Board Key: {:?}, value: {:?}", 
        //         self.key, board.get_value(self.key, self.value_type));
        // }
    }

    fn on_exit(&mut self, _commands: &mut Commands, _board: &ThinkBoard, _debug:bool) {
    }

    fn on_update(&mut self, _delta:f32, _cmd: &mut Commands, _board: &ThinkBoard, _debug:bool) -> ActionStage {
        return ActionStage::Finished;
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn clone_box(&self) -> Box<dyn ThinkAction> {
        Box::new(self.clone()) // Requires MoveToTargetAction to be Clone
    }
    
    fn get_stage(&self) -> ActionStage {
        return ActionStage::Finished;
    }
}









