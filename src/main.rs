use std::{fs::File, io::{BufReader, BufRead}};

use nalgebra::Vector2;

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
        if (self.vx - other.vx) == 0.0 || (self.vy - other.vy) == 0.0 {
            return false;
        }
        let tx = (other.px - self.px) / (self.vx - other.vx);
        let ty = (other.py - self.py) / (self.vy - other.vy);
        let px = self.get_position(tx);
        let py = other.get_position(ty);
        let in_the_future = tx >= 0.0 && ty >= 0.0;
        let within_test_area = px.x >= lower && px.x <= upper && px.y >= lower && px.y <= upper && py.x >= lower && py.x <= upper && py.y >= lower && py.y <= upper;
        in_the_future && within_test_area
    }
    

    fn get_position(&self, time: f64) -> Vector2<f64> {
        let x = self.px + (self.vx * time);
        let y = self.py + (self.vy * time);
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
    assert_eq!(solution("input.txt", 200000000000000.0, 400000000000000.0), 9622);
}
