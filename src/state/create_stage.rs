use bevy::{
    prelude::*, 
    color::palettes::basic,
    sprite::*, 
};
use bevy_rapier2d::prelude::*;
use bevy_egui::{egui, EguiContexts};
//use clipboard::ClipboardProvider;
//use clipboard::ClipboardContext;
use std::io::Read;

use super::super::state::*;
use super::super::define::*;

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

    app.cs.load_json();

    spawn_blocks(&mut commands, &app.cs.json.blocks);

    commands.spawn((
        Collider::cuboid(10.0, 10.0),
        Sensor,
        Transform::from(Transform::from_xyz(app.cs.json.goal.px, app.cs.json.goal.py, -10.0)),
        game::GoalCollision,
        ReleaseResource
    )).with_children(|parent|{
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(20.0, 20.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 0.0))),
            Transform::from_translation(Vec3::new(0.0,0.0,1.0)),
        ));
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(22.0, 22.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
            game::GoalBlock,
        ));
        for (u, c) in "GOAL!".chars().enumerate(){
            parent.spawn((
                Text2d::new(c.to_string()),
                TextFont {
                    font: asset_server.load(assets::DEFAULTFONT),
                    font_size: 75.0,
                    ..default()
                },
                MeshMaterial2d(materials.add(Color::from(basic::GRAY))),
                Transform::default()
                    .with_translation(Vec3::new((u as f32 * 14.0) - 21.0, 10.0,20.0))
                    .with_scale(Vec3::new(0.3,0.3,0.3)),
                Anchor::BottomCenter,
                Visibility::Visible,
                game::GoalText,
                ReleaseResource,
            ));
        }
    });
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
        Vec2::new(20., 20.),
        Color::srgba(0.0,0.0,0.0, 0.15),
    ).outer_edges();

    gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::splat(10.0), Color::BLACK);
}

pub fn update_blocks(
    app: Res<MyApp>,
    mut blocks: Query<(&mut Transform, &mut Sprite, &mut Collider), With<game::FixedBlock>>,
){
    for (u, (mut t, mut s, mut c))  in blocks.iter_mut().enumerate(){
        t.translation.x = app.cs.json.blocks[u].px;
        t.translation.y = app.cs.json.blocks[u].py;
        t.rotation = Quat::from_rotation_z(app.cs.json.blocks[u].degree.to_radians());
        *c = Collider::cuboid(app.cs.json.blocks[u].sx, app.cs.json.blocks[u].sy);
        s.custom_size = Some(Vec2::new(app.cs.json.blocks[u].sx * 2.0, app.cs.json.blocks[u].sy * 2.0));
    }
}

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut app: ResMut<MyApp>,
    mut goal: Single<&mut Transform, With<game::GoalCollision>>,
    blocks: Query<Entity, With<game::FixedBlock>>,
    mut commands: Commands,
    mut app_state: ResMut<NextState<AppState>>,
    //asset_server: Res<AssetServer>,
    //mut materials: ResMut<Assets<ColorMaterial>>,
    //mut meshes: ResMut<Assets<Mesh>>,
) {
    egui::Window::new("").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui|{
            ui.label("stage_");
            //ui.add_sized([380.0, 20.0], egui::TextEdit::singleline(&mut app.cs.stage_path));
            ui.add(egui::DragValue::new(&mut app.cs.stage_number).range(1..=20));
            if ui.button("open").clicked(){
                //app.cs.stage_path = res.unwrap().as_path().to_string_lossy().to_string().replace("\\","/");
                app.cs.load_json();
                for entity in blocks.iter() {
                    commands.entity(entity).try_despawn_recursive();
                }
                goal.translation.x = app.cs.json.goal.px;
                goal.translation.y = app.cs.json.goal.py;
                spawn_blocks(&mut commands, &app.cs.json.blocks);
            }
        });
        ui.separator();
        ui.horizontal(|ui|{
            ui.label("GOAL:");
            ui.label("px:");
            if ui.add(egui::DragValue::new(&mut app.cs.json.goal.px).speed(1.0).range(-2000.0..=2000.0)).on_hover_text("px").changed(){
                goal.translation.x = app.cs.json.goal.px;
            }
            ui.label("py:");
            if ui.add(egui::DragValue::new(&mut app.cs.json.goal.py).speed(1.0).range(-2000.0..=2000.0)).on_hover_text("py").changed(){
                goal.translation.y = app.cs.json.goal.py;
            }
        });
        ui.separator();
        let mut highlight = None;
        let mut delete_num = None;
        for (u, b) in &mut app.cs.json.blocks.iter_mut().enumerate(){
            ui.horizontal(|ui|{
                ui.label("px:");
                if ui.add(egui::DragValue::new(&mut b.px).speed(1.0).range(-2000.0..=2000.0)).hovered(){highlight = Some(u);}
                ui.label("py:");
                if ui.add(egui::DragValue::new(&mut b.py).speed(1.0).range(-2000.0..=2000.0)).hovered(){highlight = Some(u);}
                ui.label("sx:");
                if ui.add(egui::DragValue::new(&mut b.sx).speed(1.0).range(10.0..=2000.0)).hovered(){highlight = Some(u);}
                ui.label("sy:");
                if ui.add(egui::DragValue::new(&mut b.sy).speed(1.0).range(10.0..=2000.0)).hovered(){highlight = Some(u);}
                ui.label("degree:");
                if ui.add(egui::DragValue::new(&mut b.degree).speed(1.0).range(-180.0..=180.0)).hovered(){highlight = Some(u);}
                let btn = ui.button("delete");
                if btn.hovered(){highlight = Some(u);}
                if btn.clicked(){delete_num = Some(u);}
            });
        }
        app.cs.highlight_block_num = highlight;
        app.cs.delete_num = delete_num;
        if app.cs.delete_num.is_some(){
            let num = app.cs.delete_num.unwrap();
            app.cs.json.blocks.remove(num);
        }
        ui.horizontal(|ui|{
            if ui.button("create_block").clicked(){
                let bc = stage::BlockCollision {
                    px: 0.0,
                    py: 50.0,
                    sx: 20.0,
                    sy: 10.0,
                    degree: 0.0,
                };
                app.cs.json.blocks.push(bc);
                let mut tfm = Transform::from(Transform::from_xyz(0.0, 50.0, 0.0));
                tfm.rotate_z(0.0_f32.to_radians());
                commands.spawn((
                    Sprite{
                        color: Color::srgb(0.1, 0.1, 0.1),
                        custom_size: Some(Vec2::new(20.0 * 2.0, 10.0 * 2.0)),
                        ..Default::default()
                    },
                    tfm,
                    Collider::cuboid(20.0, 10.0),
                    game::FixedBlock,
                    ReleaseResource
                ));
            }
            if ui.button("save_json").clicked(){
                app.cs.save_json();
            }
            /*
            if ui.button("clipboard").clicked(){
                let cb = get_function_string(&app.cs);
                println!("{:?}", cb);
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(cb).unwrap();
            }
            */
            if ui.button("save_rs").clicked(){
                let cb = get_function_string(&app.cs);
                let stage_map_path = "./src/stage_map.rs";
                if std::fs::metadata(&stage_map_path).is_ok(){
                    let mut contents = String::new();
                    let mut f = std::fs::File::open(&stage_map_path).expect("file not found");
                    f.read_to_string(&mut contents).expect("something went wrong reading the file");
                    let fname = format!("pub fn get_stage{}() -> Stage", app.cs.stage_number);
                    let mut new_contents =  String::from("use super::stage::*;\n\n");
                    if contents.contains(&fname){//すでに関数がある場合
                        let re = regex::Regex::new(r"[p][u][b](?P<h>.[\s\S]+?)[}]").unwrap();
                        for caps in re.captures_iter(&contents) {
                            let text = caps.get(1).map_or("", |m| m.as_str());
                            let start = format!(" fn get_stage{}", app.cs.stage_number);
                            if text.starts_with(&start){
                                new_contents.push_str(&format!("{}\n", &cb));
                            }else{
                                let new_text = format!("pub{}{}\n", &text, "}");
                                new_contents.push_str(&new_text);
                            }
                        }
                    }else{//関数がない場合
                        new_contents = format!("{}{}",contents, cb);
                    }
                    let mut file = std::fs::File::create(&stage_map_path).unwrap();
                    file.write_all(new_contents.as_bytes()).unwrap();
                    println!("{:?}", "save_rs!");
                }
            }

            if ui.button("Tutorial").clicked(){
                app_state.set(AppState::Tutorial);
            }
        });
    });
}

