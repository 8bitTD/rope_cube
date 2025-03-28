use bevy::{
    prelude::*, 
    color::palettes::basic,
    sprite::*, 
    audio,
};
use bevy_rapier2d::prelude::*;
//use rapier2d::prelude::RigidBodyChanges;
//use rapier2d::prelude::RigidBodyType;
use rand::distributions::{Distribution, Uniform};


//use bevy_inspector_egui::quick::WorldInspectorPlugin;
use super::super::define::*;
use super::super::state::*;
use super::super::stage::*;
use super::super::facial::*;

#[derive(Debug, Component)]
pub struct RopeRoot;

#[derive(Debug, Component)]
pub struct PlayerInfo{
    pub is_grab_rope: bool,
    pub grab_time: f32,
    pub grab_power: f32,
}
impl Default for PlayerInfo{
    fn default() -> PlayerInfo{
        PlayerInfo { 
            is_grab_rope: true,
            grab_time: 1.0,
            grab_power: 50.0
        }
    }
}

#[derive(Component)]
pub struct FixedBlock;

#[derive(Component)]
pub struct GoalCollision;

#[derive(Component)]
pub struct GoalBlock;

#[derive(Component)]
pub struct BlackRectangle;

#[derive(Component)]
pub struct GoalText;

#[derive(Component)]
pub struct RopeAngle;
#[derive(Component)]
pub struct RopeSprite;

#[derive(Component)]
pub struct StageText;


#[derive(Event, Default)]
pub struct JumpEvent;
#[derive(Resource)]
pub struct JumpSound(pub Handle<AudioSource>);
#[derive(Event, Default)]
pub struct GrabEvent;
#[derive(Resource)]
pub struct GrabSound(pub Handle<AudioSource>);
#[derive(Event, Default)]
pub struct DeathEvent;
#[derive(Resource)]
pub struct DeathSound(pub Handle<AudioSource>);

#[derive(Event, Default)]
pub struct EnterEvent;
#[derive(Resource)]
pub struct EnterSound(pub Handle<AudioSource>);

#[derive(Component)]
pub struct PlayerParticle{
    pub tx: f32,
    pub ty: f32,
    pub vx: f32,
    pub vy: f32,
}
#[derive(Component)]
pub struct PlayerParticleRoot;

pub fn update_play_sound(
    mut commands: Commands,
    jump_sound: Res<JumpSound>,
    mut jump_events: EventReader<JumpEvent>,
    grab_sound: Res<GrabSound>,
    mut grab_events: EventReader<GrabEvent>,
    death_sound: Res<DeathSound>,
    mut death_events: EventReader<DeathEvent>,
    enter_sound: Res<EnterSound>,
    mut enter_events: EventReader<EnterEvent>,
) {
    if !jump_events.is_empty() {
        jump_events.clear();
        commands.spawn((
            AudioPlayer::new(jump_sound.0.clone()),
            PlaybackSettings {
                mode: { audio::PlaybackMode::Despawn },
                volume: audio::Volume::new(value::VOLUME),
                ..default()
            },
        ));
    }
    if !grab_events.is_empty() {
        grab_events.clear();
        commands.spawn((
            AudioPlayer::new(grab_sound.0.clone()),
            PlaybackSettings {
                mode: { audio::PlaybackMode::Despawn },
                volume: audio::Volume::new(value::VOLUME),
                ..default()
            },
        ));
    }
    if !death_events.is_empty() {
        death_events.clear();
        commands.spawn((
            AudioPlayer::new(death_sound.0.clone()),
            PlaybackSettings {
                mode: { audio::PlaybackMode::Despawn },
                volume: audio::Volume::new(value::VOLUME),
                ..default()
            },
        ));
    }

    if !enter_events.is_empty() {
        enter_events.clear();
        commands.spawn((
            AudioPlayer::new(enter_sound.0.clone()),
            PlaybackSettings {
                mode: { audio::PlaybackMode::Despawn },
                volume: audio::Volume::new(value::VOLUME),
                ..default()
            },
        ));
    }
    
}

