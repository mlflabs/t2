use super::actions::ThinkAction;
use super::scorers::ThinkScorer;
use bevy::prelude::*;
use std::fmt::Debug;

#[derive(Component, Debug)]
pub struct ThinkerExecutingTag;

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum ThinkerStage {
    Init,
    Evaluating,
    AssigningAction,
    LoadAction,
    RunningAction,
    ActionCleanup,
    CleaningUp,
    Finished,
}

impl Default for ThinkerStage {
    fn default() -> Self {
        Self::Evaluating
    }
}



#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum ActionStage {
    Running,
    Independent, //no need to update from thinker,
    Finished,
    Failed,
}

impl Default for ActionStage {
    fn default() -> Self {
        Self::Running
    }
}



#[derive(Clone, Component, Debug, Default)]
pub struct Thinker {
    pub stage: ThinkerStage,
    pub states: Vec<ThinkState>,
    pub state_index: usize,
    pub state_value: f32,
    pub previous_state: usize,
    pub previous_state2: usize,

    pub action_stage: ActionStage,
}

impl Thinker {
    pub fn new() -> Self {
        Thinker {
            stage: ThinkerStage::default(),
            states: Vec::new(),
            state_index: 0,
            state_value: 0.0,
            previous_state: 0,
            previous_state2: 0,
            action_stage: ActionStage::default(),
        }
    }

    pub fn reset(&mut self) {
        self.state_index = 0;
        self.state_value = 0.0;
    }

    pub fn add_state(&mut self, state: ThinkState) -> &mut Self {
        self.states.push(state);
        self
    }

    pub fn build(&mut self) -> Thinker {
        // Here you can add any additional setup or validation if needed
        self.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ThinkState {
    pub scorers: Vec<Box<dyn ThinkScorer>>,
    pub actions: Vec<Box<dyn ThinkAction>>,
    pub current_action: Option<Box<dyn ThinkAction>>,
    pub action_index: usize,
}

impl ThinkState {
    pub fn new() -> Self {
        ThinkState {
            scorers: Vec::new(),
            actions: Vec::new(),
            current_action: None,
            action_index: 0,
        }
    }

    // pub fn next_action(&mut self) -> Option<&dyn ThinkAction> {
    //     if self.action_index < self.actions.len() {
    //         let action = self.actions[self.action_index];
    //         self.action_index += 1;
    //         Some(action)
    //     } else {
    //         None
    //     }
    // }

    pub fn current_action(&self) -> &dyn ThinkAction {
        self.actions[self.action_index].as_ref()
    }

    pub fn current_action_mut(&mut self) -> &mut dyn ThinkAction {
        self.actions[self.action_index].as_mut()
    }

    pub fn set_next_action_cloned(&mut self)-> bool {
        if self.action_index < self.actions.len() {
            // Clone the Boxed action
            let action = self.actions[self.action_index].clone();
            self.action_index += 1;
            self.current_action = Some(action);
            true
        } else {
            self.current_action = None;
            false
        }
    }

    pub fn get_current_action_mut(&mut self) -> Option<&mut dyn ThinkAction> {
        self.current_action.as_mut().map(|b| b.as_mut())
    }

    pub fn reset(&mut self) {
        self.action_index = 0;
    }

    pub fn add_scorer<T: ThinkScorer + 'static>(&mut self, scorer: T) -> &mut Self {
        self.scorers.push(Box::new(scorer));
        self
    }

    pub fn add_action<T: ThinkAction + 'static>(&mut self, action: T) -> &mut Self {
        self.actions.push(Box::new(action));
        self
    }
}
