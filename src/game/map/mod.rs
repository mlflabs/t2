use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};


use bevy_ecs_tiled::prelude::*;
use std::env;

use bevy_simple_subsecond_system::prelude::*;

pub mod path;
pub use path::*;


pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {

        let mut path = env::current_dir().unwrap();
        path.push("../my_tiled_export_file.json");

        app

            .insert_resource(PathData::new(IVec2::new(30, 30), Vec2::new(1.0, 1.0), TileData::new(1.0)))
            .register_type::<PathData>()
            .register_type::<Waypoint>()
            .register_type::<Tile>()
            .register_type::<Town>()
            .register_type::<ResourceType>()
            .register_type::<Resource>()
            .register_type::<Spawner>()
            //.add_plugins(TiledMapPlugin::default())
            .add_plugins(TiledMapPlugin(TiledMapPluginConfig {
                tiled_types_export_file: Some(path),
            }))
            .add_systems(Update, handle_map_event)
            .add_systems(Startup, startup);


        
    }

}






fn handle_map_event(
    mut map_events: EventReader<TiledLayerCreated>,
    map_query: Query<&Name, With<TiledLayerCreated>>,
) {
    for e in map_events.read() {
        if let Ok(name) = map_query.get(e.entity) {
            info!("=> Received TiledMapCreated event for map '{}'", name);
        }
    }
}


#[hot]
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
   
    
    // commands.spawn((
    //     // Only the [TiledMapHandle] component is actually required to spawn a map
    //     TiledMapHandle(asset_server.load("maps/small.tmx")),
    //     // But you can add extra components to change the defaults settings and how
    //     // your map is actually displayed
    //     // TilemapAnchor::Center,
    //     Transform::from_xyz(64.0, 64.0, 0.0),
    // ));


    commands.spawn((    
        // Only the [TiledMapHandle] component is actually required to spawn a map
        TiledMapHandle(asset_server.load("maps/samplemap2.tmx")),
        // But you can add extra components to change the defaults settings and how
        // your map is actually displayed
        TilemapAnchor::Center,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // commands.spawn((
    //     // Only the [TiledMapHandle] component is actually required to spawn a map
    //     TiledMapHandle(asset_server.load("maps/small.tmx")),
    //     // But you can add extra components to change the defaults settings and how
    //     // your map is actually displayed
    //     //TilemapAnchor::TopLeft,
    // ));

    // commands.spawn((
    //     // Only the [TiledMapHandle] component is actually required to spawn a map
    //     TiledMapHandle(asset_server.load("maps/small.tmx")),
    //     // But you can add extra components to change the defaults settings and how
    //     // your map is actually displayed
    //     //TilemapAnchor::TopRight,
    // ));


    let box_red = asset_server.load("box_red.png");
    commands.spawn((
        Name::from("Box Red"),
        Sprite {
            image: box_red.clone(),
            ..default()
        },  
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));


 


}





#[derive(Default, Debug, Reflect)]
#[reflect(Default)]
pub enum TileType {
    #[default]
    Unknown,
    Forest,
    Plain,
    Moutain,
    Desert,
    Road, 
    River,
}


#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
#[component(on_add = my_on_add_hook)]
pub struct Tile {
    pub tile_type: TileType,
    pub custom_cost: f32,
    
}

//////////////////////////////////////////////////////////////////
fn my_on_add_hook(mut world: DeferredWorld, context: HookContext) {
    //println!("Context: {:?}", context);
    let tpos = world.entity(context.entity).get::<TilePos>();
    let tile = world.entity(context.entity).get::<Tile>();
    let mut pos = IVec3::new(0, 0, 0);
    let mut cost: f32 = 1.0;
    if tpos.is_some() {
        let tpos = tpos.unwrap();
        pos.x = tpos.x as i32;
        pos.y = tpos.y as i32;
    }
    if tile.is_some() {
        let tile = tile.unwrap();
        cost = tile.travel_cost();
        //println!("Cost: {}", cost);
    }
    // let p = world.entity(context.entity).get::<ChildOf>();
    // if p.is_some() {
    //     let p = p.unwrap().parent();
    //     let p_name = world.entity(p).get::<Name>().unwrap();
    //     println!("Parent: {}", p_name);
    // }
     
    let mut path_data = world.resource_mut::<PathData>();
    path_data.set_tile(&pos, TileData::new(cost));
    //println!("PathData: {:?}", path_data);
    //world.commands().insert_resource(path_data.clone());
}


impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Tile {
            tile_type,
            custom_cost: 0.0,
        }
    }
    pub fn travel_cost(&self) -> f32 {
        if self.custom_cost > 0.0 {
            return self.custom_cost;
        }
        match self.tile_type {
            TileType::Forest => 2.0,
            TileType::Plain => 1.0,
            TileType::Moutain => 3.0,
            TileType::Desert => 1.5,
            TileType::Road => 0.5,
            TileType::River => 100.0,
            TileType::Unknown => 100.0,
        }
    }
}




#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Waypoint {
    pub id: u32,
    pub pos: Vec3,
    pub n1: u32,
    pub n2: u32,
    pub n3: u32,
    pub n4: u32,
    pub n5: u32,
    pub n6: u32,
    pub n7: u32,
    pub n8: u32,
    pub n9: u32,
    pub n10: u32,
}



#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Town {
    pub id: u32,
    pub waypoint: u32,
    pub name: String,
    pub population: u32,

}

#[derive(Default, Reflect, Debug)]
#[reflect(Default)]
pub enum ResourceType {
    #[default]
    Wood,
    Stone,
    Food,    
}


#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Resource {
    pub id: u32,
    pub waypoint: u32,
    pub resource_type: ResourceType,
    pub quantity: u32,
}

#[derive(Component, Debug, Default, Reflect)]
#[reflect(Component, Default)]
pub struct Spawner {
    pub id: u32,
    pub waypoint: u32,
    pub quantity: u32,

}