use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let mut args = env::args();
  args.next();

  let argc = env::args().count();

  if argc != 3 {
    panic!("usage: cp SRC DST");
  }

  println!("copying {} to {}", "src", "dst");

  return ();
}
