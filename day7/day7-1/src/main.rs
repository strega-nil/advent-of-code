use std::fs::File;
use std::io::Read;

fn isabba(c: &[u8]) -> bool {
  c[0] == c[3]
  && c[1] == c[2]
  && c[0] != c[1]
}

fn main() {
  let mut inp = Vec::new();
  File::open("../input.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut num_tls = 0;
  for line in inp.lines() {
    let line = line.as_bytes();
    let mut is_tls = false;
    let mut is_hyper = false;

    {
      let mut i = 0;
      while i < (line.len() - 3) {
        if line[i] == b'[' {
          is_hyper = true;
          i += 1;
          continue;
        } else if line[i] == b']' {
          is_hyper = false;
          i += 1;
          continue;
        }

        if isabba(&line[i..i + 4]) {
          if is_hyper {
            is_tls = false;
            break;
          } else {
            is_tls = true;
          }
        }
        i += 1;
      }
    }
    if is_tls {
      num_tls += 1;
    }
  }

  println!("{}", num_tls);
}
