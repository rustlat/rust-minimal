enum QuadrantNum {
    Center,
    I,
    II,
    III,
    IV,
}

trait Quadrant {
    fn get_quadrant(self) -> QuadrantNum;
}

struct Point {
    x: f32,
    y: f32,
}

impl Quadrant for Point {
    fn get_quadrant(self) -> QuadrantNum {
        if self.x > 0.0 && self.y >= 0.0 {
            QuadrantNum::I
        } else if self.x <= 0.0 && self.y > 0.0 {
            QuadrantNum::II
        } else if self.x < 0.0 && self.y <= 0.0 {
            QuadrantNum::III
        } else if self.x >= 0.0 && self.y < 0.0 {
            QuadrantNum::IV
        } else {
            QuadrantNum::Center
        }
    }
}

impl Point {
    fn show_quad(self) {
        match self.get_quadrant() {
            QuadrantNum::I => println!("Quadrant I"),
            QuadrantNum::II => println!("Quadrant II"),
            QuadrantNum::III => println!("Quadrant III"),
            QuadrantNum::IV => println!("Quadrant IV"),
            QuadrantNum::Center => println!("It's not in a quadrant"),
        }
    }
}

fn main() {
    let p = Point { x: 0.0, y: 1.0 };
    // p.x = 13.0;

    p.show_quad();
}
