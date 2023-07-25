
/* 矩形，长方形 */
pub struct Rectangle {
    pub width:f64, 
    pub height: f64
}

/* 圆 */
pub struct Circle {
    pub radius: f64
}

/* 直角三角形 */
pub struct RightAngleTriangle {
    pub base: f64, 
    pub height: f64
}

/* 枚举包裹三个不同类型：矩形、圆、直角三角形 */
pub enum ShapeEnum {
    Rectangle(Rectangle),
    Circle(Circle),
    RightAngleTriangle(RightAngleTriangle), 
}

//
impl Rectangle {
    pub fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

impl RightAngleTriangle {
    pub fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}