
mod class04_homework01;

use class04_homework01::class04_homework01_1;
use class04_homework01::class04_homework01_2;


fn main() {
   //
   
}

/*
 * 第四节课作业 第一题
 * 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
 * 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
 */
 #[test]
 fn test_case_01() {

    //第一种方式：使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let mut vec1 : Vec<class04_homework01_1::ShapeEnum> = vec![
            class04_homework01_1::ShapeEnum::Rectangle(class04_homework01_1::Rectangle {width:5.5, height:4.4}),
            class04_homework01_1::ShapeEnum::Circle(class04_homework01_1::Circle {radius:5.5}),
            class04_homework01_1::ShapeEnum::RightAngleTriangle(class04_homework01_1::RightAngleTriangle{base:5.5, height:4.4}),
        ];
    for shape_enum in vec1 {
        match shape_enum {
            class04_homework01_1::ShapeEnum::Rectangle(r) => println!("Rectangle area is {}", r.area()),
            class04_homework01_1::ShapeEnum::Circle(c) => println!("Circle area is {}", c.area()),
            class04_homework01_1::ShapeEnum::RightAngleTriangle(rat) => println!("RightAngleTriangle area is {}", rat.area()),
        }
    }

    println!("----------------");

    //第二种方式：定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let rectangle = class04_homework01_2::Rectangle{width:5.5, height:4.4};
    let circle = class04_homework01_2::Circle{radius:5.5};
    let rightAngleTriangle = class04_homework01_2::RightAngleTriangle{base:5.5, height:4.4};
    //
    let vec2:Vec<&dyn class04_homework01_2::Shape> = vec![&rectangle, &circle, &rightAngleTriangle];
    //
    for item in vec2 {
        // 
        let (shape, area) = class04_homework01_2::sum_areas(item);
        //let (shape, area) = item.area();
        println!("{} area is {}", shape, area);
    }

 }

 /*
 * 第四节课作业 第二题
 * 搜索相关文档，为你自己定义的一个类型或多个类型实现加法运算（用符号 +），并构思使用Trait Object实现类型方法的调用
 * 
 * 学习资料：
 * https://rustwiki.org/zh-CN/core/ops/trait.Add.html
 * 
 * 
 * pub trait Add<Rhs = Self> {
 *   type Output;
 *   fn add(self, rhs: Rhs) -> Self::Output;
 *  }
 */
#[test]
fn test_case_02() {
    
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
       x: i32,
       y: i32,
    }
   
    impl Add for Point {
       type Output = Self;
       fn add(self, other:Self) -> Self {
           Self {
               x:self.x + other.x,
               y:self.y + other.y,
           }
       }
    }
   
    pub fn point_add<T: Add<Output = T>> (point1: T, point2: T) -> T{
        point1.add(point2)
    }

   let a = Point { x: 1, y: 0 } ;
   let b = Point { x: 2, y: 3 } ;

   let c = point_add(a, b);
   println!("Point(x:{},y:{}) + Point(x:{}, y:{}) = Point(x:{}, y:{})", a.x, a.y, b.x, b.y, c.x, c.y);
   
}



