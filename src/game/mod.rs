use bevy::prelude::*;



pub mod thinker;
pub use thinker::*;

// pub mod core;
// pub use core::*;

// pub mod helpers;
// pub use helpers::*;

// pub mod map;
// pub use map::*;

// pub mod settings;
// pub use settings::*;

pub mod sprites;
pub use sprites::*;


pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(SpritesPlugin);
            //.add_plugins((ThinkerPlugin, VoxelsPlugins));

            //.register_type::<Thirst>()
            //.register_type::<ThinkerBuilder>()
            //.add_systems(Startup, setup_camera)
            // .add_systems(
            //     Update,
            //     ((
            //         camera_move
            //     ))
            //)
            // .add_plugins((
            //     PlayerPlugin,
            //     MyCameraPlugin,
            //     BrainPlugin,
            //     NpcPlugin,
            //     MapPlugin,
            //     LittleBrainPlugin,
            //     EditorPlugin::default(),
            //     PhysicsPlugins::default(),
            //     PhysicsDebugPlugin::default()
            // ));
            
    }
}
 