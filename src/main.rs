use rand::Rng;
use std::{cmp::Ordering, io};
use std::io;

fn main() {
  let secret_number = rand::thread_rng().gen_range(1..=10);
}