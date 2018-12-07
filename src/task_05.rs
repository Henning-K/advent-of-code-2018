use super::*;

pub(crate) fn task_05_a() -> Result<usize> {
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

pub(crate) fn task_05_b() -> Result<usize> {
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
    let mut bs = bs
        .replace(c, "")
        .replace(c.to_ascii_uppercase(), "")
        .into_bytes();
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
