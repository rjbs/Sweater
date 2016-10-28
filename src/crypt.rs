use std::env;
use std::io;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let argc = env::args().count();
  if argc != 3 {
    println!("usage: detab <key> <filename>; got {} args", argc);
    std::process::exit(1);
  }

  let mut args = env::args();
  args.next();

  let key      = args.next().unwrap();
  let filename = args.next().unwrap();

  match File::open(&filename) {
    Ok(file) => {
      let mut reader = BufReader::new(file);
      do_file(key, &mut reader);
    },
    Err(err) => println!("{}: {}", &filename, err)
  }

  return ();
}

fn do_file <T: Read> (key: String, reader: &mut T) {
  let mut keybytes = key.as_bytes().iter().cycle();

  let output = io::stdout();
  let mut handle = output.lock();
  let mut stream = BufWriter::new(&mut handle);

  println!("{}", key);

  for res in reader.bytes() {
    let b = res.unwrap();
    let k = keybytes.next().unwrap();

    stream.write(&[ b ^ k ]).unwrap();
  }

  stream.flush().unwrap();
}
