use std::env;
use std::io;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let argc = env::args().count();
  if argc > 2 {
    println!("usage: entab <filename>");
    std::process::exit(1);
  }

  let mut args = env::args();
  args.next();
  let filename = args.next().unwrap();
  let openres  = File::open(&filename);

  if let Ok(file) = openres {
    let mut reader = BufReader::new(file);
    do_file(&mut reader);
  } else {
    match openres.err() {
      Some(errstr) => println!("{}: {}", &filename, errstr),
      _ => panic!("wtf")
    }
  }

  return ();
}

fn do_file <T: Read> (reader: &mut T) {
  let output = io::stdout();
  let mut handle = output.lock();
  let mut stream = BufWriter::new(&mut handle);

  // read a file from disk, collecting chunks of as long as possible
  // ...but when we see runs of n chars, we cut a chunk short and emit a run
  // sequence

  // The file is made up of two kinds of entity:
  //  * (RUN-INDICATOR, CHARACTER, RUN-LENGTH)
  //  * (CHUNK-LENGTH, CHARACTERS)
  //
  // So, we use 1-255 as chunk lengths, meaning 0 is free as a run indicator.
  // A run has overhead of 3 (indicator, character, next chunk start) versis
  // just emitting a character, so it's only worth doing if the run is 5 long
  // or better.  (At four, we break even.)

  // let mut lastc;
  // let mut chunkbuf = Vec::new();

  for res in reader.bytes() {
    let b = res.unwrap();

    stream.write(&[ 1 ]).unwrap();
    stream.write(&[ b ]).unwrap();
  }

  stream.flush().unwrap();
}
