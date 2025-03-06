use bevy::prelude::*;
use bevy_web_asset::WebAssetPlugin;

fn main() {
    App::new()
        .add_plugins(WebAssetPlugin)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut command: Commands,
    mut mesh: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
){
    // camera load
    command.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // light load
    command.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Plan load
    command.spawn((
        Mesh3d(mesh.add(Circle::new(4.0))),
        MeshMaterial3d(material.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // Character load
    command.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("https://assets.choiyunseok.com/models/base_character_girl.glb"),
    )));
}