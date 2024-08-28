use bevy::prelude::*;

pub struct CenteredCameraPlugin;
impl Plugin for CenteredCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, default_camera);
    }
}

fn default_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation = Vec3::new(140.0, -80.0, 0.0);
    camera_bundle.projection.scale = 0.3;
    commands.spawn(camera_bundle);
}
