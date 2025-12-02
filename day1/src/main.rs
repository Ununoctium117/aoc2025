use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::bail;

enum Instruction {
    Left(i32),
    Right(i32),
}
impl Instruction {
    fn to_delta(&self) -> i32 {
        match self {
            Instruction::Left(x) => -x,
            Instruction::Right(x) => *x,
        }
    }
}
impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(remainder) = s.strip_prefix('L') {
            Ok(Instruction::Left(remainder.parse()?))
        } else if let Some(remainder) = s.strip_prefix('R') {
            Ok(Instruction::Right(remainder.parse()?))
        } else {
            bail!("unknown instruction prefix")
        }
    }
}

#[derive(Debug)]
struct SpinResult {
    new_pos: u32,
    num_zeros: u32,
}
fn spin(cur_pos: u32, mut delta: i32) -> SpinResult {
    let mut result = SpinResult {
        new_pos: cur_pos,
        num_zeros: 0,
    };
    while delta > 100 {
        result.num_zeros += 1;
        delta -= 100;
    }
    while delta < -100 {
        result.num_zeros += 1;
        delta += 100;
    }

    if delta > 0 {
        result.new_pos = result.new_pos.strict_add_signed(delta);

        while result.new_pos >= 100 {
            result.new_pos -= 100;
            result.num_zeros += 1;
        }
    } else {
        while (result.new_pos as i32) < (-delta) {
            result.new_pos += 100;
            result.num_zeros += 1;
        }

        result.new_pos = result.new_pos.strict_add_signed(delta);
    }

    result
}

fn main() -> anyhow::Result<()> {
    let instructions = BufReader::new(File::open("input")?)
        .lines()
        .map(|line| line?.parse())
        .collect::<Result<Vec<Instruction>, _>>()?;

    let mut position: u32 = 50;
    let mut count = 0;

    for instruction in instructions {
        let SpinResult { new_pos, num_zeros } = spin(position, instruction.to_delta());
        position = new_pos;
        count += num_zeros;
    }

    println!("Solution 1: {count}");

    Ok(())
}
