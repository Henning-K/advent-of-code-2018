extern crate failure;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

use failure::Error;

struct Timer<'a> {
    desc: &'a str,
}

impl<'a> Timer<'a> {
    fn create(desc: &'a str) -> Self {
        Timer { desc }
    }

    fn run<T: std::fmt::Debug, F: (Fn() -> Result<T, Error>)>(&self, f: F) -> Result<(), Error> {
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

fn main() -> Result<(), Error> {
    println!("File I/O will be included in timers.");

    let timer_01_a = Timer::create("Timer 01 a");
    timer_01_a.run(task_01_a)?;

    let timer_01_b = Timer::create("Timer 01 b");
    timer_01_b.run(task_01_b)?;

    let timer_02_a = Timer::create("Timer 02 a");
    timer_02_a.run(task_02_a)?;

    let timer_02_b = Timer::create("Timer 02 b");
    timer_02_b.run(task_02_b)?;

    Ok(())
}

fn task_01_a() -> Result<i64, Error> {
    let in_file = File::open("data/01_a.txt")?;
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

fn task_01_b() -> Result<i64, Error> {
    let in_file = File::open("data/01_b.txt")?;
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

fn task_02_a() -> Result<i64, Error> {
    let in_file = File::open("data/02_a.txt")?;
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
        if bts.values().any(|&val| val==2) {
            arr[0] += 1;
        }
        if bts.values().any(|&val| val==3) {
            arr[1] += 1;
        }
    }

    Ok(arr.iter().product())
}

fn task_02_b() -> Result<String, Error> {
    let in_file = File::open("data/02_b.txt")?;
    let buf_rdr = BufReader::new(in_file);

    let mut vec = buf_rdr.lines().map(Result::unwrap).collect::<Vec<String>>();
    vec.sort_unstable();

    for (i, val) in vec.iter().enumerate() {
        for j in i..vec.len() {
            if 1 == val.chars().zip(vec[j].chars()).filter(|(a,b)| a!=b).count() {
                return Ok(val.chars().zip(vec[j].chars()).filter(|(a,b)| a==b).map(|(a,_b)| a).collect::<String>());
            }
        }
    }

    Ok(String::new())
}
