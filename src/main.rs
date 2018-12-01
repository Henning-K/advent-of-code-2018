extern crate failure;

use std::collections::BTreeSet;
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
