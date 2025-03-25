use bevy::{
    prelude::*, 
    color::palettes::basic,
    sprite::*, 
};
use bevy_rapier2d::prelude::*;
use super::super::define::*;
use super::super::state::*;
use super::game;

#[derive(Component)]
pub struct SkipButton;

#[derive(Component)]
pub struct ResetButton;

#[derive(Component)]
pub struct MouseMoveText;
#[derive(Component)]
pub struct MouseJumpText;
#[derive(Component)]
pub struct MouseGrabText;
#[derive(Component)]
pub struct MouseScrollText(f32);

#[derive(Component)]
pub struct GrabBlinkFigure(bool);

pub fn blink_figure(
    player: Single<&game::PlayerInfo, With<game::PlayerInfo>>,
    mut blink: Single<(&mut GrabBlinkFigure, &mut Visibility), With<GrabBlinkFigure>>,
    mut app: ResMut<MyApp>,
    time: Res<Time>,
){
    if player.is_grab_rope{
        *blink.1 = Visibility::Hidden;
        return;
    }
    app.tutorial_grab_blink_timer += time.delta_secs();
    if app.tutorial_grab_blink_timer > value::TUTORIALBLINKTIMER{
        blink.0.0 = !blink.0.0;
        app.tutorial_grab_blink_timer = app.tutorial_grab_blink_timer - value::TUTORIALBLINKTIMER;
    }
    match blink.0.0{
        true => *blink.1 = Visibility::Visible,
        _ => *blink.1 = Visibility::Hidden,
    };
}

pub fn check_player_position(
    mut player: Single<(&mut game::PlayerInfo, &mut ImpulseJoint, &mut Velocity, &mut Transform) ,(With<game::PlayerInfo>, Without<game::RopeRoot>, Without<Camera2d>)>,
    mut rope_root: Single<(&mut Transform, &mut Visibility), (With<game::RopeRoot>, Without<game::PlayerInfo>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<game::PlayerInfo>)>,
){
    if player.3.translation.y > -2000.0{return;}
    player.0.is_grab_rope = true;
    player.1.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Enabled;
    player.2.linvel = Vec2::new(0.0, 0.0);
    rope_root.0.translation = Vec3::new(0.0, 0.0, 0.0);
    *rope_root.1 = Visibility::Visible;
    camera.translation = Vec3::new(0.0, 0.0, 0.0);
}

pub fn mouse_scroll_text(
    mut texts: Query<(&mut TextColor, &mut MouseScrollText), With<MouseScrollText>>,
    mouse_scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
    time: Res<Time>,
){
    let mut color = Color::srgb(0.5, 0.5, 0.5);
    if mouse_scroll.delta.y != 0.0{ 
        for mut t in texts.iter_mut(){
            t.1.0 = 0.0;
        }
    }
    for mut t in texts.iter_mut(){
        t.1.0 += time.delta_secs();
        if t.1.0 < 0.25{color = Color::srgb(1.0, 1.0, 1.0);}
        t.0.0 = color;
    }
}


pub fn mouse_grab_text(
    mut texts: Query<&mut TextColor, With<MouseGrabText>>,
    player: Single<&game::PlayerInfo, With<game::PlayerInfo>>,
){
    let mut color = Color::srgb(0.5, 0.5, 0.5);
    if player.is_grab_rope && player.grab_time < 0.5{ 
        color = Color::srgb(1.0, 1.0, 1.0);
    }
    for mut t in texts.iter_mut(){
        t.0 = color;
    }
}

pub fn mouse_jump_text(
    mut texts: Query<&mut TextColor, With<MouseJumpText>>,
    player: Single<&game::PlayerInfo, With<game::PlayerInfo>>,
){
    let mut color = Color::srgb(1.0, 1.0, 1.0);
    if player.is_grab_rope { 
        color = Color::srgb(0.5, 0.5, 0.5);
    }
    for mut t in texts.iter_mut(){
        t.0 = color;
    }
}

pub fn mouse_move_text(
    mut texts: Query<&mut TextColor, With<MouseMoveText>>,
    player: Single<(&game::PlayerInfo, &Velocity), With<game::PlayerInfo>>,
    accumulated_mouse_motion: Res<bevy::input::mouse::AccumulatedMouseMotion>,
    mut app: ResMut<MyApp>,
    time: Res<Time>,
){
    let mut color = Color::srgb(1.0, 1.0, 1.0);
    if !player.0.is_grab_rope { 
        color = Color::srgb(0.5, 0.5, 0.5);
        app.tutorial_mouse_move_timer = 0.0; 
    }else{
        if accumulated_mouse_motion.delta == Vec2::ZERO{ app.tutorial_mouse_move_timer += time.delta_secs(); }
        else                                           { app.tutorial_mouse_move_timer = 0.0; }
        if app.tutorial_mouse_move_timer > value::TUTORIALMOUSEMOVETIMER{
            color = Color::srgb(0.5, 0.5, 0.5);
        }
    }
    for mut t in texts.iter_mut(){
        t.0 = color;
    }
}

