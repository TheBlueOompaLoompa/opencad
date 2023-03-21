use parametric_engine::{sketch};

fn main() {
    let mut parent_face = parametric_engine::sketch::Face{edges: vec![]};
    let test_sketch = sketch::create_sketch(&mut parent_face);
}
