struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
enum Quadrant {
    I,
    II,
    III,
    IV,
    Center,
}

fn num_sign(x: f32) -> i32 {
    if x != 0.0 {
        x.signum() as i32
    } else {
        0
    }
}

impl Point {
    fn get_quad(self) -> Quadrant {
        match (num_sign(self.x), num_sign(self.y)) {
            (1, 1) => Quadrant::I,
            (-1, 1) => Quadrant::II,
            (-1, -1) => Quadrant::III,
            (1, -1) => Quadrant::IV,
            (0, 0) => Quadrant::Center,
            (1, 0) | (0, 1) => Quadrant::I,
            (-1, 0) | (0, -1) => Quadrant::III,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let p = Point { x: -1.0, y: -1.0 };

    let q = p.get_quad();
    println!("Quadrant {:?}", q);
}