pub fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    goal: Single<Entity, With<game::GoalCollision>>,
    player: Single<Entity, With<game::PlayerInfo>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut enter_events: EventWriter<game::EnterEvent>,
){
    if collision_events.is_empty(){return;}
    for evt in collision_events.read(){
        match evt{
            CollisionEvent::Started(fe, se, _ce) => {
                let other = match fe.index() == player.index(){
                    true => {se},
                    _ => {fe}
                };
                if goal.index() == other.index(){//クリア
                    app_state.set(AppState::Game);
                    enter_events.send_default();
                }
            },
            _ => {}
        }
    }
}

pub fn rope_grab(
    mut player: Single<(&Transform, &mut ImpulseJoint, &mut game::PlayerInfo, &mut Velocity), (With<game::PlayerInfo>, Without<game::RopeRoot>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut root: Single<(&mut Transform, &mut Visibility), With<game::RopeRoot>>,
    mut jump_events: EventWriter<game::JumpEvent>,
    mut grab_events: EventWriter<game::GrabEvent>,
    app: Res<MyApp>,
    time: Res<Time>,
){
    player.2.grab_time += time.delta_secs();
    if app.is_tutorial_skip_button_hover || app.is_tutorial_reset_button_hover{return;}
    if !mouse_button_input.just_pressed(MouseButton::Left){return;}
    if !player.2.is_grab_rope{
        player.2.grab_time = 0.0;
        let mut px = player.0.translation.x;
        let mut py = player.0.translation.y;
        px -= player.3.linvel.x * 0.01;
        py -= player.3.linvel.y * 0.01;
        root.0.translation = Vec3::new(px, py, 0.0);
        *root.1 = Visibility::Visible;
        player.1.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Enabled;
        grab_events.send_default();
        player.2.is_grab_rope = true;
    }else{
        player.1.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Disabled;
        jump_events.send_default();
        *root.1 = Visibility::Hidden;
        player.2.is_grab_rope = false;
    }
}

pub fn camera(
    mut camera: Single<(&mut Transform, &mut OrthographicProjection), (With<Camera2d>, Without<game::PlayerInfo>)>,
    player: Single<&Transform, (With<game::PlayerInfo>, Without<Camera2d>)>,
    time: Res<Time>,
    accumulated_mouse_scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
){
    let ds = time.delta_secs();
    let sa = (player.translation - camera.0.translation) * ds * system::FPS*0.05;
    camera.0.translation += sa;
    if accumulated_mouse_scroll.delta == Vec2::ZERO { return; }
    let delta = accumulated_mouse_scroll.delta;
    camera.1.scale -= match debug::ISDEBUG{
        true => delta.y * ds * system::FPS,
        _ => delta.y * ds * 0.25,
    };
    if camera.1.scale < 1.0{camera.1.scale = 1.0}
    if camera.1.scale > 20.0{camera.1.scale = 20.0;}
}

pub fn push_skip_button(
    mut button: Single<(&Interaction, &Button, &mut BackgroundColor), With<SkipButton>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut app: ResMut<MyApp>,
    mut enter_events: EventWriter<game::EnterEvent>,
){
    match button.0{
        Interaction::Hovered => {
            button.2.0 = Color::srgb(0.15, 0.80, 0.15);
            app.is_tutorial_skip_button_hover = true;
        },
        Interaction::Pressed => {
            button.2.0 = Color::srgb(0.0, 0.0, 0.0);
            enter_events.send_default();
            app_state.set(AppState::Game);
        },
        _ => {
            button.2.0 = Color::srgb(0.25, 0.25, 0.25);
            app.is_tutorial_skip_button_hover = false;
        }
    };    
}

pub fn push_reset_button(
    mut button: Single<(&Interaction, &Button, &mut BackgroundColor), With<ResetButton>>,
    mut rope_root: Single<(&mut Transform, &mut Visibility), With<game::RopeRoot>>,
    mut app: ResMut<MyApp>,
    mut player: Single<(&mut game::PlayerInfo, &mut ImpulseJoint, &mut Velocity) ,With<game::PlayerInfo>>,
){
    match button.0{
        Interaction::Hovered => {
            button.2.0 = Color::srgb(0.15, 0.80, 0.15);
            app.is_tutorial_reset_button_hover = true;
        },
        Interaction::Pressed => {
            button.2.0 = Color::srgb(0.0, 0.0, 0.0);
            player.0.is_grab_rope = true;
            player.1.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Enabled;
            player.2.linvel = Vec2::new(0.0, 0.0);
            rope_root.0.translation = Vec3::new(0.0, 0.0, 0.0);
            *rope_root.1 = Visibility::Visible;
        },
        _ => {
            button.2.0 = Color::srgb(0.25, 0.25, 0.25);
            app.is_tutorial_reset_button_hover = false;
        }
    };        
}

pub fn setup_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    app: Res<MyApp>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    commands.insert_resource(ClearColor(Color::srgb(0.175, 0.175, 0.175)));
    commands.insert_resource(game::JumpSound(asset_server.load(assets::SOUNDJUMP)));
    commands.insert_resource(game::GrabSound(asset_server.load(assets::SOUNDGRAB)));
    commands.insert_resource(game::DeathSound(asset_server.load(assets::SOUNDDEATH)));
    commands.insert_resource(game::EnterSound(asset_server.load(assets::SOUNDENTER)));

    if debug::ISSKIPTUTORIAL{ app_state.set(AppState::Game); }

    commands.spawn((//カメラ
        Camera2d::default(),
        ReleaseResource
    ));
    
    commands.spawn((//バージョン表記
        Text::new(env!("CARGO_PKG_VERSION")),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 10.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::End,
            top: Val::Px(0.0),
            ..default()
        },
        ReleaseResource,
    ));

    commands.spawn((//チュートリアルテキスト
        Text::new("Tutorial"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 50.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            ..default()
        },
        ReleaseResource,
    ));

    commands.spawn((//SkipButton
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            border: UiRect::all(Val::Px(5.0)),
            left: Val::Px(230.0),
            top: Val::Px(5.0),
            ..default()
        },
        Text::new("Skip"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 40.0,
            ..default()
        },
        BorderRadius::px(5.0, 5.0, 5.0, 5.0),
        BorderColor(Color::BLACK),
        Button,
        BackgroundColor(Color::srgb(0.15,0.15, 0.15)),
        SkipButton,
        ReleaseResource,
    ));

    commands.spawn((//ResetButton
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            border: UiRect::all(Val::Px(5.0)),
            left: Val::Px(330.0),
            top: Val::Px(5.0),
            ..default()
        },
        Text::new("Reset"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 40.0,
            ..default()
        },
        BorderRadius::px(5.0, 5.0, 5.0, 5.0),
        BorderColor(Color::BLACK),
        Button,
        BackgroundColor(Color::srgb(0.15,0.15, 0.15)),
        ResetButton,
        ReleaseResource,
    ));

    commands.spawn((//説明テキスト
        Text::new("Mouse move:"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            right: Val::Px(75.0),
            bottom: Val::Px(200.0),
            ..default()
        },
        MouseMoveText,
        ReleaseResource,
    ));
    commands.spawn((//説明テキスト
        Text::new("cube swing"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::srgb(0.7, 0.7, 0.7)),
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            left: Val::Px(75.0),
            bottom: Val::Px(200.0),
            ..default()
        },
        MouseMoveText,
        ReleaseResource,
    ));
    
    commands.spawn((//説明テキスト
        Text::new("Mouse left-click:"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            right: Val::Px(92.0),
            bottom: Val::Px(170.0),
            ..default()
        },
        MouseJumpText,
        ReleaseResource,
    ));
    commands.spawn((//説明テキスト
        Text::new("jump"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            left: Val::Px(48.0),
            bottom: Val::Px(170.0),
            ..default()
        },
        MouseJumpText,
        ReleaseResource,
    ));

    commands.spawn((
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            left: Val::Px(-108.0),
            bottom: Val::Px(137.0),
            width: Val::Px(450.0),
            height: Val::Px(25.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 1.0)),
        Visibility::Hidden,
        GrabBlinkFigure(false),
        ReleaseResource,
    ));

    commands.spawn((//説明テキスト
        Text::new("When jumping, mouse left-click:"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            right: Val::Px(170.0),
            bottom: Val::Px(140.0),
            ..default()
        },
        MouseGrabText,
        ReleaseResource,
    ));
    commands.spawn((//説明テキスト
        Text::new("grab rope"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            left: Val::Px(65.0),
            bottom: Val::Px(140.0),
            ..default()
        },
        MouseGrabText,
        ReleaseResource,
    ));

    commands.spawn((//説明テキスト
        Text::new("Mouse wheel:"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            right: Val::Px(80.0),
            bottom: Val::Px(110.0),
            ..default()
        },
        MouseScrollText(0.0),
        ReleaseResource,
    ));
    commands.spawn((//説明テキスト
        Text::new("zoom in, zoom out"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            left: Val::Px(110.0),
            bottom: Val::Px(110.0),
            ..default()
        },
        MouseScrollText(0.0),
        ReleaseResource,
    ));

    commands.spawn((//説明テキスト
        Text::new("Move to the NEXT!"),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 30.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::End,
            justify_self: JustifySelf::Center,
            bottom: Val::Px(50.0),
            ..default()
        },
        ReleaseResource,
    ));

    commands.spawn((
        Collider::cuboid(10.0, 10.0),
        Sensor,
        Transform::from(Transform::from_xyz(400.0, 0.0, -10.0)),
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
        let goal_or_next = match app.stage_count == debug::MAXSTAGE{
            _ => {"NEXT!"},
        };
        for (u, c) in goal_or_next.chars().enumerate(){
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