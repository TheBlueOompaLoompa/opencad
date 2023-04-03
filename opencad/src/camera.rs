use bevy::{
    prelude::*,
    render::camera::ScalingMode, window::PrimaryWindow
};

#[derive(Default, Resource)]
pub struct CameraControlData {
    pub target: Vec3,
    pub transform: Transform,
}

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(pub Transform);

pub fn setup_camera_controller(
    mut commands: Commands,
    mut camera_control_data: ResMut<CameraControlData>
) {
    camera_control_data.target = Vec3::ZERO;
    camera_control_data.transform = Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y);

    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            scaling_mode: ScalingMode::FixedVertical(2.0),
            ..default()
        }
        .into(),
        transform: camera_control_data.transform,
        ..default()
    });
}

pub fn camera_controller_system(
    mut camera_query: Query<&Window, With<PrimaryWindow>>,
    camera_control_data: Res<CameraControlData>
) {
    
}