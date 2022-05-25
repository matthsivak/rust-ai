use super::neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
  pub neurons: Vec<Neuron>,
  pub layer: i16,
}

impl Layer {
  pub fn new(inputs_count: i32, neurons_count: i32, layer_number: i16) -> Layer {
    let mut layer = Layer {
      neurons: Vec::new(),
      layer: layer_number,
    };

    for _i in 0..neurons_count {
      layer.neurons.push(Neuron::new(inputs_count, layer_number));
    }

    layer
  }

  pub fn feed_forward(&mut self, inputs: &Vec<f32>) -> Vec<f32> {
    let mut outputs = Vec::new();

    for neuron in self.neurons.iter_mut() {
      outputs.push(neuron.feed_forward(inputs));
    }

    outputs
  }

  pub fn mutate(&mut self, rate: f32) {
    for neuron in self.neurons.iter_mut() {
      neuron.mutate(rate);
    }
  }
}
