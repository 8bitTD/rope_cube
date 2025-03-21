use bevy::{
    prelude::*, 
    window::*,
};
use bevy_rapier2d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod define;
mod state;
mod stage;

fn main() {
    let mut rp = RapierDebugRenderPlugin::default();
    rp.enabled = define::value::RAPIERDEBUGRENDERPLUGINENABLED;
    let px = match define::value::ISDEBUG{
        true => {-1700},
        _ => {50}
    };
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window {
                    title: define::common::TOOLNAME.into(),
                    position: WindowPosition::new(IVec2::new(px, 10)),
                    resolution: (1500.0, 900.0).into(),
                    transparent: true,
                    decorations: true,
                    //mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    present_mode: PresentMode::AutoNoVsync,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                exit_condition: bevy::window::ExitCondition::OnAllClosed,
                close_when_requested: true,
                ..default()
            },
        ).set(bevy::log::LogPlugin{
            level: bevy::log::Level::WARN,
            ..default()
        }).set(AssetPlugin {
	        meta_check: bevy::asset::AssetMetaCheck::Never,
	        ..default()
	    }))
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0),
            rp,
        ))
        
        //.add_plugins(WorldInspectorPlugin::new())
        .add_plugins(state::StatePlugin)
        .run();
}
