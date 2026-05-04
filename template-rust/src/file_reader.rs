//! For file reading.
//!
//! Author: Anatoly Weinstein
//! Created: April 2026

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use crate::Particle;

pub struct FileReader;

impl FileReader {
    pub fn read_file(particles: &mut Vec<Particle>, filename: &str) -> Result<()> {
        let mut x: [f64; 3];
        let mut v: [f64; 3];
        let mut m: f64;

        let f = File::open(filename)?;
        let mut reader = BufReader::new(f);
        let mut tmp_string = String::new();

        reader.read_line(&mut tmp_string)?;
        print!("Read line: {}", tmp_string);

        while tmp_string.is_empty() || tmp_string.starts_with("#") {
            tmp_string.clear();
            reader.read_line(&mut tmp_string)?;
            print!("Read line: {}", tmp_string);
        }

        let num_particles = tmp_string.trim().parse::<usize>().unwrap();
        println!("Reading {num_particles}.");

        tmp_string.clear();
        reader.read_line(&mut tmp_string)?;
        print!("Read line: {}", tmp_string);

        for i in 0..num_particles {
            let numbers = tmp_string.split_whitespace().collect::<Vec<_>>();
            x = [
                numbers[0].parse::<f64>().unwrap(),
                numbers[1].parse::<f64>().unwrap(),
                numbers[2].parse::<f64>().unwrap(),
            ];
            v = [
                numbers[3].parse::<f64>().unwrap(),
                numbers[4].parse::<f64>().unwrap(),
                numbers[5].parse::<f64>().unwrap(),
            ];
            m = numbers[6].parse::<f64>().unwrap();

            particles.push(Particle::new(x, v, m, 0));

            tmp_string.clear();
            reader.read_line(&mut tmp_string)?;

            if i < num_particles - 1 {
                print!("Read line: {}", tmp_string);
            }
        }

        Ok(())
    }
}
