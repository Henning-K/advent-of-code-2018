use super::*;
use std::str::FromStr;

pub(crate) fn task_08_a() -> Result<usize> {
    let mut in_file = File::open("data/08.txt")?;
    let mut input = String::new();
    in_file.read_to_string(&mut input)?;

    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|n| u32::from_str(n).expect("Could not convert to u32 from &str"))
        .collect();

    Ok(Node::new(&numbers).sum_metadata())
}

pub(crate) fn task_08_b() -> Result<usize> {
    let mut in_file = File::open("data/08.txt")?;
    let mut input = String::new();
    in_file.read_to_string(&mut input)?;

    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|n| u32::from_str(n).expect("Could not convert to u32 from &str"))
        .collect();

    Ok(Node::new(&numbers).node_value())
}

#[derive(Debug, PartialEq)]
pub(crate) struct Node {
    children: Option<Vec<Node>>,
    metadata: Option<Vec<u32>>,
}

impl Node {
    pub(crate) fn new(vec: &[u32]) -> Self {
        let mut idx = 0;
        Self::parse(&vec, &mut idx)
    }

    fn parse(vec: &[u32], idx: &mut usize) -> Self {
        let (children_cnt, metadata_cnt) = (vec[*idx] as usize, vec[*idx + 1] as usize);
        *idx += 2;
        if children_cnt == 0 {
            let metadata = vec[*idx..(*idx + metadata_cnt)].to_vec();
            *idx += metadata_cnt;
            Node {
                children: None,
                metadata: if metadata.len() > 0 {
                    Some(metadata)
                } else {
                    None
                },
            }
        } else {
            let node = Node {
                children: Some(
                    (0..children_cnt)
                        .map(|_| Node::parse(vec, idx))
                        .collect::<Vec<Node>>()
                ),
                metadata: {
                let metadata = vec[*idx..(*idx + metadata_cnt)].to_vec();
                if metadata.len() > 0 {
                    Some(metadata)
                } else {
                    None
                }
                },
            };
            *idx += metadata_cnt;
            node
        }
    }

    pub(crate) fn sum_metadata(&self) -> usize {
        let meta_sum = match &self.metadata {
            None => 0usize,
            Some(v) => v.iter().map(|&i| i as usize).sum(),
        };
        meta_sum
            + match &self.children {
                None => 0usize,
                Some(v) => v.iter().map(|i| i.sum_metadata()).sum(),
            }
    }

    pub(crate) fn node_value(&self) -> usize {
        match &self.children {
            None => match &self.metadata {
                None => 0,
                Some(v) => v.iter().map(|&i| i as usize).sum(),
            },
            Some(ch) => match &self.metadata {
                None => 0,
                Some(v) => {
                    v.iter().map(|&i| if i == 0 {
                        0usize
                    } else {
                        ch.get(i as usize -1).map_or(0usize, |c| c.node_value())
                    }).sum()
                },
            },
        }
    }
}

#[test]
fn test_08_a() {
    let t = Node::new(&[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]);
    assert_eq!(t.sum_metadata(), 138);
}

#[test]
fn test_08_b() {
    let t = Node::new(&[2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2]);
    assert_eq!(t.node_value(), 66);
}
