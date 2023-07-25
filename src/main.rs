
mod class04_homework01;

use class04_homework01::class04_homework01_1;
use class04_homework01::class04_homework01_2;


fn main() {
   //
   
}

/*
 * 第四节课作业
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
            class04_homework01_1::ShapeEnum::Circle(c) => println!("Circle area is {}.", c.area()),
            class04_homework01_1::ShapeEnum::RightAngleTriangle(rat) => println!("RightAngleTriangle area is {}.", rat.area()),
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