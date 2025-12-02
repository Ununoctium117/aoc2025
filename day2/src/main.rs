use std::{
    fs::File,
    io::{BufRead as _, BufReader},
};

fn is_valid_p1(id: u64) -> bool {
    let base10_str = id.to_string();
    let midpoint = if base10_str.len().is_multiple_of(2) {
        base10_str.len() / 2
    } else {
        return true;
    };

    // assumption: the string is all single-byte ASCII (which is true)
    let last_half = base10_str.as_bytes().split_off(midpoint..).unwrap();
    !base10_str.as_bytes().starts_with(last_half)
}

fn is_valid_p2(id: u64) -> bool {
    let base10_str = id.to_string();
    let bytes = base10_str.as_bytes();

    // extremely bad "find all divisors of X" but X is small so w/e
    for possible_chunk_size in 1..=bytes.len() / 2 {
        if bytes.len().is_multiple_of(possible_chunk_size) {
            let first_chunk = &bytes[..possible_chunk_size];
            let chunks = bytes.chunks_exact(possible_chunk_size);
            if chunks.skip(1).all(|x| x == first_chunk) {
                return false;
            }
        }
    }

    true
}

fn main() -> anyhow::Result<()> {
    let sum_p1: u64 = BufReader::new(File::open("input")?)
        .split(b',')
        .flat_map(|sequence| {
            let string = String::from_utf8(sequence.unwrap()).unwrap();
            let Some((lo, hi)) = string.split_once('-') else {
                panic!("not valid input");
            };

            let lo: u64 = lo.parse().unwrap();
            let hi: u64 = hi.parse().unwrap();

            lo..=hi
        })
        .filter(|x| !is_valid_p1(*x))
        // .inspect(|x| println!("invalid {x}"))
        .sum();

    println!("Part 1: {sum_p1}");

    let sum_p2: u64 = BufReader::new(File::open("input")?)
        .split(b',')
        .flat_map(|sequence| {
            let string = String::from_utf8(sequence.unwrap()).unwrap();
            let Some((lo, hi)) = string.split_once('-') else {
                panic!("not valid input");
            };

            let lo: u64 = lo.parse().unwrap();
            let hi: u64 = hi.parse().unwrap();

            lo..=hi
        })
        .filter(|x| !is_valid_p2(*x))
        // .inspect(|x| println!("invalid {x}"))
        .sum();

    println!("Part 2: {sum_p2}");

    Ok(())
}
