use std::env;
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
  let mut outbuf = [ 0u8; 4096 ];

  let mut outfile = File::create("output").unwrap();

  let mut outpos  = 0;

  loop {
    let len = file.read(&mut inbuf).unwrap();
    if len == 0 { break }

    let mut linepos = 0;

    // Flush output buffer...
    //  * when output buffer full
    //  * when within 8 of full and a tab is reached, before appending
    //  * when input is exhausted

    for b in &inbuf[0 .. len+1] {
      if outpos > 2048 {
        outfile.write(&outbuf[ 0 .. outpos-1 ]).unwrap();
        outpos = 0;
      }

      if b == &b'\t' { // literal tab <	>
        outbuf[outpos] = b' ';
        outpos += 1;
        while ! is_tabstop(&outpos) {
          outbuf[outpos] = b' ';
          outpos += 1;
        }
        continue;
      }

      if b == &b'\n' {
        linepos = 0;
      } else {
        linepos += 1;
      }

      outbuf[outpos] = *b;
      outpos  += 1;
      println!(" {} --> {}", linepos, b);
    }
  }

  if outpos > 0 {
    outfile.write(&outbuf[ 0 .. outpos-1 ]).unwrap();
  }

  outfile.flush().unwrap();
}
