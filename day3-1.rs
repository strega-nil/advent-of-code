use std::fs::File;
use std::io::Read;

fn main() {
  let mut inp = Vec::new();
  File::open("day3.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut swhite = inp.split_whitespace();
  let mut triangles = Vec::new();
  loop {
    let first = if let Some(n) = swhite.next() {
      n.parse::<u32>().unwrap()
    } else {
      break;
    };
    let second = if let Some(n) = swhite.next() {
      n.parse::<u32>().unwrap()
    } else {
      panic!();
    };
    let third = if let Some(n) = swhite.next() {
      n.parse::<u32>().unwrap()
    } else {
      panic!();
    };
    triangles.push((first, second, third));
  }
  let mut count = 0;
  for tri in &triangles {
    if tri.0 + tri.1 <= tri.2 {
      count += 1;
    } else if tri.1 + tri.2 <= tri.0 {
      count += 1;
    } else if tri.2 + tri.0 <= tri.1 {
      count += 1;
    }
  }
  println!("{}", triangles.len() - count);
}
