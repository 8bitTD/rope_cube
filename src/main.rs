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
        _ => {0}
    };
    let res = match define::value::ISDEBUG{
        true => (1500.0, 900.0),
        _ => (1920.0, 1080.0)
    };
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window {
                    title: define::common::TOOLNAME.into(),
                    position: WindowPosition::new(IVec2::new(px, 0)),
                    resolution: (res.0, res.1).into(),
                    transparent: true,
                    decorations: true,
                    present_mode: PresentMode::AutoNoVsync, 
                    //present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    fit_canvas_to_parent: true,
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
