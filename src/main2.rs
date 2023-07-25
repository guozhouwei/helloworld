
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}

// fn point_add(a: &dyn Add, point:Point) -> Self {
//     a.add(point)
// }

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 },   Point { x: 3, y: 3 });
    
}