pub fn get_function_string(cs: &CreateStage) -> String{
    let mut cb = format!("pub fn get_stage{}() -> Stage{}\n", cs.stage_number, "{");
    let ad = format!("    Stage::new({}.0, {}.0)\n", cs.json.goal.px, cs.json.goal.py);
    cb.push_str(&ad);
    for b in cs.json.blocks.iter(){
        let ad = format!("        .add_block({}.0, {}.0, {}.0, {}.0, {}.0)\n", b.px, b.py, b.sx, b.sy, b.degree);
        cb.push_str(&ad);
    }
    cb.push_str("}");
    return cb;
}

pub fn delete_block(
    app: Res<MyApp>,
    mut blocks: Query<Entity, With<game::FixedBlock>>,
    mut commands: Commands,
){
    if app.cs.delete_num.is_none(){return;}
    for (u, e) in blocks.iter_mut().enumerate(){
        if app.cs.delete_num.unwrap() != u{continue;}
        commands.entity(e).try_despawn_recursive();
    }
}

pub fn highlight_blocks(
    app: Res<MyApp>,
    mut blocks: Query<&mut Sprite, With<game::FixedBlock>>,
){
    for (u, mut b) in blocks.iter_mut().enumerate(){
        match app.cs.highlight_block_num.is_some() && app.cs.highlight_block_num.unwrap() == u{
            true => {b.color = Color::srgb(0.25, 0.25, 0.25);},
            _ => {b.color = Color::srgb(0.1, 0.1, 0.1);}
        };
    }
}

pub fn camera_focus(
    app: Res<MyApp>,
    blocks: Query<&Transform, (With<game::FixedBlock>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<game::FixedBlock>)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
){
    if !keyboard_input.just_pressed(KeyCode::KeyF){return;}
    if app.cs.highlight_block_num.is_none(){return;}
    for (u, b) in blocks.iter().enumerate(){
        if app.cs.highlight_block_num.unwrap() != u {continue;}
        camera.translation.x = b.translation.x;
        camera.translation.y = b.translation.y;
    }
}

pub fn spawn_blocks(
    commands: &mut Commands,
    blocks: &Vec<stage::BlockCollision>, 
) {
    for b in blocks{
        let mut tfm = Transform::from(Transform::from_xyz(b.px, b.py, 0.0));
        tfm.rotation = Quat::from_rotation_z(b.degree.to_radians());
        commands.spawn((
            Sprite{
                color: Color::srgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(b.sx * 2.0, b.sy * 2.0)),
                ..Default::default()
            },
            tfm,
            Collider::cuboid(b.sx, b.sy),
            game::FixedBlock,
            ReleaseResource
        ));
    }
}