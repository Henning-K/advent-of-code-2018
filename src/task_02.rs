use super::*;

pub(crate) fn task_02_a() -> Result<i64> {
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

pub(crate) fn task_02_b() -> Result<String> {
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
