use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

//use crate::camera::{CameraControlData, OriginalCameraTransform};

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.top =  egui::TopBottomPanel::top("main_tools")
        .default_height(64.0)
        .show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, "File", |ui| {
                    if ui.button("New").clicked() {

                    } else if ui.button("Save").clicked() {

                    } else if ui.button("Quit").clicked() {
                        std::process::exit(0);
                    }
                });
            });
        })
        .response
        .rect
        .height();
}