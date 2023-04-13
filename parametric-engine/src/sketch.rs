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

trait Part {
    fn apply_constraint(&mut self, constraint: Box<dyn Constraint>);
}

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

impl Face { // TODO: Create preset XY XZ YZ Planes
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
}

impl Part for Point {
    fn apply_constraint(&mut self, constraint: Box<dyn Constraint>) {
        constraint.for_point(self);
    }
}

trait Constraint {
    fn for_point(&mut self, point: Point) {}
    fn for_edge(&mut self, edge: Edge) {}
    fn for_face(&mut self, face: Face) {}
}

struct Coincident<'a> {
    other_point: &'a Point
}

impl Constraint for Coincident<'_> {
    fn for_point(&mut self, point: Point) {
        self.other_point.x = point.x;
        self.other_point.y = point.y;
    }
}