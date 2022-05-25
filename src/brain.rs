use super::layer::Layer;

#[derive(Debug)]
pub struct Brain {
  pub layers: Vec<Layer>,
  pub brain_map: Vec<i32>,
}

impl Brain {
  pub fn new(brain_map: Vec<i32>) -> Brain {
    let mut brain = Brain {
      layers: Vec::new(),
      brain_map: brain_map,
    };

    for i in 0..brain.brain_map.len() {
      brain
        .layers
        .push(Layer::new(brain.brain_map[i], brain.brain_map[i]));
    }

    brain
  }
}
