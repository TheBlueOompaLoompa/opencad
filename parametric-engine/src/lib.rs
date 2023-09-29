#![allow(dead_code)]
use godot::prelude::*;
use godot::engine::Node;

pub mod sketch;

struct ParametricEngine;

#[gdextension]
unsafe impl ExtensionLibrary for ParametricEngine {}


#[derive(GodotClass)]
#[class(base=Node)]
struct Sketch {
    #[base]
    node: Base<Node>
}

#[godot_api]
impl NodeVirtual for Sketch {
    fn init(node: Base<Node>) -> Self { Self { node } }

    /*fn physics_process(&mut self, delta: f64) {
        // In GDScript, this would be: 
        // rotation += angular_speed * delta
        
        self.sprite.rotate((self.angular_speed * delta) as f32);
        // The 'rotate' method requires a f32, 
        // therefore we convert 'self.angular_speed * delta' which is a f64 to a f32
    }*/
}

#[derive(GodotClass)]
#[class(base=Node)]
struct Constraint {
    #[base]
    node: Base<Node>
}