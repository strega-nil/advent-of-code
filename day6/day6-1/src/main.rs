use std::fs::File;
use std::io::Read;

fn main() {
  let mut inp = Vec::new();
  File::open("../input.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut arr = [[0; 26]; 8];
  for s in inp.split_whitespace() {
    for (i, x) in s.bytes().enumerate() {
      arr[i][(x - b'a') as usize] += 1;
    }
  }

  let mut commons = [0; 8];
  for (i, pos) in arr.iter().enumerate() {
    let mut common = (b'a', pos[0]);
    for (i, c) in pos.iter().enumerate() {
      if *c > common.1 && *c != 0 {
        common.0 = i as u8 + b'a';
        common.1 = *c;
      }
    }
    commons[i] = common.0;
  }
  println!("{}", std::str::from_utf8(&commons).unwrap());
}
