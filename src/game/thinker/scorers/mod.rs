use bevy::prelude::*;
use std::fmt::Debug;

use super::ThinkBoard;

pub trait ThinkScorer: Send + Sync + 'static {
    fn score(&self, board: &ThinkBoard) -> f32;

    fn as_debug(&self) -> &dyn Debug;
    fn clone_box(&self) -> Box<dyn ThinkScorer>;
}

impl Debug for Box<dyn ThinkScorer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_debug().fmt(f)
    }
}

impl Clone for Box<dyn ThinkScorer> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

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
    fn score(&self, _board: &ThinkBoard) -> f32 {
        self.value
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn clone_box(&self) -> Box<dyn ThinkScorer> {
        Box::new(self.clone())
    }
}
