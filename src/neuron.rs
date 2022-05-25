use rand::Rng;

#[derive(Debug, Clone)]
pub struct Neuron {
  weights: Vec<f32>,
  bias: f32,
  layer: i16,
}

impl Neuron {
  pub fn new(inputs_count: i32, layer: i16) -> Neuron {
    let mut neuron = Neuron {
      weights: Vec::new(),
      bias: rand::thread_rng().gen_range(-1.0..1.0),
      layer,
    };

    for _i in 0..inputs_count {
      neuron.weights.push(rand::thread_rng().gen_range(-1.0..1.0));
    }

    neuron
  }

  pub fn feed_forward(&mut self, inputs: &Vec<f32>) -> f32 {
    let mut sum = 0.0;

    for i in 0..inputs.len() {
      sum += inputs[i] * self.weights[i];
    }

    sum += self.bias;

    activate(sum)
  }
  // pub fn feed_forward(inputs: &Vec<f32>, neuron: &Neuron) -> f32 {
  //   let mut sum = 0.0;

  //   for i in 0..inputs.len() {
  //     sum += inputs[i] * neuron.weights[i];
  //   }

  //   sum += neuron.bias;

  //   sum

  // }

  pub fn mutate(&mut self, rate: f32) {
    for w in self.weights.iter_mut() {
      if rand::thread_rng().gen_range(0.0..1.0) < rate {
        *w = rand::thread_rng().gen_range(-1.0..1.0);
      }
    }
  }
}

pub fn activate(x: f32) -> f32 {
  match x {
    x if x < 0.0 => return -1.0,
    x if x > 0.0 => return 1.0,
    _ => return 0.0,
  }
}
