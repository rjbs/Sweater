use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

fn main() {
  let mut args = env::args();
  args.next();

  let argc = env::args().count();

  if argc > 1 {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;

    for argument in args {
      match do_filename(&argument) {
        Ok((f_lines, f_words, f_bytes)) => {
          lines += f_lines;
          words += f_words;
          bytes += f_bytes;
        },
        Err(e) => {
          println!("wc: {}: {}", argument, e.description());
        }
      }
    }

    if argc > 2 {
      println!("{:8} {:7} {:7} total", lines, words, bytes);
    }
  } else {
    let stdin = io::stdin();
    let mut stdin_reader = stdin.lock();

    match do_file(&mut stdin_reader, &String::from("stdin")) {
      Ok((f_lines, f_words, f_bytes)) => {
        println!("{:8} {:7} {:7}", f_lines, f_words, f_bytes);
      },
      Err(e) => {
        println!("wc: {}", e.description());
      }
    }
  }

  return ();
}

fn do_file (file: &mut Read, label: &String) -> io::Result<(u32, u32, u32)> {
  let mut buf = [ 0; 4096 ];

  let mut lines = 0;
  let mut words = 0;
  let mut bytes = 0;

  let mut in_word = false;

  loop {
    let just_read = try!( file.read(&mut buf[..]) );
    if just_read == 0 { break }

    bytes += just_read as u32;
    for b in buf.iter().take(just_read) {
      if b == &b'\n' { lines += 1 }

      if b == &b' ' || b == &b'\t' || b == &b'\n' {
        if in_word { words += 1; in_word = false }
      } else {
        if ! in_word { in_word = true }
      }
    }
  }

  if in_word { words += 1 }

  println!("{:8} {:7} {:7} {}", lines, words, bytes, &label);
  return Ok((lines, words, bytes));
}

fn do_filename (filename: &String) -> io::Result<(u32, u32, u32)> {
  let mut file = try!( File::open(filename) );

  let (lines, words, bytes) = try!( do_file(&mut file, filename) );
  return Ok((lines, words, bytes));
}
