use bevy::{
    prelude::*,
    winit::WinitSettings,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_egui::{EguiPlugin};
use parametric_engine::sketch::{SketchBundle, Face};
use smooth_bevy_cameras::{LookTransformPlugin, controllers::orbit::OrbitCameraPlugin};

mod camera;
mod ui;

fn main() {
    let mut _parent_face = Face::xy_base();

    let main_win = Window {
        title: "OpenCAD".to_string(),
        ..default()
    };

    App::new()
        .insert_resource(Msaa::Sample4)
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<ui::OccupiedScreenSpace>()
        .init_resource::<camera::CameraControlData>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(main_win),
            ..default()
        }))
        .add_plugin(
            MaterialPlugin::<CadMaterial>::default(),
        )
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_startup_system(camera::setup_camera_controller)
        .add_system(ui::ui_system)
        .add_system(cad_shade)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cad_materials: ResMut<Assets<CadMaterial>>,
) {
    let material = cad_materials.add(CadMaterial { camera_look_vec: Vec4::ZERO, scale: 1.0 });

    // cube
    commands.spawn( MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material:  material.clone(),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn cad_shade(
    cam_query: Query<(&Transform, &OrthographicProjection), With<Camera>>,
    mut mat_obj: ResMut<Assets<CadMaterial>>
) {
    for (_, mat) in mat_obj.iter_mut() {
        for (transform, prj) in cam_query.into_iter() {
            mat.camera_look_vec = Vec4::new(transform.back().x, transform.back().y, transform.back().z, 1.0);
            mat.scale = prj.scale;
        }
    }
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CadMaterial {
    #[uniform(0)]
    camera_look_vec: Vec4,
    #[uniform(1)]
    scale: f32
}

impl Material for CadMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cad_shader.wgsl".into()
    }
}