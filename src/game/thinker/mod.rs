/* trunk-ignore-all(rustfmt) */
use bevy::prelude::*;

pub mod thinksystems;
pub use thinksystems::*;

pub mod actions;
pub use actions::*;

pub mod scorers;
pub use scorers::*;

pub mod thinker;
pub use thinker::*;

pub mod thinkboard;
pub use thinkboard::*;

pub struct ThinkerPlugin;

impl Plugin for ThinkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, score_management_system)
            .add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {
    let s = ThinkState::new();
    let m = MoveToTargetAction { speed: 30. };
    let m2 = MoveToTargetAction { speed: 32. };
    let t = Thinker::new()
        .add_state(
            ThinkState::new()
                .add_action(m.clone())
                .add_action(m2.clone())
                .add_scorer(StaticScorer::new(0.1))
                .clone(),
        )
        .build();

    //let tt = t.clone();

    cmd.spawn((
        Sprite::default(),
        Transform::from_translation(Vec3::new(15., 0., 0.)),
        Player { walk_speed: 30. },
        ThinkBoard::default(),
        t,
    ));
}

#[derive(Component, Debug)]
pub struct Player {
    pub walk_speed: f32,
}
