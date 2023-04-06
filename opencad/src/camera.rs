use bevy::{
    prelude::*,
    render::{camera::ScalingMode, view::ColorGrading}
};

use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
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

    commands
        .spawn(Camera3dBundle {
            projection: OrthographicProjection {
                scale: 5.0,
                scaling_mode: ScalingMode::FixedVertical(2.0),
                ..Default::default()
            }
            .into(),
            transform: camera_control_data.transform,
            ..Default::default()
        })
        .insert(OrbitCameraBundle::new(
            OrbitCameraController {
                smoothing_weight: 0.0,
                mouse_rotate_sensitivity:  Vec2::splat(0.3),
                mouse_translate_sensitivity:  Vec2::splat(0.3),
                ..default()
            },
            Vec3::new(-2.0, 2.5, 5.0),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));

    /*commands.spawn(OrbitCameraBundle::new(
        OrbitCameraController {
            smoothing_weight: 0.0,
            mouse_rotate_sensitivity:  Vec2::splat(0.3),
            mouse_translate_sensitivity:  Vec2::splat(0.3),
            ..default()
        },
        camera_control_data.transform.translation,
        camera_control_data.target,
        Vec3::Y,
    ));*/
}