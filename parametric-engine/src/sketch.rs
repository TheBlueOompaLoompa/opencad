use bevy::prelude::*;
use std::vec::Vec;

#[derive(Bundle)]
pub struct SketchBundle {
    pub parent_face: Face,
}

impl SketchBundle {
    pub fn create_sketch<'a>(parent_face: Face) -> SketchBundle {
        SketchBundle { parent_face: parent_face }
    }
}

// TODO: Change componets to sub bundles

#[derive(Component)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>
}

#[derive(Component)]
pub struct Edge {
    pub points: [Point; 2],
}

#[derive(Component)]
pub struct Face {
    pub edges: Vec<Edge>,
}











/*
impl Face {
    pub fn xy_base() -> Face {
        return Face{edges: vec![
            Edge{points:[
                Point{x: -1.0, y: -1.0, z: Some(0.0)},
                Point{x: 1.0, y: -1.0, z: Some(0.0)},
            ]},
            Edge{points:[
                Point{x: 1.0, y: -1.0, z: Some(0.0)},
                Point{x: 1.0, y: 1.0, z: Some(0.0)},
            ]},
            Edge{points:[
                Point{x: 1.0, y: 1.0, z: Some(0.0)},
                Point{x: -1.0, y: 1.0, z: Some(0.0)},
            ]},
            Edge{points:[
                Point{x: -1.0, y: 1.0, z: Some(0.0)},
                Point{x: -1.0, y: -1.0, z: Some(0.0)},
            ]},
        ]};
    }
}*/

// TODO: Create preset XY XZ YZ Planes