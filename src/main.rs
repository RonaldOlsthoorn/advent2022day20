
use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque, HashMap}};

const BACKSPACE: char = 8u8 as char;

fn mix(encrypted_file: Vec<i32>) -> Vec<i32> {

    let mut working_file: VecDeque<(usize, i32)> = encrypted_file.iter().enumerate().map(|(i, v)| (i, *v)).collect();

    for (i, element) in encrypted_file.iter().enumerate() {

        let old_index = working_file.iter().position(|v| v == &(i, *element)).unwrap();

        if element == &0 {
            continue;
        } else  {
            let new_index = (old_index as i32 + element - 1).rem_euclid(encrypted_file.len() as i32 - 1) + 1;
            working_file.remove(old_index);
            working_file.insert(new_index as usize, (i, *element));
        }
    }

    return working_file.iter().map(|(_, v)| *v).collect();
}


fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let encrypted_file : Vec<i32> = reader.lines().map(|l| l.unwrap().parse::<i32>().unwrap()).collect();

    let mut m = HashMap::new();

    for v in encrypted_file.iter() {

      if let Some(fr) = m.get(v) {
        println!("increase {}", v);
        m.insert(v, fr + 1);
      } else {
        m.insert(v, 1);
      }
    }

    println!("doubles? {}", m.iter().any(|(k, v)| v > &1));

    let mixed_file = mix(encrypted_file);

    let zero_index = mixed_file.iter().position(|v| v == &0).unwrap();
    let first_shift = mixed_file[(zero_index + 1000) % mixed_file.len()];
    let second_shift = mixed_file[(zero_index + 2000) % mixed_file.len()];
    let third_shift = mixed_file[(zero_index + 3000) % mixed_file.len()];
    
    println!("final result {}",  first_shift + second_shift + third_shift);
}