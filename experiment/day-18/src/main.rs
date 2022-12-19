use std::collections::{HashMap, HashSet};


const INPUT: &str = include_str!("input.txt");


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cube((isize, isize, isize));


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back
}



impl Cube {
    pub fn shares_side(&self, other: &Cube) -> Option<Side> {
        let (x1, y1, z1) = self.0;
        let (x2, y2, z2) = other.0;

        if y1 == y2 && z1 == z2 {
            if x1 == x2 - 1 {
                return Some(Side::Right);
            } else if x1 == x2 + 1 {
                return Some(Side::Left);
            }
        } else if x1 == x2 && z1 == z2 {
            if y1 == y2 - 1 {
                return Some(Side::Bottom);
            } else if y1 == y2 + 1 {
                return Some(Side::Top);
            }
        } else if x1 == x2 && y1 == y2 {
            if z1 == z2 - 1 {
                return Some(Side::Back);
            } else if z1 == z2 + 1 {
                return Some(Side::Front);
            }
        }
        None
    }

    pub fn neighbors(&self) -> Vec<Cube> {
        let mut neighbors = Vec::new();
        let (x, y, z) = self.0;
        neighbors.push(Cube((x + 1, y, z)));
        neighbors.push(Cube((x - 1, y, z)));
        neighbors.push(Cube((x, y + 1, z)));
        neighbors.push(Cube((x, y - 1, z)));
        neighbors.push(Cube((x, y, z + 1)));
        neighbors.push(Cube((x, y, z - 1)));
        neighbors
    }
}

pub fn solve_part1(cubes: &[Cube]) -> usize {
    let mut hset = HashSet::new();
    
    for (i, cube1) in cubes.iter().enumerate() {
        for (j, cube2) in cubes.iter().enumerate() {
            if i == j {
                continue;
            }
            if let Some(face) = cube1.shares_side(cube2) {
                hset.insert((*cube1, face));
            }
        }
    }
    cubes.len() * 6 - hset.len()
}

// Will have to accomodate for interior cubes
// that share sides. And get total unique sides.
pub fn solve_part2(cubes: &[Cube]) -> usize {

    let mut interior_cubes = HashSet::new();

    for cube in cubes.iter() {
        if cube.neighbors().iter().all(|n| cubes.contains(n)) {
            interior_cubes.insert(*cube);
        }
    }

    let mut non_interior_cubes = HashSet::new();

    for cube in cubes.iter() {
        if !interior_cubes.contains(cube) {
            non_interior_cubes.insert(*cube);
        }
    }

    let mut interior_cubes = interior_cubes.into_iter().collect::<Vec<_>>();
    interior_cubes.sort();


    let mut hset = HashSet::new();
    
    for &cube1 in cubes.iter() {
        for &cube2 in cubes.iter() {
            if cube1 == cube2 {
                continue;
            }
            if let Some(face) = cube1.shares_side(&cube2) {
                hset.insert((cube1, face));
            }
        }
    }


    let seen_sides = cubes.len() * 6 - hset.len();

    seen_sides
    // seen_sides - interior_cubes.len() * 6
}

pub fn parse_cubes(s: &str) -> Vec<Cube> {
    let mut cubes = Vec::new();
    s
    .lines()
    .for_each(|line| {
        let cube_str = line.split(",").collect::<Vec<_>>();
        let x = cube_str[0].parse::<isize>().unwrap();
        let y = cube_str[1].parse::<isize>().unwrap();
        let z = cube_str[2].parse::<isize>().unwrap();
        cubes.push(Cube((x, y, z)));
    });

    cubes
}


fn main() {
    let s = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
    let cubes = parse_cubes(INPUT);
    // println!("{}", solve_part1(&cubes));
    println!("{}", solve_part2(&cubes));
    // println!("Hello, world!");
}
