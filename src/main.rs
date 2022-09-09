use std::f32::consts::PI;
use bevy::{
    prelude::*, 
    input::mouse::{MouseMotion, MouseWheel}, 
    render::camera::Projection,
};
use bevy_vox_mesh::VoxMeshPlugin;
use bevy_atmosphere::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// use std::f32::consts::PI;

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
        .add_plugin(AtmospherePlugin)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .add_system(cursor_grab_system)
        .add_system(mouse_motion)
        .run();
}

#[derive(Component)]
struct Player;

fn move_player(
    keys: Res<Input<KeyCode>>, 
    mut set: ParamSet<(
        Query<&mut Transform, With<Player>>,
        Query<&mut Transform, With<PanOrbitCamera>>,
        Query<&mut PanOrbitCamera>,
    )>,
){
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

    for mut player_query in set.p0().iter_mut() {
        player_query.translation += move_delta;
    }
    for mut camera_query in set.p1().iter_mut() {
        camera_query.translation += move_delta;
    }
    for mut camera_query in set.p2().iter_mut() {
        camera_query.focus += move_delta;
    }
}

fn mouse_motion(
    mut ev_motion: EventReader<MouseMotion>,
    windows: Res<Windows>,
    mut ev_scroll: EventReader<MouseWheel>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection), With<PanOrbitCamera>>,
) {
    let window = windows.get_primary().unwrap();
    if !window.cursor_locked() {
        return;
    }
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    for ev in ev_motion.iter() {
        rotation_move += ev.delta;
    }

    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }

    for (mut orbit, mut transform, _projection) in query.iter_mut() {
        let mut any = false;
        if rotation_move.length_squared() > 0. && !orbit.disabled {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * PI * 2.0;
                if orbit.upside_down { -delta } else { delta }
            };
            let delta_y = rotation_move.y / window.y * PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;
        } else if scroll.abs() > 0.0 {
            any = true;
            orbit.radius -= scroll * orbit.radius * 0.2;
            orbit.radius = f32::max(orbit.radius, 0.05);
        }

        if any {
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation = orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, orbit.radius));
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

#[derive(Component)]
struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
    pub disabled: bool
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera { focus: Vec3::ZERO, radius: 5., upside_down: false, disabled: true }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}

fn load_scene(mut commands: Commands, asset: Res<AssetServer>) {
    let world = asset.load("scene.glb");
    
}

fn setup(
    mut commands: Commands,
) {
    let translation = Vec3::new(1., 4., 10.);
    let radius = translation.length();
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(translation).looking_at((1., 0., 0.).into(), Vec3::Y),
        ..Default::default()
    }).insert(PanOrbitCamera {
        radius,
        ..Default::default()
    }).insert(AtmosphereCamera(None));

    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(-1.0, 30., 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            color: Color::WHITE,
            shadows_enabled: true,
            illuminance: 80000.,
            ..Default::default()
        },
        ..default()
    });
}

