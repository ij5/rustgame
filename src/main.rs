use bevy::{prelude::*, input::mouse::MouseMotion};
use bevy_vox_mesh::VoxMeshPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use std::f32::consts::PI;
use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};

fn main(){
    println!("Game started...");
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rust Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(VoxMeshPlugin::default())
        .add_plugin(LookTransformPlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .add_system(cursor_grab_system)
        .add_system(mouse_motion)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct MainCamera;

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>){
    let mut direction = Vec3::ZERO;
    if keys.any_pressed([KeyCode::W]) {
        direction.z -= 1.;
    }
    if keys.any_pressed([KeyCode::S]) {
        direction.z += 1.;
    }
    if keys.any_pressed([KeyCode::A]) {
        direction.x -= 1.;
    }
    if keys.any_pressed([KeyCode::D]) {
        direction.x += 1.;
    }
    
    let move_speed = 0.13;
    let move_delta = direction * move_speed;

    for mut transform in player_query.iter_mut() {
        transform.translation += move_delta;
    }
}

fn mouse_motion(
    mut motion_evr: EventReader<MouseMotion>,
    windows: Res<Windows>,
    mut camera_query: Query<&mut LookTransform, With<MainCamera>>,
) {
    let window = windows.get_primary().unwrap();
    if !window.cursor_locked() {
        return;
    }
    for ev in motion_evr.iter() {
        let mut direction = Vec3::ZERO;
        direction.x -= ev.delta.x;
        direction.y += ev.delta.y;
        let move_delta = direction * 0.1;
        for mut transform in camera_query.iter_mut() {
            transform.eye += move_delta;
            transform.look_direction();
        }
    }
}

fn spawn_player(mut commands: Commands, mut stdmats: ResMut<Assets<StandardMaterial>>, assets: Res<AssetServer>) {
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_scale((0.05, 0.05, 0.05).into()),
            // * Transform::from_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
        mesh: assets.load("chicken.vox"),
        material: stdmats.add(Color::rgb(1., 1., 1.).into()),
        ..Default::default()
    }).insert(Player);
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}

fn setup(
    mut commands: Commands,
) {
    let eye = Vec3::new(0., 3., 7.);
    let target = Vec3::new(0., 0., 0.);
    commands.spawn_bundle(LookTransformBundle {
        transform: LookTransform::new(eye, target),
        smoother: Smoother::new(0.9),
    }).insert_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 3., 7.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    }).insert(MainCamera);

    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(-1.0, 3., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        point_light: PointLight {
            intensity: 1600.,
            color: Color::WHITE,
            shadows_enabled: true,
            radius: 20.,
            ..default()
        },
        ..default()
    });
}

