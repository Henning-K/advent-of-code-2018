extern crate failure;
#[macro_use]
extern crate nom;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

mod task_03;
use task_03::*;

struct Timer<'a> {
    desc: &'a str,
}

impl<'a> Timer<'a> {
    fn create(desc: &'a str) -> Self {
        Timer { desc }
    }

    fn run<T: std::fmt::Debug, F: (Fn() -> Result<T>)>(&self, f: F) -> Result<()> {
        let start = Instant::now();
        let result = f()?;
        let dur = start.elapsed();

        println!(
            "{} = {:?} done in {}{}",
            self.desc,
            result,
            if dur.as_secs() > 0 {
                format!("{:03}s", dur.as_secs())
            } else {
                String::new()
            },
            if dur.subsec_millis() > 0 {
                format!("{:03}ms", dur.subsec_millis())
            } else if dur.subsec_micros() > 0 {
                format!("{:03}µs", dur.subsec_micros())
            } else if dur.subsec_nanos() > 0 {
                format!("{:03}ns", dur.subsec_nanos())
            } else {
                String::new()
            }
        );
        Ok(())
    }
}

fn main() -> Result<()> {
    println!("File I/O will be included in timers.");

    let timer_01_a = Timer::create("Timer 01 a");
    timer_01_a.run(task_01_a)?;

    let timer_01_b = Timer::create("Timer 01 b");
    timer_01_b.run(task_01_b)?;

    let timer_02_a = Timer::create("Timer 02 a");
    timer_02_a.run(task_02_a)?;

    let timer_02_b = Timer::create("Timer 02 b");
    timer_02_b.run(task_02_b)?;

    let timer_03_a = Timer::create("Timer 03 a");
    timer_03_a.run(task_03_a)?;

    let timer_05_a = Timer::create("Timer 05 a");
    timer_05_a.run(task_05_a)?;

    let timer_05_b = Timer::create("Timer 05 b");
    timer_05_b.run(task_05_b)?;
    
    Ok(())
}

fn task_01_a() -> Result<i64> {
    let in_file = File::open("data/01.txt")?;
    let buf_rdr = BufReader::new(in_file);
    let res = buf_rdr
        .lines()
        .map(|n| {
            n.expect("Getting line from file failed.")
                .parse::<i64>()
                .expect("Number parsing failed.")
        }).fold(0, |acc, x| acc + x);
    Ok(res)
}

fn task_01_b() -> Result<i64> {
    let in_file = File::open("data/01.txt")?;
    let buf_rdr = BufReader::new(in_file);
    let mut bts = BTreeSet::new();
    bts.insert(0i64);
    let numbers = buf_rdr
        .lines()
        .map(|n| {
            n.expect("Getting line from file failed.")
                .parse::<i64>()
                .expect("Number parsing failed.")
        }).collect::<Vec<i64>>();
    let mut acc = 0i64;
    for i in numbers.iter().cycle() {
        acc += i;
        if !bts.insert(acc) {
            return Ok(acc);
        }
    }
    unreachable!();
}

fn task_02_a() -> Result<i64> {
    let in_file = File::open("data/02.txt")?;
    let buf_rdr = BufReader::new(in_file);

    let mut arr = [0, 0];

    for line in buf_rdr.lines() {
        let mut bts = line
            .expect("")
            .chars()
            .fold(BTreeMap::<char, usize>::new(), |mut acc, c| {
                {
                    let entry = acc.entry(c).or_insert(0);
                    *entry += 1;
                }
                acc
            });
        if bts.values().any(|&val| val == 2) {
            arr[0] += 1;
        }
        if bts.values().any(|&val| val == 3) {
            arr[1] += 1;
        }
    }

    Ok(arr.iter().product())
}

fn task_02_b() -> Result<String> {
    let in_file = File::open("data/02.txt")?;
    let buf_rdr = BufReader::new(in_file);

    let mut vec = buf_rdr
        .lines()
        .map(std::result::Result::unwrap)
        .collect::<Vec<String>>();
    vec.sort_unstable();

    for (i, val) in vec.iter().enumerate() {
        for j in i..vec.len() {
            if 1 == val
                .chars()
                .zip(vec[j].chars())
                .filter(|(a, b)| a != b)
                .count()
            {
                return Ok(val
                    .chars()
                    .zip(vec[j].chars())
                    .filter(|(a, b)| a == b)
                    .map(|(a, _b)| a)
                    .collect::<String>());
            }
        }
    }

    Ok(String::new())
}

fn task_05_a() -> Result<usize> {
    let mut in_file = File::open("data/05.txt")?;
    let mut input = String::new();
    in_file.read_to_string(&mut input)?;

    let mut bs = String::from(input).into_bytes();
    let mut i = 1;
    let mut len = bs.len();
    let diff = b'a' - b'A';
    while i < len {
        if bs[i] - bs[i - 1] == diff || bs[i - 1] - bs[i] == diff {
            bs.remove(i);
            bs.remove(i - 1);
            i = 1;
            len = bs.len();
        } else {
            i += 1;
        }
    }
    Ok(bs.len())
}

fn task_05_b() -> Result<usize> {
    let mut in_file = File::open("data/05.txt")?;
    let mut input = String::new();
    in_file.read_to_string(&mut input)?;
    
    let min_n = (b'a'..b'z')
        .map(|c| test_for_letter(&input, c as char))
        .min()
        .unwrap();

    Ok(min_n)
}

fn test_for_letter(bs: &str, c: char) -> usize {
    let mut bs = bs.replace(c, "").replace(c.to_ascii_uppercase(), "").into_bytes();
    let mut i = 1;
    let mut len = bs.len();
    let diff = b'a' - b'A';
    while i < len {
        if bs[i] - bs[i - 1] == diff || bs[i - 1] - bs[i] == diff {
            bs.remove(i);
            bs.remove(i - 1);
            i = 1;
            len = bs.len();
        } else {
            i += 1;
        }
    }
    len
}
