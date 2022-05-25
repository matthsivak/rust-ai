pub mod brain;
pub mod layer;
pub mod neuron;

fn main() {
  let mut brain = brain::Brain::new(Vec::from([2, 2, 1]));

  // println!("{:?}", neuron::Neuron::feed_forward(&Vec::from([1.0, 1.0]), &brain.layers[0].neurons[0]));
  println!("{:?}", brain.layers[0].neurons[0].feed_forward(&Vec::from([1.0, 1.0])));
  // println!("{:?}", brain);
}
