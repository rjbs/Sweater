use std::env;
use std::io;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let argc = env::args().count();
  if argc > 2 {
    println!("usage: detab <filename>; got {} args", argc);
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

fn is_tabstop (linepos: &usize) -> bool {
  return linepos % 8 == 0;
}

fn do_file <T: Read> (reader: &mut T) {
  let output = io::stdout();
  let mut handle = output.lock();
  let mut stream = BufWriter::new(&mut handle);

  let mut linepos = 0;

  for res in reader.bytes() {
    let b = res.unwrap();
    if b == b'\t' { // literal tab <	>
      stream.write(&[b' ']).unwrap();
      linepos += 1;

      while ! is_tabstop(&linepos) {
        stream.write(&[b' ']).unwrap();
        linepos += 1;
      }
      continue;
    }

    if b == b'\n' {
      linepos = 0;
    } else {
      linepos += 1;
    }

    stream.write(&[b]).unwrap();
  }

  stream.flush().unwrap();
}
