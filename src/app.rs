use rand::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use l003::Engine;

pub fn main() {
    
    let f = File::open("data/dtx64s001.txt").unwrap();

    let reader = BufReader::new(f);

    let lines = reader.lines();

    let mut eng = Engine::new();

    for line in lines {
        if let Ok(lx) = line { eng.ack(&lx); }
    }
}