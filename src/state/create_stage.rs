use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use super::super::state::*;

pub fn setup_asset_stage(
    mut commands: Commands,
    mut app: ResMut<MyApp>, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    commands.insert_resource(ClearColor(Color::srgb(0.175, 0.175, 0.175)));
    commands.spawn((//カメラ
        Camera2d::default(),
        ReleaseResource
    ));
}

pub fn camera(
    mut camera: Single<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
    accumulated_mouse_scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
    //app: Res<MyApp>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    let ds = time.delta_secs();
    //let sa = (player.translation - camera.0.translation) * ds * system::FPS*0.05;
    //camera.0.translation += sa;
    if accumulated_mouse_scroll.delta != Vec2::ZERO { 
        let delta = accumulated_mouse_scroll.delta;
        camera.1.scale -=  delta.y * ds * system::FPS;
        if camera.1.scale < 1.0{camera.1.scale = 1.0}
        if camera.1.scale > 20.0{camera.1.scale = 20.0;}
    }
    

    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    if up || down || left || right{
        let x_axis = -(left as i8) + right as i8;
        let y_axis = -(down as i8) + up as i8;
        camera.0.translation.x += x_axis as f32 * ds * 500.0;
        camera.0.translation.y += y_axis as f32 * ds * 500.0;
    }    
}

pub fn update_gismo(mut gizmos: Gizmos){
    gizmos.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(200,200),
        Vec2::new(10., 10.),
        Color::srgba(0.0,0.0,0.0, 0.15),
    ).outer_edges();

    gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::splat(10.0), Color::BLACK);
}

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut app: ResMut<MyApp>,
) {
    egui::Window::new("").show(contexts.ctx_mut(), |ui| {
        egui::Grid::new("my_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.horizontal(|ui|{
                ui.label("path: ");
                ui.text_edit_singleline(&mut app.cs.stage_path);
                if ui.button("...").clicked(){
        
                }
            });
           
        });
    });
}