pub fn update_fade_stage_text(
    mut app: ResMut<MyApp>, 
    time: Res<Time>,
    mut text_query: Query<(&mut Text, &mut TextColor, &TextFont), With<StageText>>,
){
    if app.game_state == GameState::Out{return;}
    if app.text_stage_alpha <= -1.0{return;}
    for mut t in text_query.iter_mut(){
        if app.text_stage_alpha == value::DEFAULTTEXTSTAGEALPHA  &&  t.2.font_size == 100.0{
            t.0.0 =  match app.stage_count == debug::MAXSTAGE {
                true => {"Last Stage".into()},
                _ => {format!("Stage {}",app.stage_count)},
            };
        }
        let alpha = match app.text_stage_alpha > 1.0{
            true => 1.0,
            _ => app.text_stage_alpha,
        };
        t.1.0 = Color::srgba(1.0,1.0,1.0, alpha);
    }
    app.text_stage_alpha -= time.delta_secs();
}

pub fn rope_angle_animation(//ロープの長さ、角度を調整する処理
    player: Single<(&Transform, &PlayerInfo, &Velocity), (With<PlayerInfo>, Without<RopeAngle>, Without<RopeSprite>, Without<RopeRoot>)>,
    rope_root: Single<&Transform, (With<RopeRoot>, Without<RopeSprite>, Without<RopeAngle>, Without<PlayerInfo>)>,
    mut rope_angle: Single<&mut Transform, (With<RopeAngle>, Without<RopeSprite>, Without<RopeRoot>,Without<PlayerInfo>)>,
    mut rope_sprite: Single<(&mut Sprite, &mut Transform, &mut Visibility), (With<RopeSprite>, Without<RopeAngle>, Without<RopeRoot>, Without<PlayerInfo>)>,
    mut app: ResMut<MyApp>,
    time: Res<Time>,
){
    if player.1.is_grab_rope{ *rope_sprite.2 = Visibility::Visible;}
    else                    { *rope_sprite.2 = Visibility::Hidden; }
    //app.joint_distance = value::DEFAULTJOINTDISTANCE * player.2.linvel.element_sum();
    let pp = player.0.translation;
    let pv = player.2.linvel * time.delta_secs();
    let rp = rope_root.translation;
    let sax = pp.x + pv.x - rp.x;
    let say = pp.y + pv.y - rp.y;
    let val = say.atan2(sax) - 1.5708;
    rope_angle.rotation = Quat::from_rotation_z(val);
    let distance = ((pp.x - rp.x).powi(2) + (pp.y - rp.y).powi(2)).sqrt();
    app.rope_distance = distance;
    rope_sprite.1.scale.x = 2.0;
    rope_sprite.1.scale.y = distance / value::DEFAULTROPEDISTANCE;
    rope_sprite.1.translation.y = distance * 0.5;
}

pub fn update_goal_animation(
    mut text_query: Query<(&mut TextColor, &mut Transform), With<GoalText>>,
    mut goal_material: Single<&mut MeshMaterial2d<ColorMaterial>, With<GoalBlock>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>
) {
    let elapsed = time.elapsed().as_secs_f32();
    let r_wave = (2.0 * std::f32::consts::PI * elapsed  / 1.24 as f32).sin() + 0.8;
    let g_wave = (2.0 * std::f32::consts::PI * elapsed  / 0.77 as f32).sin() + 0.8;
    let b_wave = (2.0 * std::f32::consts::PI * elapsed  / 1.03 as f32).sin() + 0.8;
    goal_material.0 =  materials.add(Color::srgb(r_wave, g_wave, b_wave));

    for (u, (mut text, mut transform))in text_query.iter_mut().enumerate(){
        let os = (u+1) as f32 * 0.175;
        let transform_wave_y = ((2.0 * std::f32::consts::PI * (elapsed - os)  / 1.0 as f32).sin() + 3.0) * 0.075;
        transform.scale.y = transform_wave_y;
        let r_wave = (2.0 * std::f32::consts::PI * (elapsed - os)  / 1.24 as f32).sin() + 0.8;
        let g_wave = (2.0 * std::f32::consts::PI * (elapsed - os)  / 0.77 as f32).sin() + 0.8;
        let b_wave = (2.0 * std::f32::consts::PI * (elapsed - os)  / 1.03 as f32).sin() + 0.8;
        text.0 = Color::srgb(r_wave, g_wave, b_wave);
    }
}

