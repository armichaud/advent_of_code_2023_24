use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;
use nalgebra::{Vector2, Matrix2};

#[derive(Debug)]
struct Hailstone {
    px: f64,
    py: f64,
    #[allow(dead_code)]
    pz: f64,
    vx: f64,
    vy: f64,
    #[allow(dead_code)]
    vz: f64,
}

impl Hailstone {
    fn collides(&self, other: &Hailstone, lower: f64, upper: f64) -> bool {
        let velocities = Matrix2::from_row_slice(&[self.vx, -other.vx, self.vy, -other.vy]);
        let positions = Vector2::new(other.px - self.px, other.py - self.py);
        if let Some(inverse) = velocities.try_inverse() {
            let intersection_times = inverse * positions;
            if intersection_times.x < 0.0 || intersection_times.y < 0.0 {
                return false;
            }
            let p = self.get_position(intersection_times.x);
            p.x >= lower && p.x <= upper && p.y >= lower && p.y <= upper
        } else {
            false
        }
    }
    

    fn get_position(&self, time: f64) -> Vector2<f64> {
        Vector2::new(self.px + (self.vx * time), self.py + (self.vy * time))
    }
}

fn get_hailstones(file: &str) -> Vec<Hailstone> {
    let file = File::open(file).unwrap();
    let lines = BufReader::new(file).lines();
    let mut hailstones = Vec::<Hailstone>::new();
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split("@");
        let position = split.next().unwrap().split(", ").map(|x| x.trim().parse::<f64>().unwrap()).collect::<Vec<f64>>();
        let velocity = split.next().unwrap().split(", ").map(|x| x.trim().parse::<f64>().unwrap()).collect::<Vec<f64>>();
        hailstones.push(Hailstone {
            px: position[0],
            py: position[1],
            pz: position[2],
            vx: velocity[0],
            vy: velocity[1],
            vz: velocity[2],
        });
    }
    hailstones
}

fn solution(file: &str, lower: f64, upper: f64) -> usize {
    let hailstones = get_hailstones(file);
    hailstones.iter().combinations(2).filter(|hailstones| {
        hailstones[0].collides(&hailstones[1], lower, upper) 
    }).count()
}

fn main() {
    assert_eq!(solution("example.txt", 7.0, 27.0), 2);
    assert_eq!(solution("input.txt", 200000000000000.0, 400000000000000.0), 11246);
}
