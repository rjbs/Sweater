use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

// read a file from disk, turning runs of spaces into tabs
// read file line by line, resetting line state on new lines
// when we move from not-a-tabstop linepos to a tabstop linepos, replace all
// the leading spaces with a single tab

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

  if let Ok(mut file) = openres {
    do_file(&mut file);
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

fn do_file (file: &mut File) {
  let mut inbuf  = [ 0u8; 4096 ];

  let output = io::stdout();
  let mut handle = output.lock();
  let mut stream = BufWriter::new(&mut handle);

  loop {
    let len = file.read(&mut inbuf).unwrap();
    if len == 0 { break }

    let mut linepos = 0;

    for b in &inbuf[0 .. len] {
      if b == &b'\t' { // literal tab <	>
        stream.write(&[b' ']).unwrap();
        linepos += 1;

        while ! is_tabstop(&linepos) {
          stream.write(&[b' ']).unwrap();
          linepos += 1;
        }
        continue;
      }

      if b == &b'\n' {
        linepos = 0;
      } else {
        linepos += 1;
      }

      stream.write(&[*b]).unwrap();
    }
  }

  stream.flush().unwrap();
}
