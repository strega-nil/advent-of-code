use std::fs::File;
use std::io::Read;

fn main() {
  let mut inp = Vec::new();
  File::open("day3.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut swhite = inp.split_whitespace();
  let mut triangles = Vec::new();
  'lop: loop {
    let mut first = [0; 3];
    let mut second = [0; 3];
    let mut third = [0; 3];
    for x in 0..3 {
      first[x] = if let Some(n) = swhite.next() {
        n.parse::<u32>().unwrap()
      } else if x == 0 {
        break 'lop;
      } else {
        panic!();
      };
      second[x] = if let Some(n) = swhite.next() {
        n.parse::<u32>().unwrap()
      } else {
        panic!();
      };
      third[x] = if let Some(n) = swhite.next() {
        n.parse::<u32>().unwrap()
      } else {
        panic!();
      };
    }
    triangles.push((first[0], first[1], first[2]));
    triangles.push((second[0], second[1], second[2]));
    triangles.push((third[0], third[1], third[2]));
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
