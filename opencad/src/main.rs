use bevy::{
    prelude::*,
    winit::WinitSettings,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use parametric_engine::{sketch};

mod camera;
mod ui;

fn main() {
    let mut parent_face = parametric_engine::sketch::Face{edges: vec![]};
    let test_sketch = sketch::create_sketch(&mut parent_face);

    App::new()
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<ui::OccupiedScreenSpace>()
        .init_resource::<camera::CameraControlData>()
        .add_plugins(DefaultPlugins/*.set(WindowPlugin {
            primary_window: Some(Window {
                title: "OpenCAD".to_string(),
                ..default()
            }),
            ..default()
        })*/)
        .add_plugin(
            MaterialPlugin::<CadMaterial>::default(),
        )
        .add_plugin(EguiPlugin)
        .add_system(ui::ui_system)
        .add_system(camera::camera_controller_system)
        .add_system(cad_shade)
        .add_startup_system(setup)
        .add_startup_system(camera::setup_camera_controller)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cad_materials: ResMut<Assets<CadMaterial>>,
) {
    // Camera
    /*commands.spawn(Camera2dBundle::default());

    commands.spawn(NodeBundle {

    })*/

    let material = cad_materials.add(CadMaterial {});

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
    camera_control_data: Res<camera::CameraControlData>
) {
    camera_control_data.transform.forward();
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CadMaterial {
    //#[uniform(0)]
    //cameraLookVec: Vec3,
}

impl Material for CadMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/cad_shader.wgsl".into()
    }
}