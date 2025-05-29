use bevy::prelude::*;



pub const SCORER_STATIC: usize = 1;



pub const STATE_IDLE: usize = 0;
pub const STATE_WALK: usize = 1;
pub const STATE_WORK: usize = 2;

pub mod thinker;
pub use thinker::*;


pub mod thinkerboard;
pub use thinkerboard::*;


















