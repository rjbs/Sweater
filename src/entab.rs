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

fn is_tabstop (linepos: &usize) -> bool {
  return linepos % 8 == 0;
}

fn do_file <T: Read> (reader: &mut T) {
  let output = io::stdout();
  let mut handle = output.lock();
  let mut stream = BufWriter::new(&mut handle);

  // read a file from disk, turning runs of spaces into tabs
  // read file line by line, resetting line state on new lines
  // when we move from not-a-tabstop linepos to a tabstop linepos, replace all
  // the leading spaces with a single tab

  let mut linepos = 0;

  // only needs to be as long as the space between tabstops
  let mut spacebuf = Vec::new();

  for res in reader.bytes() {
    let b = res.unwrap();

    if is_tabstop(&linepos) && spacebuf.len() > 0 {
      stream.write(&[ b'\t' ]).unwrap();
      spacebuf.truncate(0);
    }

    if b == b'\n' {
      stream.write(&spacebuf).unwrap();
      stream.write(&[ b'\n' ]).unwrap();
      spacebuf.truncate(0);
      linepos = 0;
      continue;
    }

    linepos += 1;

    if b == b' ' {
      spacebuf.push(b);
    } else {
      stream.write(&[b]).unwrap();
    }

  }

  stream.write(&spacebuf).unwrap();

  stream.flush().unwrap();
}
