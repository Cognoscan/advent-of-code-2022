use std::{ops::{Add, Sub, SubAssign, AddAssign, RangeInclusive}, collections::HashSet};


#[derive(Clone, PartialEq, Eq, Copy, Hash, Default, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl From<(i32,i32)> for Pos {
    fn from(pos: (i32,i32)) -> Self {
        Self { x: pos.0, y: pos.1 }
    }
}

impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x+rhs.x, y: self.y+rhs.y }
    }
}

impl Sub<Pos> for Pos {
    type Output = Pos;
    fn sub(self, rhs: Pos) -> Self::Output {
        Pos { x: self.x-rhs.x, y: self.y-rhs.y }
    }
}

impl SubAssign<Pos> for Pos {
    fn sub_assign(&mut self, rhs: Pos) {
        self.x-=rhs.x;
        self.y-=rhs.y;
    }
}

impl AddAssign<Pos> for Pos {
    fn add_assign(&mut self, rhs: Pos) {
        self.x+=rhs.x;
        self.y+=rhs.y;
    }
}

const X_RANGE: RangeInclusive<i32> = -180..=5;
const Y_RANGE: RangeInclusive<i32> = -14..=350;

/*
            for x in X_RANGE {
                for y in Y_RANGE {
                    if head.x == x && head.y == y {
                        print!("H");
                    }
                    else {
                        let mut disp = false;
                        for (idx, knot) in knots.iter().enumerate() {
                            if knot.x == x && knot.y == y {
                                print!("{}", idx+1);
                                disp = true;
                                break;
                            }
                        }
                        if !disp { print!("."); }
                    }
                }
                println!();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
*/

fn simulate(input: &str, knots: usize, draw: bool) -> usize {
    let mut head = Pos::default();
    let mut knots = vec![Pos::default(); knots-1];

    let mut visited = HashSet::new();
    for line in input.lines() {
        let (dir, count) = line.split_once(' ').unwrap();
        let count = count.parse::<i32>().unwrap();

        if draw {
            print!("\x1b[2J");
            for x in X_RANGE.step_by(2) {
                for y in Y_RANGE.step_by(2) {
                    let mut set = 0;
                    if x == 0 && y == 0 { set |= 0x1; }
                    if head.x == x   && head.y == y   { set |= 0x1; }
                    if head.x == x   && head.y == y+1 { set |= 0x2; }
                    if head.x == x+1 && head.y == y   { set |= 0x4; }
                    if head.x == x+1 && head.y == y+1 { set |= 0x8; }
                    for knot in knots.iter() {
                        if knot.x == x   && knot.y == y   { set |= 0x1; }
                        if knot.x == x   && knot.y == y+1 { set |= 0x2; }
                        if knot.x == x+1 && knot.y == y   { set |= 0x4; }
                        if knot.x == x+1 && knot.y == y+1 { set |= 0x8; }
                    }
                    match set {
                        0x1 => print!("▘"),
                        0x2 => print!("▝"),
                        0x3 => print!("▀"),
                        0x4 => print!("▖"),
                        0x5 => print!("▌"),
                        0x6 => print!("▞"),
                        0x7 => print!("▛"),
                        0x8 => print!("▗"),
                        0x9 => print!("▚"),
                        0xA => print!("▐"),
                        0xB => print!("▜"),
                        0xC => print!("▄"),
                        0xD => print!("▙"),
                        0xE => print!("▟"),
                        0xF => print!("█"),
                        _ => print!(" "),
                    }
                }
                println!();
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        for _ in 0..count {

            match dir {
                "U" => head -= (0, 1).into(),
                "D" => head += (0, 1).into(),
                "L" => head -= (1, 0).into(),
                "R" => head += (1, 0).into(),
                _ => (),
            }

            let mut lead_pos = head;
            for knot in knots.iter_mut() {
                let delta = lead_pos - *knot;
                if delta.x.abs() > 2 { panic!(); }
                if delta.y.abs() > 2 { panic!(); }
                match delta {
                    Pos { x: 2, y: 2 } => *knot += (1,1).into(),
                    Pos { x: 2, y: -2 } => *knot += (1,-1).into(),
                    Pos { x: -2, y: 2 } => *knot += (-1,1).into(),
                    Pos { x: -2, y: -2 } => *knot += (-1,-1).into(),
                    Pos { x: 2, y } => *knot += (1,y).into(),
                    Pos { x: -2, y } => *knot += (-1,y).into(),
                    Pos { x, y: 2 } => *knot += (x,1).into(),
                    Pos { x, y: -2 } => *knot += (x,-1).into(),
                    _ => (),
                }
                lead_pos = *knot;
            }
            visited.insert(*knots.last().unwrap());
        }
    }

    if draw {
        print!("\x1b[2J");
        for x in X_RANGE.step_by(2) {
            for y in Y_RANGE.step_by(2) {
                let mut set = 0;
                if visited.contains(&Pos{x:x+0, y:y+0}) { set |= 0x1; }
                if visited.contains(&Pos{x:x+0, y:y+1}) { set |= 0x2; }
                if visited.contains(&Pos{x:x+1, y:y+0}) { set |= 0x4; }
                if visited.contains(&Pos{x:x+1, y:y+1}) { set |= 0x8; }
                match set {
                    0x1 => print!("▘"),
                    0x2 => print!("▝"),
                    0x3 => print!("▀"),
                    0x4 => print!("▖"),
                    0x5 => print!("▌"),
                    0x6 => print!("▞"),
                    0x7 => print!("▛"),
                    0x8 => print!("▗"),
                    0x9 => print!("▚"),
                    0xA => print!("▐"),
                    0xB => print!("▜"),
                    0xC => print!("▄"),
                    0xD => print!("▙"),
                    0xE => print!("▟"),
                    0xF => print!("█"),
                    _ => print!(" "),
                }
            }
            println!();
        }
        let mut xmin = 0;
        let mut xmax = 0;
        let mut ymin = 0;
        let mut ymax = 0;
        for p in visited.iter() {
            xmax = p.x.max(xmax);
            xmin = p.x.min(xmin);
            ymax = p.y.max(ymax);
            ymin = p.y.min(ymin);
        }
        println!("x: {} to {}", xmin, xmax);
        println!("y: {} to {}", ymin, ymax);
    }
    visited.len()
}

fn main() {
    let input = include_str!("input");

    let part1 = simulate(input, 2, false);
    let part2 = simulate(input, 10, true);
    println!("Part 1: {} locations", part1);
    println!("Part 2: {} locations", part2);
}
