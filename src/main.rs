use std::env;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;


mod setup;
use setup::*;

mod game;

mod helper;


fn main() {

    // Use a custom file path to export registered types in Tiled format
    let mut path = env::current_dir().unwrap();
    path.push("../my_tiled_export_file.json");


    App::new()

        .register_type::<BiomeInfos>()

        .add_plugins(SetupPlugin) 
        .add_plugins(game::GamePlugin)


        .add_plugins(helper::HelperPlugin)
        .add_plugins(TiledMapPlugin::default())
        // .add_plugins(TiledMapPlugin(TiledMapPluginConfig {
        //     tiled_types_export_file: Some(path),
        // }))

        .add_systems(Startup, startup)

        
    .run();
}
 
 

 

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn a 2D camera (required by Bevy)
    commands.spawn(Camera2d);

    // Load a map then spawn it
    commands.spawn((
        // Only the [TiledMapHandle] component is actually required to spawn a map
        TiledMapHandle(asset_server.load("maps/samplemap2.tmx")),
        // But you can add extra components to change the defaults settings and how
        // your map is actually displayed
        TilemapAnchor::Center,
    ));
}  


#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
struct BiomeInfos {
    block_line_of_sight: bool,
    ty: BiomeType,
}

#[derive(Default, Reflect)]
#[reflect(Default)]
enum BiomeType {
    #[default]
    Unknown,
    Forest,
    Plain,
    Moutain,
    Desert,
}