pub fn update_game_state(
    mut app: ResMut<MyApp>,
    mut black_color: Single<&mut MeshMaterial2d<ColorMaterial>, With<BlackRectangle>>,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut facial: Query<&mut Visibility, With<FacialParts>>,
){
    if app.game_state == GameState::In{
        app.game_state_timer += time.delta_secs();
        black_color.0 = materials.add(Color::srgba(0.0,0.0,0.0, 1.0 - (app.game_state_timer*1.0)));
        if app.game_state_timer >= value::FADETIME{
            black_color.0 = materials.add(Color::srgba(0.0,0.0,0.0, 0.0));
            app.game_state_timer = 0.0;
            app.game_state = GameState::Play;
        }
    }
    if app.game_state == GameState::Out{
        app.game_state_timer += time.delta_secs();
        black_color.0 = materials.add(Color::srgba(0.0,0.0,0.0, app.game_state_timer*1.0));
        if app.game_state_timer >= value::FADETIME{
            black_color.0 = materials.add(Color::srgba(0.0,0.0,0.0, 1.0));
            for mut v in facial.iter_mut(){*v = Visibility::Visible;}
            app.game_state_timer = 0.0;
            app.game_state = GameState::In;
            app.is_reset_game = true;
        }
    }
}

pub fn reset_game(
    mut commands: Commands,
    mut player: Single<(&mut PlayerInfo, &mut Transform, &mut Velocity, &mut ImpulseJoint, Entity, &mut Sprite), (With<PlayerInfo>, Without<PlayerParticle>)>,
    mut player_particle: Query<(&mut RigidBody, &mut Transform, &PlayerParticle), (
        With<PlayerParticle>, Without<PlayerInfo>, Without<FixedBlock>, Without<GoalCollision>, Without<RopeRoot>
    )>,
    mut player_particle_root: Single<&mut Transform, (With<PlayerParticleRoot>, Without<PlayerInfo>, Without<PlayerParticle>, Without<RopeRoot>)>,
    mut root: Single<(&mut Transform, &mut Visibility), (With<RopeRoot>, Without<PlayerInfo>)>,
    mut app: ResMut<MyApp>, 
    mut camera: Single<&mut OrthographicProjection, With<Camera2d>>,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    meshes: ResMut<Assets<Mesh>>,
    block_query: Query<Entity, With<FixedBlock>>,
    goal_query: Query<Entity, With<GoalCollision>>,
    mut app_state: ResMut<NextState<AppState>>,
){
    if !app.is_reset_game{return;}
    if app.stage_count > debug::MAXSTAGE{app_state.set(AppState::Ending);}
    commands.entity(player.4).remove::<RigidBodyDisabled>();
    player.1.translation = Vec3::new(0.0, 0.0, 0.0);
    player.5.custom_size = Some(Vec2::new(20.0, 20.0)); 
    player.2.linvel = Vec2::new(0.0, 0.0);
    root.0.translation = Vec3::new(0.0, 0.0, 0.0);
    *root.1 = Visibility::Visible;
    player.3.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Enabled;
    player.0.is_grab_rope = true;
    player_particle_root.translation = Vec3::new(0.0, -1000000.0, 10.0);
    for (mut pr, mut pt, pp) in player_particle.iter_mut(){
        pt.translation.x = pp.tx;
        pt.translation.y = pp.ty;
        *pr = RigidBody::Fixed;
    }
    
    camera.scale = 1.0;
    for entity in &block_query {
        commands.entity(entity).despawn_recursive();
    }
    for entity in &goal_query {
        commands.entity(entity).despawn_recursive();
    }
    app.text_stage_alpha = value::DEFAULTTEXTSTAGEALPHA;
    app.is_reset_game = false;
    create_block(commands, app.into(), asset_server, materials, meshes);
}

