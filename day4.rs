use std::fs::File;
use std::io::Read;

fn isalpha(c: u8) -> bool {
  c >= b'a' && c <= b'z'
}

fn isdigit(c: u8) -> bool {
  c >= b'0' && c <= b'9'
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
    let mut set = [0; 26];
    let mut common = [0; 5];
    let mut sector_id = 0;
    loop {
      match room[i] {
        b'-' => {i += 1;},
        c if isalpha(c) => {
          set[(c - b'a') as usize] += 1;
          i += 1;
        },
        c if isdigit(c) => {
          let mut s = String::new();
          while isdigit(room[i]) {
            s.push(room[i] as char);
            i += 1;
          }
          i += 1;
          common[0] = room[i];
          common[1] = room[i + 1];
          common[2] = room[i + 2];
          common[3] = room[i + 3];
          common[4] = room[i + 4];
          sector_id = s.parse::<u32>().unwrap();
          if foo(set) == common {
            sectorid += sector_id;
          } else {
            println!("{:?}", rm);
            println!("{:?}\n{:?}", foo(set), common);
            // not a real room
          }
          break;
        },
        _ => panic!(),
      }
    }
  }

  println!("{}", sectorid);
}
