use std::fs::File;
use std::io::Read;

enum Command {
  Rect(usize, usize),
  RotCol(usize, usize),
  RotRow(usize, usize),
}

impl Command {
  fn parse(mut s: &str) -> Self {
    if s.starts_with("rect") {
      s = &s[5..];
      let mut end_n1 = 0;
      let mut n2 = 0;
      for (i, byte) in s.bytes().enumerate() {
        if !(byte as char).is_digit(10) {
          end_n1 = i;
          n2 = i + 1;
          break;
        }
      }
      let x_coord = s[..end_n1].parse().unwrap();
      let y_coord = s[n2..].parse().unwrap();
      Command::Rect(x_coord, y_coord)
    } else if s.starts_with("rotate column x=") {
      s = &s[16..];
      let mut end_n1 = 0;
      let mut n2 = 0;
      for (i, byte) in s.bytes().enumerate() {
        if !(byte as char).is_digit(10) {
          end_n1 = i;
          n2 = i + 4;
          break;
        }
      }
      let which = s[..end_n1].parse().unwrap();
      let by = s[n2..].parse().unwrap();
      Command::RotCol(which, by)
    } else if s.starts_with("rotate row y=") {
      s = &s[13..];
      let mut end_n1 = 0;
      let mut n2 = 0;
      for (i, byte) in s.bytes().enumerate() {
        if !(byte as char).is_digit(10) {
          end_n1 = i;
          n2 = i + 4;
          break;
        }
      }
      let which = s[..end_n1].parse().unwrap();
      let by = s[n2..].parse().unwrap();
      Command::RotRow(which, by)
    } else {
      panic!("invalid command: {}", s);
    }
  }
}

fn rotate_row(x: &mut [bool], by: usize) {
  let mut new = [false; 50];
  for i in 0..x.len() {
    if by <= i {
      new[i] = x[i - by];
    } else {
      new[i] = x[x.len() + i - by];
    }
  }
  for i in 0..x.len() {
    x[i] = new[i];
  }
}

fn main() {
  let mut inp = Vec::new();
  File::open("input.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut screen = [[false; 50]; 6];

  for command in inp.lines() {
    match Command::parse(command) {
      Command::Rect(x, y) => {
        for x in 0..x {
          for y in 0..y {
            screen[y][x] = true;
          }
        }
      }
      Command::RotCol(which, by) => {
        let mut new = [false; 6];
        for i in 0..screen.len() {
          if by <= i {
            new[i] = screen[i - by][which];
          } else {
            new[i] = screen[screen.len() + i - by][which];
          }
        }
        for i in 0..screen.len() {
          screen[i][which] = new[i];
        }
      }
      Command::RotRow(which, by) => {
        let row = &mut screen[which];
        rotate_row(row, by);
      }
    }
  }
  let mut acc = 0;
  for y in &screen[..] {
    for x in &y[..] {
      if *x {
        acc += 1;
        print!(":");
      } else {
        print!(" ");
      }
    }
    println!("");
  }
  println!("{}", acc);
}
