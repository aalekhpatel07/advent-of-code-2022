use std::ops::{Rem, Div};

fn main() {
    let inputs = include_str!("input.txt");
    println!("Part 1: {}", solve_part1(inputs));
    println!("Part 2: (should read PZBGZEJB)\n{}", solve_part2(inputs));
}


#[derive(Debug, Clone)]
pub enum Instruction {
    Add(isize),
    Noop
}


impl TryFrom<&str> for Instruction {
    type Error = std::io::Error;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }
        if s.starts_with("addx") {
            return Ok(Instruction::Add(s.split(" ").last().unwrap().parse::<isize>().unwrap()))
        }
        Err(Self::Error::new(std::io::ErrorKind::InvalidData, "Couldn't parse instruction".to_owned()))
    }
}

pub fn get_instructions(inputs: &str) -> Vec<Instruction> {
    inputs
    .lines()
    .take_while(|&line| !line.is_empty())
    .map(|line| {
        Instruction::try_from(line).unwrap()
    })
    .collect()
}

pub fn update_part1_state(
    indices_to_sum: &[isize],
    cycles: isize,
    current_register_value: isize
) -> isize {
    match indices_to_sum.contains(&cycles) {
        true => cycles * current_register_value,
        false => 0
    }
}

pub fn solve_part1(inputs: &str) -> isize {

    let instructions = get_instructions(inputs);

    let mut cycles: isize = 0;
    let mut register_value: isize = 1;

    let mut result: isize = 0;
    let indices_to_sum: [isize; 6] = [20, 60, 100, 140, 180, 220];

    for instruction in instructions.into_iter() {
        match instruction {
            Instruction::Noop => {
                cycles += 1;
                result += update_part1_state(&indices_to_sum, cycles, register_value);
            },
            Instruction::Add(step) => {
                cycles += 1;
                result += update_part1_state(&indices_to_sum, cycles, register_value);

                cycles += 1;
                result += update_part1_state(&indices_to_sum, cycles, register_value);

                register_value += step;
            }
        }
    }

    result
}


pub fn update_part2_state(
    sprite_visible: &mut[[bool; 40]; 6],
    register_value: isize,
    cycles: isize
) {
    // % 40 so that we draw on all the rows of the CRT monitor.
    if [(register_value - 1) % 40, register_value % 40, (register_value + 1) % 40].contains(&(cycles % 40)) {
        let row = cycles.div(40);
        let col = cycles.rem(40);
        sprite_visible[row as usize][col as usize] = true;
    }
}

pub fn solve_part2(inputs: &str) -> String {

    let instructions: Vec<Instruction> = inputs
    .lines()
    .take_while(|&line| !line.is_empty())
    .map(|line| {
        Instruction::try_from(line).unwrap()
    })
    .collect();

    let mut cycles: isize = 0;
    let mut register_value: isize = 1;

    let mut sprite_visible: [[bool; 40]; 6] = [[false; 40]; 6];
    // sprite_visible[0][0] = true;

    for instruction in instructions.into_iter() {
        match instruction {
            Instruction::Noop => {
                update_part2_state(&mut sprite_visible, register_value, cycles);
                cycles += 1;
            },
            Instruction::Add(step) => {
                update_part2_state(&mut sprite_visible, register_value, cycles);
                cycles += 1;

                update_part2_state(&mut sprite_visible, register_value, cycles);
                cycles += 1;

                register_value += step;
            }
        }
    }
    
    sprite_visible
    .iter()
    .map(|row| {
        row
        .iter()
        .map(|&entry| if entry { "#" } else { "." })
        .collect::<String>()
    })
    .collect::<Vec<String>>()
    .join("\n")

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_big() {
        let inputs = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

        assert_eq!(13140, solve_part1(inputs));
        assert_eq!(&solve_part2(inputs), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }

}