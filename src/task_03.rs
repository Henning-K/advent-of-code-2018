#![allow(non_snake_case)]
use super::*;

use nom::{digit, types::CompleteStr};

pub(crate) fn task_03_a() -> Result<u64> {
    let in_file = File::open("data/03.txt")?;
    let buf_rdr = BufReader::new(in_file);

    let claims: Vec<Claim> = buf_rdr
        .lines()
        .map(std::result::Result::unwrap)
        .map(|s| Claim::from_str(&s).expect("Claim::from_str failed."))
        .collect();

    // for c in claims {
    //     println!("{:?}", &c);
    // }

    Ok(0)
}

#[derive(Debug, PartialEq)]
struct Claim {
    id: u32,
    LU_x: u32,
    LU_y: u32,
    RL_x: u32,
    RL_y: u32,
}

impl Claim {
    fn new(id: u32, LU_x: u32, LU_y: u32, width: u32, height: u32) -> Self {
        Claim {
            id,
            LU_x,
            LU_y,
            RL_x: LU_x + width - 1,
            RL_y: LU_y + height - 1,
        }
    }

    fn from_str<'a>(s: &'a str) -> std::result::Result<Self, nom::Err<CompleteStr>> {
        claim_line(CompleteStr::from(s)).map(|(_rest, claim)| claim)
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
        LU_x: digit_muncher
        >>
        tag!(",")
        >>
        LU_y: digit_muncher
        >>
        tag!(": ")
        >>
        width: digit_muncher
        >>
        tag!("x")
        >> 
        height: digit_muncher
        >>
        (Claim::new(id, LU_x, LU_y, width, height)))
    )
);
