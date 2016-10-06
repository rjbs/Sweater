use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let argc = env::args().count();

  if argc != 3 {
    panic!("usage: cp SRC DST");
  }

  let mut args = env::args();
  args.next();

  let src = args.next().unwrap();
  let dst = args.next().unwrap();

  cp(src, dst);

  return ();
}

fn cp(src: String, dst: String) -> () {
  println!("copy {} to {}", src, dst);

  let mut input  = File::open(src).unwrap();
  let mut output = File::create(dst).unwrap();

  let mut buf = [ 0; 4096 ];
  while let Ok(len) = input.read(&mut buf) {
    if len == 0 {
      // close input, output
      return ();
    }

    output.write(&buf[0 .. len]).unwrap();
  }

  return ();
}
