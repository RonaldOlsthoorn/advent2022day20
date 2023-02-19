
use std::{io::{BufReader, BufRead}, fs::File, collections::{VecDeque, HashMap, HashSet}};
use ndarray::{Array2};

const BACKSPACE: char = 8u8 as char;

struct BoulderField {
    field: Array2<u32>,
    depth: usize
}

impl BoulderField {

    fn get(&self, x: usize, y: usize, z: usize) -> bool {
        self.field[[x, y]] & (1 << z) != 0
    }

    fn set_occupied(&mut self, x: usize, y: usize, z: usize) {
        self.field[[x, y]] |= 1 << z;
    }

    fn set_unoccupied(&mut self, x: usize, y: usize, z: usize) {
        self.field[[x, y]] &= !(1 << z);
    }

    fn get_number_of_neighbours(&self, x: usize, y: usize, z: usize) -> usize {

        let mut res = 0;

        if x + 1 < self.field.dim().0 && self.get(x + 1, y, z) {
            res += 1;
        }
        if x > 0 && self.get(x - 1, y, z) {
            res += 1;
        }

        if y + 1 < self.field.dim().1 && self.get(x, y + 1, z) {
            res += 1;
        }
        if y > 0 && self.get(x, y - 1, z) {
            res += 1;
        }

        if z + 1 < self.depth && self.get(x, y, z + 1) {
            res += 1;
        }
        if z > 0 && self.get(x, y, z - 1) {
            res += 1;
        }

        return res;

    }

    fn filter_cavities(&mut self, start: (usize, usize, usize)) {

        let mut Q = HashSet::new();
        Q.insert(start);
        let mut empty_spaces: HashSet<(usize, usize, usize)> = HashSet::new();
        let mut filled_spaces: HashSet<(usize, usize, usize)> = HashSet::new();
        

        while !Q.is_empty() {

            let (x, y, z) = &Q.iter().next().unwrap().clone();
            Q.remove(&(*x, *y, *z));

            if !self.get(*x, *y, *z) {
                
                if x + 1 < self.field.dim().0 && !empty_spaces.contains(&(x + 1, *y, *z)) && !filled_spaces.contains(&(x + 1, *y, *z)) {
                    Q.insert((x + 1, *y, *z));
                }
                if *x > 0 && !empty_spaces.contains(&(x - 1, *y, *z)) && !filled_spaces.contains(&(x - 1, *y, *z)) {
                    Q.insert((x - 1, *y, *z));
                }
        
                if y + 1 < self.field.dim().1 && !empty_spaces.contains(&(*x, y + 1, *z)) && !filled_spaces.contains(&(*x, y + 1, *z)) {
                    Q.insert((*x, y + 1, *z));
                }
                if *y > 0 && !empty_spaces.contains(&(*x, y - 1, *z)) && !filled_spaces.contains(&(*x, y - 1, *z)) {
                    Q.insert((*x, y - 1, *z));
                }
        
                if z + 1 < self.depth && !empty_spaces.contains(&(*x, *y, z + 1)) && !filled_spaces.contains(&(*x, *y, z + 1)) {
                    Q.insert((*x, *y, z + 1));
                }
                if *z > 0 && !empty_spaces.contains(&(*x, *y, z - 1)) && !filled_spaces.contains(&(*x, *y, z - 1)) {
                    Q.insert((*x, *y, z - 1));
                }
                
                empty_spaces.insert((*x, *y, *z));
            } else{
                filled_spaces.insert((*x, *y, *z));
            }
        }

        for row in 0..self.field.dim().0 {
            for column in 0..self.field.dim().1 {

                let mut strip = self.field[[row, column]];

                for z in 0..self.depth {
                    if !self.get(row, column, z) && !empty_spaces.contains(&(row, column, z)) {
                        strip |= 1 << z;
                    }
                }

                self.field[[row, column]] = strip;
            }
        }
    }

    fn calculate_boundary_area(&self) -> usize{

        let mut res = 0;

        for row in 0..self.field.dim().0 {
            for column in 0..self.field.dim().1 {
                for z in 0..self.depth {
                    if self.get(row, column, z) {
                        res += 6;
                        res -= self.get_number_of_neighbours(row, column, z);
                    }
                }
            }
        }

        res
    }
}

fn main() {

    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;
    
    for line in reader.lines().map(|l| l.unwrap()) {

        for (i, coord) in line.split(',').enumerate() {
            if i == 0 {
                x_max = std::cmp::max(x_max, coord.parse::<usize>().unwrap());
            } else if i == 1 {
                y_max = std::cmp::max(y_max, coord.parse::<usize>().unwrap());
            } else {
                z_max = std::cmp::max(z_max, coord.parse::<usize>().unwrap());
            }
        }
    }

    let reader = BufReader::new(File::open("input.txt").unwrap());

    println!("x_max {} y_max {} z_max {}", x_max, y_max, z_max);

    let mut field = BoulderField{field: Array2::zeros((x_max + 3, y_max + 3)), depth: z_max + 3};

    for line in reader.lines().map(|l| l.unwrap()) {

        let x = line.split(',').nth(0).unwrap().parse::<usize>().unwrap();
        let y = line.split(',').nth(1).unwrap().parse::<usize>().unwrap();
        let z = line.split(',').nth(2).unwrap().parse::<usize>().unwrap();

        field.set_occupied(x + 1, y + 1, z + 1);
    }

    field.filter_cavities((0, 0, 0));

    println!("number_of_exposed_squares {}", field.calculate_boundary_area());
   
}