#![allow(non_snake_case)]
use super::*;

use ::nom::types::CompleteStr;
use std::cmp::max;

pub(crate) fn task_03_a() -> Result<usize> {
    let in_file = File::open("data/03.txt")?;
    let buf_rdr = BufReader::new(in_file);

    let claims: Vec<Claim> = buf_rdr
        .lines()
        .map(std::result::Result::unwrap)
        .map(|s| Claim::from_str(&s).expect("Claim::from_str failed."))
        .collect();

    let (width, height) = claims.iter().fold((0usize,0usize), |acc, c| {
        (max(acc.0, (c.x2+1) as usize), max(acc.1, (c.y2+1) as usize))
    });

    let mut board: Vec<u32> = (0..width*height).into_iter().map(|_| 0u32).collect();

    for claim in claims {
        for col in claim.x1..(claim.x2+1) {
            for row in claim.y1..(claim.y2+1) {
                let (col, row) = (col as usize, row as usize);
                board[width * row + col] += 1;
            }
        }
    }

    Ok(board.iter().filter(|&&x| x>=2).count())
}

pub(crate) fn task_03_b() -> Result<u32> {
    let in_file = File::open("data/03.txt")?;
    let buf_rdr = BufReader::new(in_file);

    let claims: Vec<Claim> = buf_rdr
        .lines()
        .map(std::result::Result::unwrap)
        .map(|s| Claim::from_str(&s).expect("Claim::from_str failed."))
        .collect();

    for (claim_a, i) in claims.iter().zip(0..) {
        if !claims[0..i].iter().any(|claim_b| claim_a.does_intersect(claim_b)) &&
            !claims[(i+1)..].iter().any(|claim_b| claim_a.does_intersect(claim_b)) {
                return Ok(claim_a.id);
        }
    }

    Ok(0) // 0 is not an ID in the input therefore it represents the error case here.
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Claim {
    fn new(id: u32, x1: u32, y1: u32, width: u32, height: u32) -> Self {
        Claim {
            id,
            x1,
            y1,
            x2: x1 + width-1,
            y2: y1 + height-1,
        }
    }

    fn from_str<'a>(s: &'a str) -> std::result::Result<Self, nom::Err<CompleteStr>> {
        claim_line(CompleteStr::from(s)).map(|(_rest, claim)| claim)
    }

    fn does_intersect(&self, other: &Self) -> bool {
        self.x1 <= (other.x2+1) && self.x2+1 >= (other.x1)
            && self.y1 <= (other.y2+1) && (self.y2+1) >= other.y1
    }
}

fn to_u32(input: CompleteStr) -> std::result::Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(&input, 10)
}

named!(digit_muncher<CompleteStr, u32>,
    map_res!(digit, to_u32)
);

named!(claim_line<CompleteStr, Claim>,
    dbg_dmp!(
        do_parse!(
        tag!("#")
        >>
        id: digit_muncher
        >>
        tag!(" @ ")
        >>
        x1: digit_muncher
        >>
        tag!(",")
        >>
        y1: digit_muncher
        >>
        tag!(": ")
        >>
        width: digit_muncher
        >>
        tag!("x")
        >> 
        height: digit_muncher
        >>
        (Claim::new(id, x1, y1, width, height)))
    )
);
