use std::vec::Vec;

pub fn create_sketch<'a>(parent_face: &'a mut Face) -> Sketch {
    Sketch { parent_face: parent_face, faces: vec![], constraints: vec![] }
}

pub struct Sketch <'a> {
    pub parent_face: &'a mut Face,
    pub faces: Vec<Face>,
    pub constraints: Vec<Constraint>,
}

pub struct Constraint {
    pub c_type: ConstraintType, // Sorry, can't use the name type
    pub targets: Vec<TargetTypes>,
}

pub enum ConstraintType {
    Horizontal,
    Vertical,
}

pub enum TargetTypes {
    Point,
    Edge,
    Face,
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>
}

pub struct Edge {
    pub points: [Point; 2],
}

pub struct Face {
    pub edges: Vec<Edge>,
}

// TODO: Create preset XY XZ YZ Planes