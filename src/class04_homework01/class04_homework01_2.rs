//
pub trait Shape {
    //返回值 (图形, 面积)
    fn area(&self) -> (String, f64);
}

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
impl Shape for Rectangle {
    fn area(&self) -> (String, f64) {
        (String::from("Rectangle"), self.width * self.height)
    }
}

impl Shape for Circle {
    fn area(&self) -> (String, f64) {
        (String::from("Circle"), std::f64::consts::PI * self.radius.powi(2))
    }
}

impl Shape for RightAngleTriangle {
    fn area(&self) -> (String, f64) {
        (String::from("RightAngleTriangle"), 0.5 * self.base * self.height)
    }
}

//
pub fn sum_areas(a: &dyn Shape) -> (String, f64){
    a.area()
}