
mod class04_homework01;
mod class03_homework;

use class04_homework01::class04_homework01_1;
use class04_homework01::class04_homework01_2;

use class03_homework::class03_homework01;

// macro_rules! add_as2 {
//     ( $($a:expr), * ) => {
//         {
//             0 $(+$a)*
//         }
//     }
// }

//转map
use std::collections::HashMap;

macro_rules! convert_to_map {
    ($($key:expr => $value:expr), *) => {
        {
            let mut person_map = HashMap::new();
            $(person_map.insert($key, $value);) *
            person_map   
        }
    };
}

fn main() {
   //
//    let sum = add_as2!(1,2,3,4,5,6,7,8,9);
//     println!("sum = {}", sum);

    let person = convert_to_map!{
        "name" => "zhouzhou",
        "gender" => "man",
        "address" => "beijing"
    };

    println!("{:?}", person);
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

    #[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
   
   //todo 范型方式实现，如何用 trait object 实现呢？？？
    pub fn point_add<T: Add<Output = T>> (point1: T, point2: T) -> T{
        point1.add(point2)
    }

   let a = Point { x: 1, y: 0 } ;
   let b = Point { x: 2, y: 3 } ;

   let c = point_add(a, b);
   println!("Point(x:{},y:{}) + Point(x:{}, y:{}) = Point(x:{}, y:{})", a.x, a.y, b.x, b.y, c.x, c.y);
   
}


#[test]
fn test_case_homework_01() {
    let person = convert_to_map!{
        "name" => "zhouzhou",
        "gender" => "man",
        "address" => "beijing"
    };

    println!("{:?}", person);

}



//--------------第五课 课堂笔记-----------------
static BSTRING: &'static str = "abc";
fn foo() -> &'static str {
    //let s = String::from("abc");
    BSTRING
}

#[test]
fn test_case_03() {
    //
    let s = foo();
}

fn foo1(s: &str) -> &str {
    if s.len() > 5 {
        &s[..5]
    } else {
        s
    }
}

#[test]
fn test_case_04() {
    let s = String::from("abcdef");
    //
    println!("{}", foo1(&s));
}

fn foo2(s: &str) -> &str {
    s
}

#[test]
fn test_case_05() {
    //let result;
    // {
    //     let s = String::from("abcdef");
    //     result = foo2(&s);
    // }
    //println!("{}", result);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[test]
fn test_case_06() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximun is configured to be {}", max),
        _ =>(),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximun is configured to be {}", max);
    }

    let a = UsState::Alabama;
    let coin = Coin::Quarter(a);

    let mut count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }


}


#[test]
fn test_case_07() {
    //let x = Some("value");
    let x:Option<&str> = None;
    assert_eq!(x.expect("fruits are healthy123"), "value");
}

// use std::fs::File;
// use std::io::{self, Read};

// fn read_username_from_file() -> Result<String, io::Error> {
//     let username_file_result = File::open("/Users/zhouzhou/rust_workspace/helloworld/hello.txt");

//     let mut username_file = match username_file_result {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut username = String::new();

//     match username_file.read_to_string(&mut username) {
//         Ok(_) => Ok(username),
//         Err(e) => Err(e),
//     }
    
// }

// #[test]
// fn test_case_08() {
//     let username = read_username_from_file();
//     match username {
//         Ok(v) => print!("ok = {}", v),
//         Err(e) => println!("err = {}", e),
//     }
// }


use std::convert;
use std::fs::File;
use std::io::{self, Read};
use std::str::Chars;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("/Users/zhouzhou/rust_workspace/helloworld/hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    let a = "100".parse::<u32>().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, "oh no"))?;
    Ok(username)
}

#[test]
fn test_case_09() {
    let result = read_username_from_file();
    match result {
             Ok(v) => println!("ok = {}", v),
             Err(e) => println!("err = {}", e),
    }
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

#[test]
fn test_case_10() {
    let a = "mssssp
    bbb
    ffff
    7777
    uuuuu
    ...";

    let b = last_char_of_first_line(a);
    println!("***: {:?}", b)
}

macro_rules! add {
    ($a: expr, $b: expr) => {
        {
            $a+$b
        }
    };

    ($a: expr) => {
        {
            $a
        }
    }
}


#[test]
fn test_case_11() {
    let sum = add!(1,2);
    println!("1 + 2 = {}", sum);

    let third = add!(9);
    println!("{}", third);
}

macro_rules! add_as {
    ($a: expr, $b: expr, $type:ty) => {
        {
            $a as $type + $b as $type
        }
    };
}

#[test]
fn test_case_12() {
    let sum = add_as!(8, -9, i32);
    println!("8+9={}", sum);
}

macro_rules! add_as1 {
    ( $($a:expr), * ) => {
        {
            0 $(+$a)*
        }
    }
}

#[test]
fn test_case_13() {
    let sum = add_as1!(1,2,3,4,5,6,7,8,9);
    println!("sum = {}", sum);
}

macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

#[test]
fn test_case_14() {
    calculate! {
        eval 1 +2
    }

    calculate!{
        eval (1 + 2) * (3 /4)
    }
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
//实现单态化方法
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[test]
fn test_case_15() {
    let p = Point { x: 5.5, y: 10.1 };
    //
    println!("p.x = {}", p.x());
    println!("p.distance_from_origin = {}", p.distance_from_origin());
}


trait Animal {
    fn talk(&self);
}

struct Cat;
struct  Dog;

impl Animal for Cat {
    fn talk(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn talk(&self) {
        println!("bark");
    }
}

//fn animal_talk (a: Box<dyn Animal>) {
fn animal_talk (a: &dyn Animal) {
//fn animal_talk<T: Animal>(a: &T){
    a.talk();
}

#[test]
fn test_case_16() {
    let cat = Cat{};
    let dog = Dog{};
    animal_talk(&cat);
    animal_talk(&dog);
    //animal_talk(Box::new(cat));
    //animal_talk(Box::new(dog));

    let ans:Vec<&dyn Animal> = vec![&cat, &dog];
}
