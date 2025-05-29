use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;

use crate::game::thinker::{
    Score, 
    ThinkerTag,
    Scorer, 
    Scorers, 
    SCORER_STATIC, 
    STATE_IDLE, 
    ScorerComparisons};

pub struct SpritesPlugin;

impl Plugin for SpritesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AsepriteUltraPlugin)
           .add_systems(Startup, setup);
    }
}



fn setup(mut cmd: Commands, server: Res<AssetServer>) {
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
        Player {
            walk_speed: 30.,
        },
        ThinkerTag,
        Score::default(),
        Scorers {
            scorers: vec![
                Scorer {
                    scorer_type: SCORER_STATIC,
                    comparison: ScorerComparisons::Contains,
                    value: 0.1,
                    score: 0.0,
                    state: STATE_IDLE,
                }


            ],
        },
        
    ));
}


#[derive(Component, Debug)]
pub struct Player {
    pub walk_speed: f32,
}
