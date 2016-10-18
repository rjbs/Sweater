use std::env;
use std::io;
use std::io::{BufReader,BufWriter};
use std::io::prelude::*;
use std::fs::File;

fn main() {
  let argc = env::args().count();
  if argc > 2 {
    println!("usage: compress <filename>");
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

  let mut lastc = None;
  let mut runlen = 0;
  let mut chunkbuf = Vec::new();

  for res in reader.bytes() {
    let b = res.unwrap();

    if let Some(n) = lastc {
      if n == b {
        runlen = runlen + 1;
        if runlen == 255 {
          // Same character 255 times in a row, means it must also be the whole
          // chunk buffer.
          chunkbuf.truncate(0);

          stream.write(&[0]).unwrap();
          stream.write(&[b]).unwrap();
          stream.write(&[runlen as u8]).unwrap();

          lastc = None;
          runlen = 0;
          continue;
        }
      } else {
        if runlen >= 5 {
          let clen = chunkbuf.len();
          if clen > runlen {
            chunkbuf.truncate(clen - runlen as usize);
            stream.write(&[ clen as u8 - runlen as u8 ]).unwrap();
            stream.write(&chunkbuf).unwrap();
          }
          chunkbuf.truncate(0);

          stream.write(&[ 0, n, runlen as u8]).unwrap();
        }

        runlen = 1;
      }
    }

    if chunkbuf.len() == 255 {
      stream.write(&[ chunkbuf.len() as u8 ]).unwrap();
      stream.write(&chunkbuf).unwrap();
      chunkbuf.truncate(0);
    }

    chunkbuf.push(b);
    lastc = Some(b);
  }

  stream.write(&[ chunkbuf.len() as u8 ]).unwrap();
  stream.write(&chunkbuf).unwrap();

  stream.flush().unwrap();
}
