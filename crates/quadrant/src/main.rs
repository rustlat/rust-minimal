struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn show_quad(self) {
        let phi = (self.y / self.x).atan().to_degrees();
        let quad = (phi / 180.0).round() as i32 % 4;

        match quad {
            0 => println!("Quadrant I"),
            1 => println!("Quadrant II"),
            2 => println!("Quadrant III"),
            3 => println!("Quadrant IV"),
            _ => println!("It's the center!"),
        }
    }
}

fn main() {
    let p = Point { x: 1.0, y: -1.0 };

    p.show_quad();
}
