use std::env;
use std::io;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let argc = env::args().count();
  if argc > 2 {
    println!("usage: decompress <filename>");
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

  // The file is made up of two kinds of entity:
  //  * (RUN-INDICATOR, CHARACTER, RUN-LENGTH)
  //  * (CHUNK-LENGTH, CHARACTERS)

  let mut buf = [ 0; 1 ];

  loop {
    let just_read = reader.read(&mut buf[..]).unwrap();

    // eof!  we're done here
    if just_read == 0 { break }

    let mut chunk = [0u8; 255];

    if buf[0] == 0 {
      let mut pair = [ 0u8; 2 ];
      reader.read_exact(&mut pair[..]).unwrap();

      io::repeat(pair[0]).read_exact(&mut chunk[0 .. pair[1] as usize]).unwrap();
      stream.write(&chunk[0 .. pair[1] as usize]).unwrap();
    } else {
      let mut cread = reader.take(buf[0] as u64);

      cread.read_exact(&mut chunk[0 .. buf[0] as usize]).unwrap();
      stream.write(&chunk[0 .. buf[0] as usize]).unwrap();
      // else { read char-many bytes; write them to stream }
    }
  }
}
