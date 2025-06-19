
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_simple_subsecond_system::prelude::*;
use std::f32::INFINITY;



pub struct PathPlugin;
impl Plugin for PathPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}


#[derive(Clone, Debug, Reflect)] 
pub struct TileData {
    pub cost: f32,
    
}

impl Default for TileData {
    fn default() -> Self {
        Self { cost: 1.0 }
    }
}

impl TileData {
    pub fn new(cost: f32) -> Self {
        TileData {
            cost,
        }
    }
}


#[derive(Clone, Resource, Reflect, Debug, Default)] 
pub struct PathData {
    pub tile_size: Vec2,
    pub map_size: IVec2,
    pub data: HashMap<IVec3, TileData>,
}

impl PathData {

    pub fn new(map_size: IVec2, tile_size: Vec2, default: TileData) -> Self {
        let mut data = HashMap::new();
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                data.insert(IVec3::new(x, y, 0), default.clone());
            }
        }

        PathData {
            tile_size,
            map_size,
            data,
        }
    }

    pub fn set_tile(&mut self, pos: &IVec3, tile: TileData) {
        self.data.insert(pos.clone(), tile);
    }

    pub fn get_tile(&self, pos: &IVec3) -> Option<&TileData> {
        self.data.get(pos)
    }

    pub fn get_tile_pos(&self, pos: &Vec3) -> IVec3 {
        let x = (pos.x / self.tile_size.x).floor() as i32;
        let y = (pos.y / self.tile_size.y).floor() as i32;
        IVec3::new(x, y, 0)
    }

    pub fn is_valid_pos(&self, pos: &IVec3) -> bool {
        pos.x >= 0 && pos.x < self.map_size.x && pos.y >= 0 && pos.y < self.map_size.y
    }

    pub fn is_valid_tile(&self, pos: &IVec3) -> bool {
        if self.data.contains_key(pos) {
            return true;
        }
        false
    }
  

    pub fn get_neighbors(&self, pos: &IVec3) -> Vec<IVec3> {
        let mut neighbors = Vec::new();
        let deltas = [
            IVec3::new(0, 1, 0), // up
            IVec3::new(0, -1, 0), // down
            IVec3::new(1, 0, 0), //righ
            IVec3::new(-1, 0, 0), //left
            
            IVec3::new(0, 0, 1), //in
            IVec3::new(0, 0, -1), //out

            IVec3::new(1, 1, 0), //up right
            IVec3::new(-1, 1, 0), //up left
            IVec3::new(1, -1, 0), //down right
            IVec3::new(-1, -1, 0), //down left
        ];

        for delta in deltas.iter() {
            let neighbor_pos = pos + delta;

            if self.is_valid_pos(&neighbor_pos) {
                if delta.z != 0 {
                    if self.is_valid_tile(&neighbor_pos) {
                        neighbors.push(neighbor_pos);
                    }
                }
                else {
                    neighbors.push(neighbor_pos);
                }
            }
        }
        neighbors
    }

}






    
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {

}

    
