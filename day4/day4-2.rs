use std::fs::File;
use std::io::Read;

fn isalpha(c: u8) -> bool {
  c >= b'a' && c <= b'z'
}

fn isdigit(c: u8) -> bool {
  c >= b'0' && c <= b'9'
}

fn shift(mut c: u8, by: u32) -> u8 {
  if c == b' ' {
    return c;
  }
  c += (by % 26) as u8;
  if c > b'z' {
    c -= 26;
  }
  c
}

fn foo(chars: [u32; 26]) -> [u8; 5] {
  let mut commons = [(0u8, 0u32); 5];
  for i in 0..26 {
    let current = chars[i as usize];
    for j in 0..5 {
      if current > commons[j].1 {
        let mut k = 4;
        while k > j {
          commons[k as usize] = commons[(k - 1) as usize];
          k -= 1;
        }
        commons[j] = (i + b'a', chars[i as usize]);
        break;
      }
    }
  }
  commons.sort_by(|el1, el2| if el1.1 == el2.1 {
    (el1.0).cmp(&el2.0)
  } else {
    (el1.1).cmp(&el2.1).reverse()
  });
  let mut ret = [0; 5];
  for i in 0..5 {
    ret[i] = commons[i].0;
  }
  ret
}

fn main() {
  let mut inp = Vec::new();
  File::open("day4.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut sectorid = 0;
  for rm in inp.split_whitespace() {
    let room = rm.bytes().collect::<Vec<_>>();
    let mut i = 0;
    let mut v = Vec::new();
    loop {
      match room[i] {
        b'-' => {
          v.push(b' ');
          i += 1;
        },
        c if isalpha(c) => {
          v.push(c);
          i += 1;
        },
        c if isdigit(c) => {
          let mut s = String::new();
          while isdigit(room[i]) {
            s.push(room[i] as char);
            i += 1;
          }
          let sby = s.parse::<u32>().unwrap();
          println!(
            "{} = {}",
            v.into_iter().map(|c| shift(c, sby) as char).collect::<String>(),
            sby
          );
          break;
        },
        _ => panic!(),
      }
    }
  }
}
