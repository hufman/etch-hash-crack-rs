extern crate etch_hash;
use std::str;
//use std::cell::Cell;

// [.a-z]
const CHARS: &[u8] = &[46, 97, 98, 99, 100, 101, 102, 103,
                      104, 105, 106, 107, 108, 109,
                      110, 111, 112, 113, 114, 115,
                      116, 117, 118, 119, 120, 121, 122];
const MAX_DEPTH: usize = 4;

fn crack_step<'a>(start: &mut Vec<u8>, start_hash: u32, target_hash: u32, remaining_levels: u8) -> Option<Vec<u8>> {
    //println!("Trying {}", str::from_utf8(start.as_slice()).unwrap());
    if remaining_levels == 0 { return None; }
    // check for chars at the current level
    for next in CHARS.chunks(1) {
      let next_hash = etch_hash::hash_more(start_hash, next);
      if next_hash == target_hash {
          start.push(next[0]);
          return Some(start.clone());
        }
    }
    // next level
    for next in CHARS.chunks(1) {
      let next_hash = etch_hash::hash_more(start_hash, next);
      start.push(next[0]);
      let result = crack_step(start, next_hash, target_hash, remaining_levels-1);
      if result != None {
          return result;
      }
      start.pop();
    }
    None
}

fn crack<'a>(target_hash: u32) -> Option<Vec<u8>> {
    let mut start = Vec::with_capacity(MAX_DEPTH+1);
    let start_hash = etch_hash::hash(start.as_slice());
    return crack_step(&mut start, start_hash, target_hash, MAX_DEPTH as u8);
}

fn main() {
    let found = crack(1511848646);   // should match "ab"
    match found {
      Some(target) => println!("Found! {}", str::from_utf8(target.as_slice()).unwrap()),
      None => println!("Not found"),
    }
}
