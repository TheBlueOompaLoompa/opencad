use std::vec::Vec;

pub struct SketchBundle {
    pub parent_face: Face,
}

impl SketchBundle {
    pub fn create_sketch<'a>(parent_face: Face) -> SketchBundle {
        SketchBundle { parent_face: parent_face }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>
}
impl Point {
    pub const fn splat(n: f64) -> Self {
        return Self { x: n, y: n, z: Some(n) }
    }

    pub const fn splat_2d(n: f64) -> Self {
        return Self { x: n, y: n, z: None }
    }

    pub const ZERO: Self = Self::splat(0.0);
    pub const ZERO_2D: Self = Self::splat_2d(0.0);
}

pub struct Edge {
    pub points: [Point; 2],
}

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

pub trait Constraint {
    fn for_point(&mut self, _point: Point) {}
    fn for_edge(&self, _edge: Edge) {}
    fn for_face(&self, _face: Face) {}
}

pub struct Coincident<'a>{
    pub other_point: &'a mut Point
}

impl Constraint for Coincident<'_> {
    fn for_point(&mut self, point: Point) {
        self.other_point.x = point.x;
        self.other_point.y = point.y;
    }
}

#[test]
fn coincident_constraint() {
    let p1 = Point::ZERO_2D;
    let mut p2 = Point::splat_2d(1.0);

    let mut t = Coincident { other_point: &mut p2 };
    t.for_point(p1);

    assert!(p1 == p2);
}