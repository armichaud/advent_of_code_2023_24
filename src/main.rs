use core::time;
use std::{fs::File, io::{BufReader, BufRead}};

use nalgebra::Vector2;

#[derive(Debug)]
struct Hailstone {
    px: usize,
    py: usize,
    #[allow(dead_code)]
    pz: usize,
    vx: isize,
    vy: isize,
    #[allow(dead_code)]
    vz: isize,
}

impl Hailstone {
    fn collides(&self, other: &Hailstone, lower: f64, upper: f64) -> bool {
        let p1 = Vector2::new(self.px as f64, self.py as f64);
        let p2 = Vector2::new(other.px as f64, other.py as f64);
        let v1 = Vector2::new(self.vx as f64, self.vy as f64);
        let v2 = Vector2::new(other.vx as f64, other.vy as f64);
        // println!("p1: {}, p2: {}, v1: {}, v2: {}", p1, p2, v1, v2);
        let dp = p2 - p1;
        let dv = v1 - v2;
        // println!("dp: {}, dv: {}", dp, dv);
        if dv == Vector2::zeros() {
            return false;
        }
        let time_at_intersection = dp.component_div(&dv);
        let tx = time_at_intersection.x;
        let ty = time_at_intersection.y;
        // println!("T: {}", time_at_intersection);
        if tx < 0.0 || ty < 0.0 {
            return false;
        }
        let p1_at_intersection = self.get_position(tx);
        // let p2_at_intersection = other.get_position(ty);
        // println!("p1: {}, p2: {}", p1_at_intersection, p2_at_intersection);
        let within_bound = p1_at_intersection.x >= lower && p1_at_intersection.x <= upper && p1_at_intersection.y >= lower && p1_at_intersection.y <= upper;
        within_bound
    }

    fn get_position(&self, time: f64) -> Vector2<f64> {
        let x = self.px as f64 + (self.vx as f64 * time);
        let y = self.py as f64 + (self.vy as f64 * time);
        Vector2::new(x, y)
    }
}

fn get_hailstones(file: &str) -> Vec<Hailstone> {
    let file = File::open(file).unwrap();
    let lines = BufReader::new(file).lines();
    let mut hailstones = Vec::<Hailstone>::new();
    for line in lines {
        let line = line.unwrap();
        let mut split = line.split("@");
        let position = split.next().unwrap().split(", ").map(|x| x.trim().parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let velocity = split.next().unwrap().split(", ").map(|x| x.trim().parse::<isize>().unwrap()).collect::<Vec<isize>>();
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
    let nhailstones = hailstones.len();
    let mut collisions = 0;
    for i in 0..nhailstones {
        for j in i+1..nhailstones {
            if hailstones[i].collides(&hailstones[j], lower, upper) {
                collisions += 1;
            }
        }
    }
    collisions
}

fn main() {
    assert_eq!(solution("example.txt", 7.0, 27.0), 2);
    // assert_eq!(solution("input.txt", 200000000000000.0, 400000000000000.0), 12045);
}
