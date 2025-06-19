use bevy::prelude::*;
use bevy_turborand::prelude::*;
use std::fmt::Debug;

use super::ThinkScorer;
use super::ThinkBoard;







#[derive(Clone, Debug, Reflect)]
pub struct RandomScorer {
    pub min_value: f32,
    pub max_value: f32,
}

impl RandomScorer {
    pub fn new(min_value: f32, max_value: f32) -> Self {
        RandomScorer { min_value, max_value }
    }
}

impl ThinkScorer for RandomScorer {
    fn score(&self, _board: &ThinkBoard, rng: &mut GlobalRng) -> f32 {

        let value = rng.f32() * (self.max_value - self.min_value) + self.min_value;
        value
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }

    fn clone_box(&self) -> Box<dyn ThinkScorer> {
        Box::new(self.clone())
    }
}
