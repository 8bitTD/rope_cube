use bevy::prelude::*;
pub mod game;
pub mod ending;
pub mod tutorial;
use super::define::*;

#[derive(Resource)] 
pub struct MyApp{
    pub stage_count: usize,
    pub game_state: GameState,
    pub game_state_timer: f32,
    pub joint_distance: f32,
    pub is_reset_game: bool,
    pub text_stage_alpha: f32,
    pub is_clear: bool,
    pub timer: f32,
    pub grab_count: usize,
    pub is_ending_end: bool,
    pub is_tutorial_skip_button_hover: bool,
    pub is_tutorial_reset_button_hover: bool,
    pub tutorial_grab_blink_timer: f32,
    pub continues: usize,
}
impl Default for MyApp{
    fn default() -> MyApp{
        MyApp { 
            stage_count: debug::STARTSTAGE,
            game_state: GameState::In,
            game_state_timer: 0.0,
            joint_distance: 100.0, 
            is_reset_game: false,
            text_stage_alpha: value::DEFAULTTEXTSTAGEALPHA,
            is_clear: debug::ISCLEAR,
            timer: 0.0,
            grab_count: 0,
            is_ending_end: false,
            is_tutorial_skip_button_hover: false,
            is_tutorial_reset_button_hover: false,
            tutorial_grab_blink_timer: 0.0,
            continues: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState{
    #[default]
    Tutorial,
    Game,
    Ending,
}
#[derive(Debug, Eq, PartialEq, Clone, Copy, Default, Resource)]
pub enum GameState{
    #[default]
    In,
    Play,
    Out,
}

#[derive(Component)]
pub struct ReleaseResource;//リソース開放用
pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build (&self, app: &mut App){
        app
        .init_state::<AppState>()
        .insert_resource(MyApp::default())
        .add_event::<game::JumpEvent>()
        .add_event::<game::GrabEvent>()
        .add_event::<game::DeathEvent>()
        .add_event::<game::EnterEvent>()
        .add_systems(OnEnter(AppState::Tutorial), tutorial::setup_asset)
        .add_systems(Update,
            (
                game::update_gismo,
                tutorial::camera,
                tutorial::rope_grab,
                tutorial::push_skip_button,
                tutorial::push_reset_button,
                game::player_move,
                game::update_play_sound,
                game::update_goal_animation,
                tutorial::collision_events,
                tutorial::mouse_move_text,
                tutorial::mouse_jump_text,
                tutorial::mouse_grab_text,
                tutorial::mouse_scroll_text,
                tutorial::check_player_position,
                tutorial::blink_figure,
            ).chain().run_if(in_state(AppState::Tutorial)),
        )
        .add_systems(PostUpdate, (game::rope_angle_animation).chain().run_if(in_state(AppState::Tutorial)))
        .add_systems(OnExit(AppState::Tutorial), despawn)
        .add_systems(OnEnter(AppState::Game), game::setup_asset)
        .add_systems(Update, 
            (
                game::update_fade_stage_text,
                game::update_gismo,
                game::player_move,
                game::rope_grab,
                //game::rope_angle_animation,
                game::camera,
                game::collision_events,
                game::reset_game,
                game::update_game_state,
                game::update_goal_animation,
                game::update_play_sound,
                game::debug,
            ).chain().run_if(in_state(AppState::Game)),
        )
        .add_systems(PostUpdate, (game::rope_angle_animation).chain().run_if(in_state(AppState::Game)))
        .add_systems(OnExit(AppState::Game), despawn)
        
        .add_systems(OnEnter(AppState::Ending), ending::spawn_system)
        .add_systems(Update, 
            (
                ending::update_debug,
                ending::update_player,
                ending::update_move_text,
                ending::update_click_text_animation,
            ).run_if(in_state(AppState::Ending)),
        )
        .add_systems(OnExit(AppState::Ending), despawn);
        
    }
}

pub fn despawn(
    mut commands: Commands, 
    query: Query<Entity, With<ReleaseResource>>,
){
    for entity in &mut query.iter() {
        commands.entity(entity).try_despawn_recursive();
    }
}
