use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    let zoom = 20.0;
    camera.transform.scale.x /= zoom;
    camera.transform.scale.y /= zoom;
    camera.transform.translation.x += 10.0;
    commands.spawn_bundle(camera);
}
