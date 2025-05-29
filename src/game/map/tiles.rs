use bevy::prelude::*;





#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
struct BiomeInfos {
    campType: CampType,
    terrainType: TerrainType,
}

#[derive(Default, Reflect)]
#[reflect(Default)]
enum TerrainType {
    #[default]
    Unknown,
    Forest,
    Plain,
    Road,
    Moutain,
    Desert,
    River,
    ShalowWater,
    DeepWater,
}


#[derive(Default, Reflect)]
#[reflect(Default)]
enum CampType {
    #[default]
    Unknown,
    Settler,
    Village,
    Town,
    City,
}