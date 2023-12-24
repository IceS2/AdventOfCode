use std::{fs, ops::Range, collections::HashSet};

fn main() {
    let mut hailstones: Vec<Hailstone> = fs::read_to_string("input.txt").unwrap()
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

    // Based on https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/keqf8uq/
    hailstones.sort();


    let mut potential_velocities_x: HashSet<i64> = HashSet::new();
    let mut potential_velocities_y: HashSet<i64> = HashSet::new();
    let mut potential_velocities_z: HashSet<i64> = HashSet::new();

    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            if hailstones[i].x.a1 == hailstones[j].x.a1 && hailstones[i].x.a1.abs() > 100 {
                let mut potential_x_set: HashSet<i64> = HashSet::new();
                let distance = hailstones[i].x.a0 - hailstones[j].x.a0;

                for v in -1000..=1000 {
                    if v == hailstones[i].x.a1 {
                        continue;
                    }

                    if distance % (v - hailstones[i].x.a1) == 0 {
                        potential_x_set.insert(v);
                    }
                }

                if !potential_velocities_x.is_empty() {
                    potential_velocities_x = potential_velocities_x.intersection(&potential_x_set).copied().collect();
                } else {
                    potential_velocities_x = potential_x_set.clone();
                }
            }
            if hailstones[i].y.a1 == hailstones[j].y.a1 && hailstones[i].y.a1.abs() > 100 {
                let mut potential_y_set: HashSet<i64> = HashSet::new();
                let distance = hailstones[i].y.a0 - hailstones[j].y.a0;

                for v in -1000..=1000 {
                    if v == hailstones[i].y.a1 {
                        continue;
                    }

                    if distance % (v - hailstones[i].y.a1) == 0 {
                        potential_y_set.insert(v);
                    }
                }

                if !potential_velocities_y.is_empty() {
                    potential_velocities_y = potential_velocities_y.intersection(&potential_y_set).copied().collect();
                } else {
                    potential_velocities_y = potential_y_set.clone();
                }
            }
            if hailstones[i].z.a1 == hailstones[j].z.a1 && hailstones[i].z.a1.abs() > 100 {
                let mut potential_z_set: HashSet<i64> = HashSet::new();
                let distance = hailstones[i].z.a0 - hailstones[j].z.a0;

                for v in -1000..=1000 {
                    if v == hailstones[i].z.a1 {
                        continue;
                    }

                    if distance % (v - hailstones[i].z.a1) == 0 {
                        potential_z_set.insert(v);
                    }
                }

                if !potential_velocities_z.is_empty() {
                    potential_velocities_z = potential_velocities_z.intersection(&potential_z_set).copied().collect();
                } else {
                    potential_velocities_z = potential_z_set.clone();
                }
            }
        }
    }
    println!("{:?}", potential_velocities_x);
    println!("{:?}", potential_velocities_y);
    println!("{:?}", potential_velocities_z);

    let h1 = hailstones[0].clone();
    let h2 = hailstones[1].clone();
    println!("{:?}", h1);
    println!("{:?}", h2);

    let vx = potential_velocities_x.iter().copied().collect::<Vec<i64>>().pop().unwrap();
    let vy = potential_velocities_y.iter().copied().collect::<Vec<i64>>().pop().unwrap();
    let vz = potential_velocities_z.iter().copied().collect::<Vec<i64>>().pop().unwrap();

    let ma = (h1.y.a1 - vy) as f64 / (h1.x.a1 - vx) as f64;
    let mb = (h2.y.a1 - vy) as f64 / (h2.x.a1 - vx) as f64;
    println!("{} {}", ma, mb);

    let ca = h1.y.a0 as f64 - ma * h1.x.a0 as f64;
    let cb = h2.y.a0 as f64 - mb * h2.x.a0 as f64;

    let xpos = (cb - ca)/(ma - mb);
    let ypos = ma * xpos + ca;

    let time = (xpos - h1.x.a0 as f64) / (h1.x.a1 - vx) as f64;

    let zpos = h1.z.a0 as f64 + (h1.z.a1 - vz) as f64 * time;

    println!("{} {} {}", xpos, ypos, zpos);
    println!("Sum: {}", xpos.abs() + ypos.abs() + zpos.abs());

}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Coefficients {
    a0: i64,
    a1: i64,
}

impl From<(i64, i64)> for Coefficients {
    fn from((a0, a1): (i64, i64)) -> Self {
        Coefficients { a0, a1 }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
