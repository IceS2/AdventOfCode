use std::{fs, ops::Range};

fn main() {
    let hailstones: Vec<Hailstone> = fs::read_to_string("input.txt").unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line
            .split(" @ ")
            .map(|split|
                split.split(',')
                    .map(|c| c.trim().parse::<i64>().unwrap())
                    .collect::<Vec<i64>>()
            )
            .map(|split| (split[0], split[1], split[2]))
            .collect::<Vec<(i64, i64, i64)>>())
        .map(|line| ((line[0].0, line[1].0), (line[0].1, line[1].1), (line[0].2, line[1].2)).into())
        .collect();

    println!("{:?}", hailstones);

    let range: Range<i64> = 200000000000000..400000000000000;
    let mut sum: i32 = 0;

    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            // println!("[{:?}][{:?}]", i, j);
            if let Some((x, y)) = hailstones[i].intersects_at_xy(&hailstones[j]) {
                if range.start as f64 <= x && range.end as f64 > x && range.start as f64 <= y && range.end as f64 > y {
                    // println!("({:?}, {:?})", x, y);
                    sum += 1;
                }
            }
        }
    }

    println!("Sum: {}", sum);
}

#[derive(Debug)]
struct Coefficients {
    a0: i64,
    a1: i64,
}

impl From<(i64, i64)> for Coefficients {
    fn from((a0, a1): (i64, i64)) -> Self {
        Coefficients { a0, a1 }
    }
}

#[derive(Debug)]
struct Hailstone {
    x: Coefficients,
    y: Coefficients,
    z: Coefficients,
}

impl From<((i64, i64), (i64, i64), (i64, i64))> for Hailstone {
    fn from(((x0, x1), (y0, y1), (z0, z1)): ((i64, i64), (i64, i64), (i64, i64))) -> Self {
        Hailstone {
            x: (x0, x1).into(),
            y: (y0, y1).into(),
            z: (z0, z1).into(),
        }
    }
}

impl Hailstone {
    fn intersects_at_xy(&self, other: &Hailstone) -> Option<(f64, f64)> {
        // self.x.a1*a + self.x.a0 = other.x.a1*b + other.x.a0;
        //
        // let b = (self.x.a1 * a + self.x.a0 - other.x.a0) / other.x.a1;
        //
        // self.y.a1*a + self.y.a0 = other.y.a1 * ((self.x.a1 * a + self.x.a0 - other.x.a0) / other.x.a1) + other.y.a0;
        //
        // self.y.a1 * a + self.y.a0 = ([(other.y.a1 * self.x.a1 * a) + (other.y.a1 * self.x.a0) - (other.y.a1 * other.x.a0)] / other.x.a1) + other.y.a0;
        //
        // (other.x.a1 * self.y.a1 * a) + (other.x.a1 * self.y.a0) = (other.y.a1 * self.x.a1 * a) + (other.y.a1 * self.x.a0) - (other.y.a1 * other.x.a0) + (other.x.a1 * other.y.a0);
        //
        // (other.x.a1 * self.y.a1 * a) - (other.y.a1 * self.x.a1 * a) = (other.y.a1 * self.x.a0) - (other.y.a1 * other.x.a0) + (other.x.a1 * other.y.a0) - (other.x.a1 * self.y.a0);
        //
        // (other.x.a1 * self.y.a1 - other.y.a1 * self.x.a1) * a = (other.y.a1 * self.x.a0) - (other.y.a1 * other.x.a0) + (other.x.a1 * other.y.a0) - (other.x.a1 * self.y.a0);

        let a = ((other.y.a1 * self.x.a0) - (other.y.a1 * other.x.a0) + (other.x.a1 * other.y.a0) - (other.x.a1 * self.y.a0)) as f64 / (other.x.a1 * self.y.a1 - other.y.a1 * self.x.a1) as f64;
        let b = (self.x.a1 as f64 * a + self.x.a0 as f64 - other.x.a0 as f64) / other.x.a1 as f64;

        if a < 0.0  || b < 0.0 || a == f64::INFINITY || a == f64::NEG_INFINITY {
            // println!("Path crossed in the past for hailstone one of the Hailstones or they are Parallel");
            return None;
        }

        let x = self.x.a1 as f64 * a + self.x.a0 as f64;
        let y = self.y.a1 as f64 * a + self.y.a0 as f64;

        Some((x, y))
    }
}
