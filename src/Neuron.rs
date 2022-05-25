#[derive(Debug)]
pub struct Neuron {
  weights: Vec<f32>,
  bias: f32,
}

impl Neuron {
  pub fn new(inputs_count: i32) -> Neuron {
    let mut neuron = Neuron {
      weights: Vec::new(),
      bias: rand::thread_rng().gen_range(-1.0..1.0),
    };

    for i in 0..inputs_count {
      neuron.weights.push(rand::thread_rng().gen_range(-1.0..1.0));
    }

    neuron
  }
}
