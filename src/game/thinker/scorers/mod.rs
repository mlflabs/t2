use bevy::prelude::*;
use std::fmt::Debug;
use bevy_turborand::prelude::*;

use super::ThinkBoard;

mod random;
mod staticscorer;

pub trait ThinkScorer: Send + Sync + 'static {
    fn score(&self, board: &ThinkBoard,rng: &mut GlobalRng) -> f32;

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

