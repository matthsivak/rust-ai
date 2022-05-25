use super::neuron::Neuron;

#[derive(Debug)]
pub struct Layer {
  pub neurons: Vec<Neuron>,
}

impl Layer {
  pub fn new(inputs_count: i32, neurons_count: i32) -> Layer {
    let mut layer = Layer {
      neurons: Vec::new(),
    };

    for _i in 0..neurons_count {
      layer.neurons.push(Neuron::new(inputs_count));
    }

    layer
  }

  pub fn (&self, inputs: &Vec<f32>) -> Vec<f32> {
    let mut outputs = Vec::new();

    for neuron in &self.neurons {
      outputs.push(neuron.feed_forward(inputs));
    }

    outputs
  }

  pub fn mutate(&mut self, rate: f32) {
    for neuron in self.neurons {
      neuron.mutate(rate);
    }
  }
}
