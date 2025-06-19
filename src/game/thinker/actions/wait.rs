use bevy::prelude::*;
use std::fmt::Debug;
use crate::game::ActionStage;

use super::ThinkAction; 
use super::ThinkBoard;






#[derive(Clone, Default, Debug, Reflect)]
pub struct WaitAction {
    pub time_sec: f32,
    pub current_time: f32,
}

impl WaitAction {
    fn new(time_sec: f32) -> Self {
        WaitAction {
            time_sec,
            current_time: 0.0,
        }
    }
}

impl ThinkAction for WaitAction {
    fn on_enter(&mut self, _commands: &mut Commands, _board: &ThinkBoard, debug:bool) {
        self.current_time = 0.0;
    }

    fn on_exit(&mut self, _commands: &mut Commands, _board: &ThinkBoard, debug:bool) {
    }

    fn on_update(&mut self, delta:f32, cmd: &mut Commands, _board: &ThinkBoard, debug:bool) -> ActionStage {
        self.current_time += delta;
        if self.current_time >= self.time_sec {
            return ActionStage::Finished;
        }
        return ActionStage::Running;
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn clone_box(&self) -> Box<dyn ThinkAction> {
        Box::new(self.clone()) // Requires MoveToTargetAction to be Clone
    }
    
    fn get_stage(&self) -> ActionStage {
        if self.current_time >= self.time_sec {
            return ActionStage::Finished;
        }
        return ActionStage::Running;
    }
}









