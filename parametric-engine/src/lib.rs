use godot::prelude::*;
use godot::engine::{Sprite2D, Sprite2DVirtual};

struct ParametricEngine;

#[gdextension]
unsafe impl ExtensionLibrary for ParametricEngine {}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Sketch {
    #[base]
    node: Base<Node>
}

#[godot_api]
impl NodeVirtual for Sketch {
    fn init(node: Base<Node>) -> Self { Self { node }}
}

#[godot_api]
impl Sketch {
    #[func]
    fn recalculate_constraints(&mut self) {

    }

    #[signal]
    fn calculated();
}

#[derive(GodotClass)]
#[class(base=Node)]
struct Constraint {
    #[base]
    node: Base<Node>
}