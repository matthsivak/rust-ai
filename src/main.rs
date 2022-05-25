pub mod brain;
pub mod layer;
pub mod neuron;

fn main() {
  let mut brain = brain::Brain::new(Vec::from([10, 64, 256, 256, 64, 5]));

  println!("{:?}", brain.feed_forward(&Vec::from([0.2, -0.2])));

  // println!("{:?}", brain);
}
