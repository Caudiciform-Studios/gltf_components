use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct StartingCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: None,
                    exit_condition: bevy::window::ExitCondition::DontExit,
                    close_when_requested: false,
                }))
        .add_plugins(gltf_components::plugin)
        .add_systems(Startup, spawn_example)
        .add_systems(Update, print_starting_cubes)
        .register_type::<StartingCube>()
        .run();
}

fn spawn_example(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("models/example.glb"),
    )));
}

fn print_starting_cubes(query: Query<Entity, With<StartingCube>>) {
    for entity in &query {
        println!("{entity:?}");
    }
}
