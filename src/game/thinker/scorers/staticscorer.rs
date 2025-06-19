use bevy::prelude::*;
use super::ThinkScorer;
use super::ThinkBoard;
use bevy_turborand::prelude::*;
use std::fmt::Debug;


#[derive(Clone, Debug, Reflect)]
pub struct StaticScorer {
    pub value: f32,
}

impl StaticScorer {
    pub fn new(value: f32) -> Self {
        StaticScorer { value }
    }
}

impl ThinkScorer for StaticScorer {
    fn score(&self, _board: &ThinkBoard, _rng: &mut GlobalRng) -> f32 {
        self.value
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn clone_box(&self) -> Box<dyn ThinkScorer> {
        Box::new(self.clone())
    }
}