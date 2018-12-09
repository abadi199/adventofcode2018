use nom::*;
use std::{
    fmt,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let claims: Vec<Claim> = read_input()
        .iter()
        .filter_map(|input| Claim::from_string(input))
        .collect();

    let mut fabric = Fabric::empty();
    for claim in claims.iter() {
        fabric.add(&claim);
    }

    match fabric.find_no_overlap(&claims) {
        None => println!("Not found"),
        Some(claim_id) => println!("Solution: {}", claim_id),
    }
}

fn read_input() -> Vec<String> {
    BufReader::new(File::open("input.txt").expect("Unable to open input.txt"))
        .lines()
        .filter_map(|result| result.ok())
        .collect()
}

struct Fabric {
    data: Vec<Vec<u32>>,
}

impl Fabric {
    fn empty() -> Self {
        Fabric { data: Vec::new() }
    }

    fn find_no_overlap(&self, claims: &[Claim]) -> Option<String> {
        for claim in claims.iter() {
            if self.no_overlap(claim) {
                return Some(claim.id.clone());
            }
        }

        None
    }

    fn no_overlap(&self, claim: &Claim) -> bool {
        for y in claim.top..(claim.top + claim.height) {
            for x in claim.left..(claim.left + claim.width) {
                let value = self.get(x, y);
                if value > 1 {
                    return false;
                }
            }
        }

        true
    }

    fn print(&mut self) -> &mut Self {
        println!("{:?}", self);
        self
    }

    fn get(&self, x: usize, y: usize) -> u32 {
        self.data
            .get(y)
            .and_then(|inside| inside.get(x))
            .unwrap_or(&0)
            .clone()
    }

    fn area(&self) -> u32 {
        self.data.iter().fold(0, |count, inside| {
            count
                + inside.iter().fold(0, |inside_count, value| {
                    if value > &1 {
                        return inside_count + 1;
                    } else {
                        return inside_count;
                    }
                })
        })
    }

    fn add(&mut self, claim: &Claim) -> &mut Self {
        let diff = ((claim.top + claim.height) as isize) - (self.data.len() as isize);
        self.grow_y(diff);

        for y in claim.top..(claim.top as usize + claim.height as usize) {
            let inside: &mut Vec<u32> = self.data.get_mut(y).unwrap();
            let len = std::cmp::max(inside.len(), claim.left + claim.width);
            for x in 0..len {
                let mut value = inside.get(x).unwrap_or(&0).clone();
                if x >= claim.left && x < claim.left + claim.width {
                    value = value + 1;
                }

                if x < inside.len() {
                    inside[x] = value;
                } else {
                    inside.push(value);
                }
            }
        }

        self
    }

    fn grow_y(&mut self, diff: isize) {
        if diff <= 0 {
            return;
        }

        for _ in 0..diff {
            self.data.push(Vec::new());
        }
    }
}

impl Debug for Fabric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for y in 0..self.data.len() {
            for x in 0..self.data[y].len() {
                if let Some(count) = self.data.get(y).and_then(|inside| inside.get(x)) {
                    write!(f, "{}", count)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

const FABRIC_WIDTH: usize = 1000;
const FABRIC_HEIGHT: usize = 1000;

struct Claim {
    id: String,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Claim {
    pub fn from_string(input: &str) -> Option<Claim> {
        match claim(input).map(|(_, c)| c) {
            Ok(claim) => Some(claim),
            Err(err) => {
                println!("{}", err);
                None
            }
        }
    }
}

impl Debug for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        for y in 0..FABRIC_HEIGHT {
            for x in 0..FABRIC_WIDTH {
                if y >= self.top
                    && y < self.top + self.height
                    && x >= self.left
                    && x < self.left + self.width
                {
                    write!(f, "{}", self.id)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn end(input: &str) -> IResult<&str, &str> {
    Ok((input, input))
}

fn to_usize(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.trim().parse::<usize>()
}

named!(claim<&str, Claim>,
       do_parse!(tag!("#") >>
                 id: take_until_and_consume!("@") >>
                 left: map_res!(take_until_and_consume!(","), to_usize) >>
                 top: map_res!(take_until_and_consume!(":"), to_usize) >>
                 width: map_res!(take_until_and_consume!("x"), to_usize) >>
                 height: map_res!(end, to_usize) >>
                 (Claim {
                     id: id.trim().to_string(),
                     left: left,
                     top: top,
                     width: width,
                     height: height,
                 })
       )
);
