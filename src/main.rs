use bevy::{
    prelude::*, 
    window::*,
};
use bevy_rapier2d::prelude::*;
//use bevy_inspector_egui::quick::WorldInspectorPlugin;
mod define;
mod facial;
mod fps;
mod stage;
mod state;

fn main() {
    let mut rp = RapierDebugRenderPlugin::default();
    rp.enabled = define::debug::RAPIERDEBUGRENDERPLUGINENABLED;
    let px = match define::debug::ISDEBUG{
        true => {-1700},
        _ => {0}
    };
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin{
                primary_window: Some(Window {
                    title: define::common::TOOLNAME.into(),
                    position: WindowPosition::new(IVec2::new(px, 0)),
                    resolution: (1500.0, 900.0).into(),
                    present_mode: PresentMode::AutoNoVsync, 
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
        .add_plugins(fps::FPSPlugin::new(1))
        .run();
}
