use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use super::thinker::*;
// use crate::game::thinker::{
//     Scorer, ScorerComparisons, Scorers, Thinker, SCORER_STATIC, STATE_IDLE,
// };

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AsepriteUltraPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut cmd: Commands, server: Res<AssetServer>) {

    let thinker  = Thinker::default()
        .add_state(
            ThinkState::new()
                // .add_action(MoveToTargetAction::default())
                // .add_action(TestAction::default())
                // .add_scorer(StaticScorer::new(0.1))
                .clone(),
        );

    cmd.spawn((
        AseAnimation {
            animation: Animation::tag("Walk")
                .with_repeat(AnimationRepeat::Loop)
                .with_direction(AnimationDirection::Forward)
                .with_speed(0.4),
            aseprite: server.load("sprites/Human-Worker-Red.ase"),
        },
        Sprite::default(),
        Transform::from_translation(Vec3::new(15., 0., 0.)),
        Player { walk_speed: 30. },
        // Thinker::default(),
        // Scorers {
        //     scorers: vec![Scorer {
        //         scorer_type: SCORER_STATIC,
        //         comparison: ScorerComparisons::Contains,
        //         value: 0.1,
        //         success_score: 1.0,
        //         failure_score: 0.0,
        //         state: STATE_IDLE,
        //         last_score: 0.0,
        //     }],
        // },
    ));
}

#[derive(Component, Debug)]
pub struct Player {
    pub walk_speed: f32,
}
