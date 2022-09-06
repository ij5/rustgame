use bevy::prelude::*;
use bevy_vox_mesh::VoxMeshPlugin;
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
        .add_plugin(VoxMeshPlugin::default())
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_system(move_player)
        .add_system(cursor_grab_system)
        .run();
}

#[derive(Component)]
struct Player;

fn move_player(keys: Res<Input<KeyCode>>, mut player_query: Query<&mut Transform, With<Player>>){
    let mut direction = Vec3::ZERO;
    if keys.any_pressed([KeyCode::W]) {
        direction.z += 1.;
    }
    if keys.any_pressed([KeyCode::S]) {
        direction.z -= 1.;
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

fn spawn_player(mut commands: Commands, mut stdmats: ResMut<Assets<StandardMaterial>>, assets: Res<AssetServer>) {
    commands.spawn_bundle(PbrBundle {
        transform: Transform::from_scale((0.1, 0.1, 0.1).into()),
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
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 3., 7.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

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

