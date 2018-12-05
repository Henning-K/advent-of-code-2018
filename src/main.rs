extern crate failure;
#[macro_use]
extern crate nom;

use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time::Instant;

use failure::Error;

type Result<T> = std::result::Result<T, Error>;

mod task_01;
use task_01::*;
mod task_02;
use task_02::*;
mod task_03;
use task_03::*;
mod task_05;
use task_05::*;

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

macro_rules! timer_create_run {
    ($timer:ident, $task:ident) => {
        let $timer = Timer::create(stringify!($timer));
        $timer.run($task)?;
    };
}

fn main() -> Result<()> {
    println!("Disk I/O will be included in timers.");

    timer_create_run!(timer_01_a, task_01_a);
    timer_create_run!(timer_01_b, task_01_b);
    timer_create_run!(timer_02_a, task_02_a);
    timer_create_run!(timer_02_b, task_02_b);
    timer_create_run!(timer_03_a, task_03_a);
    
    timer_create_run!(timer_05_a, task_05_a);
    timer_create_run!(timer_05_b, task_05_b);
    
    Ok(())
}