pub fn camera(
    mut camera: Single<(&mut Transform, &mut OrthographicProjection), (With<Camera2d>, Without<PlayerInfo>)>,
    player: Single<&Transform, (With<PlayerInfo>, Without<Camera2d>)>,
    time: Res<Time>,
    accumulated_mouse_scroll: Res<bevy::input::mouse::AccumulatedMouseScroll>,
    goal: Single<&Transform,(With<GoalCollision>, Without<PlayerInfo>, Without<Camera2d>)>,
    app: Res<MyApp>,
){
    match app.game_state{
        GameState::In => {
            camera.0.translation = goal.translation;
        },
        _ => {
            let ds = time.delta_secs();
            let sa = (player.translation - camera.0.translation) * ds * system::FPS*0.05;
            camera.0.translation += sa;
            if accumulated_mouse_scroll.delta == Vec2::ZERO { return; }
            let delta = accumulated_mouse_scroll.delta;
            
            camera.1.scale -= get_camera_scale(delta.y, ds);

            if camera.1.scale < 1.0{camera.1.scale = 1.0}
            if camera.1.scale > 20.0{camera.1.scale = 20.0;}
        },
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_camera_scale(delta: f32, ds: f32) -> f32{
    delta * ds * system::FPS
}

#[cfg(target_arch = "wasm32")]
pub fn get_camera_scale(delta: f32, ds: f32) -> f32{
    delta * ds * 0.25
}

pub fn rope_grab(
    mut player: Single<(&Transform, &mut ImpulseJoint, &mut PlayerInfo, &mut Velocity), (With<PlayerInfo>, Without<RopeRoot>)>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut root: Single<(&mut Transform, &mut Visibility), With<RopeRoot>>,
    mut app: ResMut<MyApp>,
    mut jump_events: EventWriter<JumpEvent>,
    mut grab_events: EventWriter<GrabEvent>,
){
    if app.game_state != GameState::Play{return;}
    if !mouse_button_input.just_pressed(MouseButton::Left){return;}
    if !player.2.is_grab_rope{
        let mut px = player.0.translation.x;
        let mut py = player.0.translation.y;
        px -= player.3.linvel.x * 0.01;
        py -= player.3.linvel.y * 0.01;
        root.0.translation = Vec3::new(px, py, 0.0);
        *root.1 = Visibility::Visible;
        player.1.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Enabled;
        //player.1.data.as_mut().set_local_anchor1(Vec2::new(0.0, 100.0));
        app.grab_count += 1;
        grab_events.send_default();
        player.2.is_grab_rope = true;
    }else{
        player.1.data.as_mut().raw.enabled = rapier2d::dynamics::JointEnabled::Disabled;
        jump_events.send_default();
        *root.1 = Visibility::Hidden;
        player.2.is_grab_rope = false;
    }
}

#[cfg(target_arch = "wasm32")]
pub fn debug(){}

#[cfg(not(target_arch = "wasm32"))]
pub fn debug(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut root: Single<&mut Transform, With<RopeRoot>>,
    time: Res<Time>,
    mut app: ResMut<MyApp>,
){
    
    let up = keyboard_input.any_pressed([KeyCode::KeyW, KeyCode::ArrowUp]);
    let down = keyboard_input.any_pressed([KeyCode::KeyS, KeyCode::ArrowDown]);
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);
    let x_axis = -(left as i8) + right as i8;
    let y_axis = -(down as i8) + up as i8;
    let ds = time.delta_secs();
    root.translation.x += x_axis as f32 * ds * 500.0;
    root.translation.y += y_axis as f32 * ds * 500.0;

    if keyboard_input.pressed(KeyCode::ShiftLeft) && keyboard_input.just_pressed(KeyCode::KeyN){
        app.game_state = GameState::Out;
        app.stage_count -= 1;
        if app.stage_count < 1{ app.stage_count = 1; }
    }else if keyboard_input.just_pressed(KeyCode::KeyN){
        app.game_state = GameState::Out;
        app.stage_count += 1;
    }
    /*
    if keyboard_input.just_pressed(KeyCode::KeyC){
        app.game_state = GameState::Out;
        app_state.set(AppState::CreateStage);
    }
    */
}

pub fn player_move(
    time: Res<Time>,
    mut player: Single<(&mut Velocity, &PlayerInfo), With<PlayerInfo>>,
    q_windows: Query<&Window, With<bevy::window::PrimaryWindow>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app: ResMut<MyApp>,
){
    app.timer += time.delta_secs();
    if q_windows.single().cursor_position().is_none(){return;}
    if !player.1.is_grab_rope{return;}
    let ws = q_windows.single().size();
    let mut m_pos = q_windows.single().cursor_position().unwrap();
    m_pos.x -= ws.x * 0.5;
    m_pos.y -= ws.y * 0.5;
    let ds = time.delta_secs() * system::FPS;
    let x_axis = m_pos.x;
    let y_axis = -m_pos.y;
    player.0.linvel.x += x_axis as f32 * ds * 0.02;
    player.0.linvel.y += y_axis as f32 * ds * 0.02; 
    if keyboard_input.just_pressed(KeyCode::Escape){ app.is_reset_game = true; }
}

pub fn update_gismo(mut gizmos: Gizmos){
    gizmos.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(40,40),
        Vec2::new(100., 100.),
        Color::srgba(0.0,0.0,0.0, 0.15),
    ).outer_edges();
}

