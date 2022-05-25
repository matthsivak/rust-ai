use rand::Rng;

#[derive(Debug)]
struct Neuron {
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

#[derive(Debug)]
struct Layer {
  neurons: Vec<Neuron>,
}

impl Layer {
  pub fn new(inputs_count: i32, neurons_count: i32) -> Layer {
    let mut layer = Layer {
      neurons: Vec::new(),
    };

    for i in 0..neurons_count {
      layer.neurons.push(Neuron::new(inputs_count));
    }

    layer
  }
}

#[derive(Debug)]
struct Brain {
  layers: Vec<Layer>,
  brain_map: Vec<i32>,
}

impl Brain {
  pub fn new(brain_map: Vec<i32>) -> Brain {
    let mut brain = Brain {
      layers: Vec::new(),
      brain_map: brain_map.to_vec(),
    };

    for i in 0..brain_map.len() {
      brain.layers.push(Layer::new(brain_map[i], brain_map[i]));
    }

    brain
  }
}

fn main() {
  let mut brain = Brain::new(vec![2, 2, 1]);

  println!("{:?}", brain);
}
