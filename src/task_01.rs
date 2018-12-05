use super::*;

pub(crate) fn task_01_a() -> Result<i64> {
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

pub(crate) fn task_01_b() -> Result<i64> {
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
