use bevy::{
    // color::palettes::tailwind,
    // math::ops::cos,
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
};
use bevy_simple_subsecond_system::prelude::*;
use bevy_turborand::prelude::*;


mod setup;
use setup::*;

mod utils;
use utils::*;

mod game;

mod helper;

fn main() {
    // Use a custom file path to export registered types in Tiled format
    

    App::new()
        
        .add_plugins(SetupPlugin)
        .add_plugins(RemotePlugin::default())
        .add_plugins(RemoteHttpPlugin::default())
        .add_plugins(SimpleSubsecondPlugin::default())
        .add_plugins(game::GamePlugin)
        .add_plugins(helper::HelperPlugin)
        .add_plugins(RngPlugin::default())
        // .add_plugins(TiledMapPlugin(TiledMapPluginConfig {
        //     tiled_types_export_file: Some(path),
        // }))
        // .add_plugins(bevy_egui::EguiPlugin  { enable_multipass_for_primary_context: true })
        //.add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera (required by Bevy)
    commands.spawn(Camera2d);
}


