use bevy::{prelude::*, utils::Duration};

use super::define::*;

#[derive(Component)]
pub struct UIPrintBackground;
#[derive(Component)]
pub struct UIPrint;
pub struct FPSPlugin {
    pub wait_duration: Duration,
}
impl FPSPlugin{
    pub fn new(duration: u64) ->  FPSPlugin{
        FPSPlugin { wait_duration: Duration::from_secs(duration) }
    }
}
#[derive(Resource)]
struct FPSState {
    timer: Timer,
}
impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        let state = FPSState { timer: Timer::new(self.wait_duration, TimerMode::Repeating) };
        app.insert_resource(state)
        .add_systems(Startup, setup_asset)
        .add_systems(Update, (
            print_fps,
            show_fps,
        ));
    }
}

fn setup_asset(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(5.0),
            top: Val::Px(5.0),
            //width: Val::Px(60.0),
            //height: Val::Px(20.0),
            ..default()
        },
        Visibility::Hidden,
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
        UIPrintBackground,
    )).with_children(|parent|{
        parent.spawn((
            Text::new(""),
            TextFont{
                font: asset_server.load(assets::DEFAULTFONT),
                font_size: 15.0,
                ..default()
            },
            TextColor(Color::srgba(1.0,1.0,1.0,1.0)),
            TextLayout::new_with_justify(JustifyText::Right),
            UIPrint
        ));
    });
}

fn show_fps(
    mut fps: Single<&mut Visibility, With<UIPrintBackground>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1){
        if **fps == Visibility::Hidden{ **fps = Visibility::Visible;}
        else { **fps = Visibility::Hidden;}
    }
}


fn print_fps(mut state: ResMut<FPSState>, time: Res<Time>, mut texts: Query<(&UIPrint, &mut Text)>,) {
    if !state.timer.tick(time.delta()).finished() {return;}
    let (_uic, mut ts) = texts.single_mut();
    ts.0 = format!("fps: {}", (1.0 / time.delta_secs()) as i32);
}