pub fn collision_events(
    mut collision_events: EventReader<CollisionEvent>,
    blocks: Query<Entity, With<FixedBlock>>,
    goal: Single<Entity, With<GoalCollision>>,
    mut app: ResMut<MyApp>,
    mut commands: Commands,
    mut player: Single<(Entity, &mut Sprite, &Transform), With<PlayerInfo>>,
    mut death_events: EventWriter<DeathEvent>,
    mut player_particle: Query<(&mut RigidBody, &mut Velocity, &PlayerParticle), (With<PlayerParticle>, Without<PlayerInfo>, Without<GoalCollision>, Without<FixedBlock>)>,
    mut player_particle_root: Single<&mut Transform, (With<PlayerParticleRoot>, Without<PlayerInfo>)>,
    mut enter_events: EventWriter<EnterEvent>,
    mut facial: Query<&mut Visibility, With<FacialParts>>,
){
    if app.game_state != GameState::Play {return;}
    if collision_events.is_empty(){return;}
    for evt in collision_events.read(){
        match evt{
            CollisionEvent::Started(fe, se, _ce) => {
                let other = match fe.index() == player.0.index(){
                    true => {se},
                    _ => {fe}
                };
                let res = blocks.get(*other);
                if res.is_ok(){//ゲームオーバー
                    death_events.send_default();
                    app.game_state = GameState::Out;
                    for mut v in facial.iter_mut(){*v = Visibility::Hidden;}
                    commands.entity(player.0).insert(RigidBodyDisabled);
                    player_particle_root.translation = player.2.translation;
                    for (mut pr, mut pv, pp) in player_particle.iter_mut(){
                        *pr = RigidBody::Dynamic;
                        pv.linvel.x = pp.vx;
                        pv.linvel.y = pp.vy;
                    }
                    player.1.custom_size = Some(Vec2::new(0.0, 0.0));
                    app.continues += 1;
                }
                if goal.index() == other.index(){//クリア
                    app.game_state = GameState::Out;
                    app.stage_count += 1;
                    enter_events.send_default();
                    commands.entity(player.0).insert(RigidBodyDisabled);
                }
            },
            _ => {}
        }
    }
}

