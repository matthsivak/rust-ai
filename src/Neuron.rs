use rand::Rng;

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

    for _i in 0..inputs_count {
      neuron.weights.push(rand::thread_rng().gen_range(-1.0..1.0));
    }

    neuron
  }

  pub fn feed_forward(&mut self, inputs: &Vec<f32>) -> i32 {
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
    for (let i = 0; i < this.weights.length; i++) {
      if rand::thread_rng().gen_range(0.0..1.0) < rate {
        self.weights[i] = rand::thread_rng().gen_range(-1.0..1.0);
      }
    }
  }
}

pub fn activate(x: f32) -> i32 {
  match x {
    x if x < 0.0 => return -1,
    x if x > 0.0 => return 1,
    _ => return 0,
  }
}
