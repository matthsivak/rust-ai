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

    for i in 1..brain.brain_map.len() {
      brain.layers.push(Layer::new(
        brain.brain_map[i - 1],
        brain.brain_map[i],
        i as i16,
      ));
    }

    brain
  }

  //  public feedForward(inputs: number[]) {
  //   let outputs: number[] = inputs;

  //   for (let layer of this.layers) {
  //     outputs = layer.feedForward(outputs) as number[];
  //   }

  //   return outputs;
  // }

  pub fn feed_forward(&mut self, inputs: &Vec<f32>) -> Vec<f32> {
    let mut outputs: Vec<f32> = inputs.clone();

    for layer in self.layers.iter_mut() {
      outputs = layer.feed_forward(&outputs);
    }

    outputs
  }

  pub fn mutate(&mut self, rate: f32) {
    for layer in self.layers.iter_mut() {
      layer.mutate(rate);
    }
  }
}