pub fn facial_animation(
    mut f_normal: Single<(&mut Transform, &mut FacialNormal, &mut GlobalTransform, &mut Visibility), (With<FacialNormal>, Without<FacialNormalEyes>)>,
    mut f_normal_eyes: Single<(&mut Transform, &mut GlobalTransform), With<FacialNormalEyes>>,
    mut f_root: Single<(&mut Visibility, &mut FacialRoot), (With<FacialRoot>, Without<FacialNormal>, Without<FacialSmile>, Without<FacialNormalEyes>)>,
    mut f_smile: Single<(&mut Visibility, &mut FacialSmile, &mut Transform), (With<FacialSmile>, Without<FacialNormal>, Without<FacialRoot>, Without<FacialNormalEyes>)>,
    app: Res<MyApp>,
    time: Res<Time>,
    player: Single<&PlayerInfo, With<PlayerInfo>>
){
    f_root.1.state_timer += time.delta_secs();

    match app.rope_distance > value::DEFAULTROPEDISTANCE * 0.5 {
        true => {
            if player.is_grab_rope{
                f_root.1.state = FacialState::Smile;
                f_smile.1.is_smile_jump = true;
            }else{
                if !f_smile.1.is_smile_jump{
                    f_root.1.state = FacialState::Normal;
                }
            }
        },
        _ => {
            f_root.1.state = FacialState::Normal;
            f_smile.1.is_smile_jump = false;
        }
    }
    match f_root.1.state{
        FacialState::Normal => {
            f_smile.2.scale = Vec3::new(0.0, 0.0, 0.0);
            f_normal.0.scale = Vec3::new(1.0, 1.0, 1.0);
        },
        _ => {
            f_smile.2.scale = Vec3::new(1.0, 1.0, 1.0);
            f_normal.0.scale = Vec3::new(0.0, 0.0, 0.0);
        }
    };

    //瞬き
    f_normal.1.blink_timer += time.delta_secs();
    if f_normal.1.blink_timer > f_normal.1.blink_period_timer && f_normal.1.blink_timer < f_normal.1.blink_period_timer + value::FACIALBLINK{
        f_normal_eyes.0.scale.y = 0.5;
    }else if f_normal.1.blink_timer > f_normal.1.blink_period_timer + value::FACIALBLINK{
        f_normal.1.blink_timer = 0.0;
        f_normal.1.period_timer_reset();
    }else{
        f_normal_eyes.0.scale.y = 1.0;
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {

    let col = Color::srgb(0.75, 0.50, 0.25);
    let root = commands.spawn((//ロープの根元部分
        Mesh2d(meshes.add(Circle::new(2.5))),
        MeshMaterial2d(materials.add(col)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Fixed,
        Sensor,
        Velocity::zero(),
        Collider::cuboid(2.0, 2.0),
        Visibility::Visible,
        game::RopeRoot,
        ReleaseResource
    )).with_children(|parent|{
        parent.spawn((
            Transform::from_xyz(0.0, 0.0, 0.0),
            game::RopeAngle,
        )).with_children(|parent2|{
            parent2.spawn((
                Sprite{
                    color: col,
                    custom_size: Some(Vec2::new(1.0,value::DEFAULTROPEDISTANCE)),
                    ..Default::default()
                },
                Transform::from_xyz(0.0, 0.0, -10.0),
                Visibility::Visible,
                game::RopeSprite
            ));
        });
    }).id();
    let joint = RopeJointBuilder::new(value::DEFAULTROPEDISTANCE)
        .set_motor(0.0, 10.0, 50.0, 0.25)
        //.max_distance(100.0)
        //.motor_model(MotorModel::ForceBased)
        //.set_motor(0.0, 0.0, 30.0, 1.0)
        //.local_axis2(Vec2::new(0.0, 10.0))
        //.motor_velocity(1.0, 1.0)
        .local_anchor1(Vec2::new(0.0, 10.0))
        //.local_anchor2(Vec2::new( 0.001, 0.0));
        .local_anchor2(Vec2::new( 0.0, 0.005));
    commands.spawn((
        Sprite{
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Dynamic,
        GravityScale(0.75),
        ActiveEvents::COLLISION_EVENTS,
        Visibility::Visible,
        //LockedAxes::ROTATION_LOCKED_Z,
        Velocity::zero(),
        Collider::cuboid(4.0, 4.0),
        ImpulseJoint::new(root, joint),
        PlayerInfo::default(),
        ReleaseResource
    )).with_children(|parent|{
        parent.spawn((
            FacialRoot::default(),
            Transform::from_xyz(0.0, 0.0, 1.0),
            Visibility::Visible,
        )).with_children(|parent2|{
            parent2.spawn((
                FacialNormal::default(),
                Transform::from_xyz(0.0, 0.0, 1.0),
                Visibility::Visible,
            )).with_children(|parent3|{
                parent3.spawn((
                    FacialNormalEyes,
                    Transform::from_xyz(0.0, 0.0, 1.0),
                )).with_children(|parent4|{
                    parent4.spawn((
                        Mesh2d(meshes.add(Circle::new(1.75))),
                        Transform::from_xyz(-4.5, 3.0, 1.0),
                        MeshMaterial2d(materials.add(Color::BLACK)),
                        FacialParts,
                    ));
                    parent4.spawn((
                        Mesh2d(meshes.add(Circle::new(1.75))),
                        Transform::from_xyz(4.5, 3.0, 1.0),
                        MeshMaterial2d(materials.add(Color::BLACK)),
                        FacialParts,
                    ));
                });
                let mut l_mouse = Transform::from_xyz(-2.0, -5.0, 1.0);
                l_mouse.rotate_z((-15.0_f32).to_radians());
                let mut r_mouse = Transform::from_xyz(2.0, -5.0, 1.0);
                r_mouse.rotate_z((15.0_f32).to_radians());
                parent3.spawn((
                    Mesh2d(meshes.add(Rectangle::new(5.0, 2.0))),
                    l_mouse,
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));
                parent3.spawn((
                    Mesh2d(meshes.add(Rectangle::new(5.0, 2.0))),
                    r_mouse,
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));
            });

            let mut lu_eye = Transform::from_xyz(-5.0, 4.0, 1.0);
            lu_eye.rotate_z((-20.0_f32).to_radians());
            let mut lb_eye = Transform::from_xyz(-5.0, 2.0, 1.0);
            lb_eye.rotate_z((20.0_f32).to_radians());
            let mut ru_eye = Transform::from_xyz(5.0, 4.0, 1.0);
            ru_eye.rotate_z((200.0_f32).to_radians());
            let mut rb_eye = Transform::from_xyz(5.0, 2.0, 1.0);
            rb_eye.rotate_z((160.0_f32).to_radians());
            parent2.spawn((
                Transform::from_xyz(0.0, 0.0, 1.0),
                Visibility::Visible,
                FacialSmile::default(),
            )).with_children(|parent3|{
                parent3.spawn((
                    Mesh2d(meshes.add(Rectangle::new(7.0, 1.5))),
                    lu_eye,
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));
                parent3.spawn((
                    Mesh2d(meshes.add(Rectangle::new(7.0, 1.5))),
                    lb_eye,
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));

                parent3.spawn((
                    Mesh2d(meshes.add(Rectangle::new(7.0, 1.5))),
                    ru_eye,
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));
                parent3.spawn((
                    Mesh2d(meshes.add(Rectangle::new(7.0, 1.5))),
                    rb_eye,
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));

                parent3.spawn((
                    Mesh2d(meshes.add(Triangle2d::new(Vec2::new(-4.0, 1.0), Vec2::new(4.0, 1.0), Vec2::new(0.0, -6.0)))),
                    Transform::from_xyz(0.0, -4.0, 1.0),
                    MeshMaterial2d(materials.add(Color::BLACK)),
                    FacialParts,
                ));
            });
        });
    });
    commands.spawn((
        Transform::from_xyz(0.0, -1000000.0, 10.0),
        ReleaseResource,
        PlayerParticleRoot,
    )).with_children(|parent|{
        for x in 0..10{
            for y in 0..10{
                let tx = (x as f32 * 2.0) - 9.0;
                let ty = (y as f32 * 2.0) - 9.0;
                let range_x = Uniform::new(-1000.0,1000.0);
                let mut rng_vx = rand::thread_rng();
                let vx = range_x.sample(&mut rng_vx);
                let range_y = Uniform::new(0.0,400.0);
                let mut rng_vy = rand::thread_rng();
                let vy = range_y.sample(&mut rng_vy);
                let range_sx = Uniform::new(2.0,5.0);
                let mut rng_sx = rand::thread_rng();
                let sx = range_sx.sample(&mut rng_sx);
                let range_sy = Uniform::new(2.0,5.0);
                let mut rng_sy = rand::thread_rng();
                let sy = range_sy.sample(&mut rng_sy);
                parent.spawn((
                    Sprite{
                        color: Color::srgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(sx, sy)),
                        ..Default::default()
                    },
                    RigidBody::Fixed,
                    Ccd::enabled(),
                    Transform::from_xyz(tx, ty, 10.0),
                    Velocity::zero(),
                    Collider::cuboid(sx-2.0, sy-2.0),
                    PlayerParticle{tx: tx, ty: ty, vx: vx, vy: vy},
                ));
            }
        }
    });
}

pub fn setup_asset(
    mut commands: Commands,
    mut app: ResMut<MyApp>, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(ClearColor(Color::srgb(0.175, 0.175, 0.175)));
    commands.insert_resource(JumpSound(asset_server.load(assets::SOUNDJUMP)));
    commands.insert_resource(GrabSound(asset_server.load(assets::SOUNDGRAB)));
    commands.insert_resource(DeathSound(asset_server.load(assets::SOUNDDEATH)));

    *app = MyApp::default();

    commands.spawn((//カメラ
        Camera2d::default(),
        ReleaseResource
    ));
    
    commands.spawn((//黒フェード用
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(Color::from(basic::BLACK))),
        Transform::default().with_translation(Vec3::new(0.0,0.0,10.0)).with_scale(Vec3::splat(20000.0)),
        Visibility::Visible,
        BlackRectangle,
        ReleaseResource,
    ));
    
    let stage_text = match app.stage_count == debug::MAXSTAGE{
        true => {"Last Stage".into()},
        _ => {format!("Stage {}",app.stage_count)},
    };
    commands.spawn((//ステージ表示テキスト
        Text::new(&stage_text),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 100.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Px(-150.0),
            ..default()
        },
        StageText,
        ReleaseResource,
    ));

    commands.spawn((
        Text::new(format!("(Total {} Stages)", debug::MAXSTAGE)),
        TextFont {
            font: asset_server.load(assets::DEFAULTFONT),
            font_size: 50.0,
            ..default()
        },
        Node {
            position_type: PositionType::Relative,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            top: Val::Px(-55.0),
            ..default()
        },
        StageText,
        ReleaseResource,
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
    commands.spawn((//ループBGM用
        AudioPlayer::new(asset_server.load(assets::BGM)),
        PlaybackSettings {
            mode: { audio::PlaybackMode::Loop },
            volume: audio::Volume::new(value::VOLUME),
            ..default()
        },
        ReleaseResource
    ));
    create_block(commands, app.into(), asset_server, materials, meshes);
}

pub fn create_block(
    mut commands: Commands,
    app: Res<MyApp>, 
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
){
    //let stage_name = format!("stage_{:03}",app.stage_count);
    //let stage_path = format!("stage/{}.json", &stage_name);
    let stage = Stage::new_stage(app.stage_count);
    for b in stage.blocks{//ブロック作成
        let mut tfm = Transform::from(Transform::from_xyz(b.px, b.py, 0.0));
        tfm.rotate_z(b.degree.to_radians());
        commands.spawn((
            Sprite{
                color: Color::srgb(0.1, 0.1, 0.1),
                custom_size: Some(Vec2::new(b.sx * 2.0, b.sy * 2.0)),
                ..Default::default()
            },
            Collider::cuboid(b.sx, b.sy),
            //Transform::from(Transform::from_xyz(b.px, b.py, 0.0)),
            tfm,
            FixedBlock,
            ReleaseResource
        ));
    }
    commands.spawn((
        Collider::cuboid(10.0, 10.0),
        Sensor,
        Transform::from(Transform::from_xyz(stage.goal.px, stage.goal.py, -10.0)),
        GoalCollision,
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
            GoalBlock,
        ));
        let goal_or_next = match app.stage_count == debug::MAXSTAGE{
            true => {"GOAL!"},
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
                GoalText,
                ReleaseResource,
            ));
        }
    });
}
