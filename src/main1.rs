
trait Shape {
    fn area(&self) -> f64;
}

/* 矩形，长方形 */
struct Rectangle {
    width:f64, 
    height: f64
}

/* 圆 */
struct Circle {
    radius: f64
}

/* 直角三角形 */
struct RightAngleTriangle {
    base: f64, 
    height: f64
}

enum Shape_enum {
    Rectangle(Rectangle),
    Circle(Circle),
    RightAngleTriangle(RightAngleTriangle), 
}

//
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl Shape for RightAngleTriangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

//
fn sum_areas(a: &dyn Shape) -> f64{
    a.area()
}

fn main() {
    let rectangle: Rectangle = Rectangle{width:5.5, height:4.4};
    let circle = Circle{radius:5.5};
    let rightAngleTriangle = RightAngleTriangle{base:5.5, height:4.4};
    //
    let ans:Vec<&dyn Shape> = vec![&rectangle, &circle, &rightAngleTriangle];
    //
    for item in ans {
        // 
        let sum = sum_areas(item);
        println!("area: {}", sum);
        //
        println!("area2: {}",item.area());
    }
    
}