use std::os;
use std::rand::{thread_rng, Rng};

fn main() {
  let all_args = os::args();
  let mut args = all_args[1..].to_vec();

  let mut rng = thread_rng();
  rng.shuffle(args.as_mut_slice());

  for arg in args.iter() {
    print!("{} ", arg);
  }
  println!("");
}
