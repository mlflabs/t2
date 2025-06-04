use bevy::log::LogPlugin;
use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (600., 800.).into(),
                        title: "VectorRun".to_string(),
                        resizable: true,
                        position: WindowPosition::At(IVec2::new(10, 550)),
                        //position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                    level: bevy::log::Level::DEBUG,
                    ..Default::default()
                }),
        );
    }
}
