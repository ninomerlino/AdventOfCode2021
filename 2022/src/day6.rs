use std::{
    fs::File,
    io::{BufReader, Read, Seek},
};

use identity_hash::BuildIdentityHasher;

use crate::common::{FastHashSet, Result};

#[inline]
fn is_start_of_packet(buf: &[u8; 4]) -> bool {
    (buf[0] != buf[1])
        && (buf[0] != buf[2])
        && (buf[0] != buf[3])
        && (buf[1] != buf[2])
        && (buf[1] != buf[3])
        && (buf[2] != buf[3])
}

fn find_start_of_packet(file: &File) -> Result<u64> {
    let mut stream = BufReader::new(file);
    let mut buffer = [0_u8; 4];
    stream.read_exact(&mut buffer)?;

    while !is_start_of_packet(&buffer) {
        buffer.rotate_left(1);
        stream.read_exact(&mut buffer[3..4])?;
    }
    Ok(stream.stream_position()?)
}

fn is_start_of_message(buf: &[u8; 14]) -> bool {
    let mut bytes = FastHashSet::with_hasher(BuildIdentityHasher::default());
    for elm in buf {
        if bytes.contains(elm) {
            return false;
        }
        bytes.insert(*elm);
    }
    true
}

fn find_start_of_message(file: &File) -> Result<u64> {
    let mut stream = BufReader::new(file);
    let mut buffer = [0_u8; 14];
    stream.read_exact(&mut buffer)?;

    while !is_start_of_message(&buffer) {
        for i in 0..13 {
            buffer[i] = buffer[i + 1];
        }
        stream.read_exact(&mut buffer[13..14])?;
    }
    Ok(stream.stream_position()?)
}

pub fn challenge() {
    let input_file = File::open("inputs/day6.txt").unwrap();
    let first_start_of_packet = find_start_of_packet(&input_file).unwrap();
    println!("Start of packet at {}", first_start_of_packet);
}

pub fn challenge_2() {
    let input_file = File::open("inputs/day6.txt").unwrap();
    let first_start_of_message = find_start_of_message(&input_file).unwrap();
    println!("Start of packet at {}", first_start_of_message);
}
