use std::fs::File;
use std::io::Read;

fn isaba(c: &[u8]) -> bool {
  c[0] == c[2]
  && c[0] != c[1]
}

fn aba_equiv(xs: &[u8], ys: &[u8]) -> bool {
  v1[0] == v2[1] && v2[0] == v1[1]
}

fn main() {
  let mut inp = Vec::new();
  File::open("../input.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut num_ssl = 0;
  for line in inp.lines() {
    let line = line.as_bytes();

    let mut is_hyper = false;
    let mut hyper_aba = vec![];
    let mut nonh_aba = vec![];

    {
      let mut i = 0;
      while i < (line.len() - 2) {
        if line[i] == b'[' {
          is_hyper = true;
          i += 1;
          continue;
        } else if line[i] == b']' {
          is_hyper = false;
          i += 1;
          continue;
        }

        if isaba(&line[i..i + 3]) {
          if is_hyper {
            hyper_aba.push(line[i..i + 3].to_owned())
          } else {
            nonh_aba.push(line[i..i + 3].to_owned())
          }
        }
        i += 1;
      }
    }
    'h: for h in &hyper_aba {
      for n in &nonh_aba {
        if aba_equiv(h, n) {
          num_ssl += 1;
          break 'h;
        }
      }
    }
  }

  println!("{}", num_ssl);
}
