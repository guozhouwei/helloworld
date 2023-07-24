mod asciiutil;

use std::f32::consts::E;
use std::fmt::Display;
use std::io::{self, stdout};
use std::collections::HashMap;
use std::{string, result};

/*
 * 课程
 */
#[derive(Debug)]
struct Course {
    name: String,
    compulsory: bool,  //true 必修课，false 选修课
    teacher_name: String,  //授课老师
}

/*
 * 社团
 */
#[derive(Debug)]
struct Ssociation {
    name: String,   //社团名称
    aim: String,  //社团目标
}

/*
 * 班级
 */
#[derive(Debug)]
struct GradeClass {
    name:String, //班级名称
    grade: String,  //年级
    faculty: String,   //院系
    speciality: String,   //专业
    instructor: String, //辅导员
}

/*
 * 性别
 */
#[derive(Debug)]
enum Gender {
    Male,   // 男
    Female, // 女
  }

/*
 * 学生
 */
#[derive(Debug)]
struct Student {
    name: String,  //姓名
    age: u8,   //年龄
    sex: Gender,   //性别
    grade_class: GradeClass, //班级
    ssociations: Vec<Ssociation>, //参加的社团
    courses: Vec<Course>, //学习的课程
}

/**
 * 第三课作业：
 * 请基于Rust的基本数据结构写一个简单的学生管理系统（比如，学生，社团，班级、课程等），明确类型之间的关系，进行基本的CRUD操作
 */
fn main() {

    println!("step 1 创建课程:");
    let mut course_map= HashMap::new();
    
    /*
     * step 1  创建课程
     * 英语，计算机，高数，数字电路
     */
    //loop {
        println!("请输入课程名:");
        let mut input = String::new(); //课程名        
        io::stdin().read_line(&mut input).expect("error");
        let name: String = input.trim().to_string();
        //
        println!("是否是必修课（yes/no）:");
        let mut input1 = String::new(); //是否必修课
        io::stdin().read_line(&mut input1).expect("error");
        let mut compulsory = false;
        if "yes" == input1.trim() {
            compulsory = true;
        }
        //
        let course = Course {
                                        name,
                                        compulsory,
                                        teacher_name: String::from("张三"),
                                    };
        course_map.insert("数学", &course);

        //
        println!("是否继续创建课程（yes/no）:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("error");
        let continue_create = input2.trim();
        if "yes" == continue_create {
            println!("继续创建课程。。。");
        } else {
            println!("课程创建完毕。。。");
            
           // break course_map
        }
    //};
    //打印课程
    for (key, value) in course_map {
        println!("课程:{}, 是否必修课:{}, 教师:{}", value.name, value.compulsory, value.teacher_name);
    }
    
    /*
     * step 2 创建社团
     * 学生会，宣传部，吉他社，轮滑社
     */
        println!("请输入社团名:");
        let mut input2 = String::new(); //课程名        
        io::stdin().read_line(&mut input2).expect("error");
        let ssociation_name: String = input2.trim().to_string();
        //
        println!("请输入社团宗旨:");
        let mut input3 = String::new(); 
        io::stdin().read_line(&mut input3).expect("error");
        let aim: String = input3.trim().to_string();
        //
        let ssociation = Ssociation {
            name : ssociation_name,
            aim,
        };

        println!("社团名：{}, 社团宗旨:{}", ssociation.name, ssociation.aim);

  
    /*
     * step 3 班级
     * 院系，年级，专业，辅导员
     */
    println!("创建班级:");
        println!("请输入院系:");
        let mut input4 = String::new();       
        io::stdin().read_line(&mut input4).expect("error");
        let grade: String = input4.trim().to_string();
        //
        println!("请输入年级:");
        let mut input5 = String::new(); 
        io::stdin().read_line(&mut input5).expect("error");
        let faculty: String = input5.trim().to_string();
        //
        println!("请输入专业:");
        let mut input6 = String::new(); 
        io::stdin().read_line(&mut input6).expect("error");
        let speciality: String = input6.trim().to_string();
        //
        println!("请输入辅导员姓名:");
        let mut input7 = String::new(); 
        io::stdin().read_line(&mut input7).expect("error");
        let instructor: String = input7.trim().to_string();
        //

        let grade_class = GradeClass {
                            name:String::from("小二班"),
                            grade,  
                            faculty,   
                            speciality,   
                            instructor, 
                        };

        println!("班级: {}-{}-{}-{}", grade_class.grade, grade_class.faculty, grade_class.speciality, grade_class.instructor);
    
    /*
     * step 4 学生
     * 姓名，年龄，专业，
     * 加入2023年级计算机1班级
     * 加入吉他社，舞蹈社
     * 学习计算机课程，舞蹈课程
     * 删除课程再添加课程
     */
    // println!("录入学生信息:");
    //     println!("请输入学生姓名:");
    //     let mut input8 = String::new();       
    //     io::stdin().read_line(&mut input8).expect("error");
    //     let name: String = input8.trim().to_string();                  
    //     //
    //     println!("请输入年龄:");
    //     let mut input9 = String::new(); 
    //     io::stdin().read_line(&mut input9).expect("error");
    //     let age = input9.trim().parse().unwrap();
    //     //
    //     println!("请输入性别(男/女): ");
    //     let mut input10 = String::new(); 
    //     io::stdin().read_line(&mut input10).expect("error");
    //     let sex_str = input10.trim();
    //     let sex = Gender::Male;
    //     if "女" == sex_str {
    //         sex = Gender::Female;
    //     }

    //     //
    //     let mut s1 = "请选择班级: ".to_string();
    //     let s2 = grade_class.name;
    //     s1 += &s2;
    //     println!("{}", s1);
        
    //     let mut input11 = String::new(); 
    //     io::stdin().read_line(&mut input11).expect("error");
    //     let grade_class01 = input11.trim();
    //     //
    //     if grade_class.name == grade_class01 {
            
    //     }
        //学习的课程 略

    // let student = Student {
    //     name,  //姓名
    //     age,   //年龄
    //     sex,   //性别 男 true，女 false 
    //     grade_class, //班级
    //     ssociations: Vec<Ssociation>, //参加的社团
    //     courses: Vec<Course>, //学习课程
    // };

    //println!("学生姓名:{}, 年龄: {}, 性别: {}, 班级:{}, 参加社团:{}, 学习课程:{}", student.name, student.age, student.sex, );

    

    
}





/*
* 思考题：
* 如何将字符串转换为字符数组?
* String -> [char] 或 Vec<char>
*/
pub fn str_convert_vec_and_array(para: &str) -> (Vec<char>, [char; 58]) {
    //1. Vec<char>
    let char_vec: Vec<char> = para.chars().collect::<Vec<_>>();

    //2. [char]
    let length = para.chars().count();
    assert_eq!(length, 58);
    //
    let mut charArray:[char; 58] = ['0'; 58];
    //
    let chars = para.chars();
    let mut i = 0;
    chars.for_each(|c| {
            charArray[i] = c;
            i += 1;
    });
    (char_vec, charArray)
}

    
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test () {
        /*
        * 作业
        * （1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
        * （2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
        * （3）使用Cargo编译运行此工程
        * （需要自行发现其中的细节，一个考点是：ascii码字符的顺序）
        */
        //添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
        let vec: Vec<char> = asciiutil::print_ascii_froma2Z();
        //
        println!("打印 Z~a:");
        //println!("{:?}",vec);
        //动态数组循环打印
        for ele in vec.iter() {
            print!("{ele}\t");
        }
        
        println!("");

        //添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
        let char_vec: Vec<char> = asciiutil::asciiprint::print_ascii_fromA2z();
        println!("打印 A~z:");
        for ele in char_vec.iter() {
            print!("{ele} \t");
        }
    }

    #[test]
    fn test_string_convert() {    
        let s = String::from("initial contents");
        let hello1 = String::from("عليكم السلام");
        let hello2 = String::from("Dobrý den");
        let hello3 = String::from("Hello");
        let hello4 = String::from("こんにちは");
        let hello5 = String::from("안녕하세요");
        let hello6 = String::from("你好");

        //拼接各种形式的字符
        let mut fix = String::from("$$$$");
        fix.push_str(&s);
        fix.push_str(&hello1);
        fix.push_str(&hello2);
        fix.push_str(&hello3);
        fix.push_str(&hello4);
        fix.push_str(&hello5);
        fix.push_str(&hello6);
        //
        println!("打印拼接后的字符串：{}", &fix);
        println!("");

        //各种字符组合,转成动态数组与char数组
        let (charVec, charArray) = str_convert_vec_and_array(&fix);
        println!("字符串转Vec<char>:{:?}", charVec);
        println!("");
        println!("字符串转[char]:{:?}", charArray);

    }


    #[test]
    fn test_char_string_length() {    
        //字符长度
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';

        println!("字符 'z' length={}字节", std::mem::size_of_val(&c));
        println!("字符 'ℤ' length={}字节", std::mem::size_of_val(&z));
        println!("字符 '😻' length={}字节", std::mem::size_of_val(&heart_eyed_cat));
        //字符串长度
        let s = "zℤ😻";
        println!("字符串 \"zℤ😻\" length={}字节", s.len());
        //字符串中字符(按照utf8)长度
        for (idx, item) in s.chars().enumerate() {
            println!("idx:{},item:{},item_utf8_length={}字节", idx, item, item.len_utf8());
        }

        //字符串转字符后长度
        for (idx, item) in s.chars().enumerate() {
            println!("idx:{},item:{},item_length={}字节", idx, item, std::mem::size_of_val(&item));
        }

    }
}

#[test]
fn testCase1() {
    let a = 10u32; // 创建所有者
    println!("stack上a的值: {}", a);
    println!("stack上a的地址: {:p}", &a);

    let b = &a; // 创建所有者的引用
    println!("stack上b的地址: {:p}", &b);
    println!("stack上b存储的a地址: {b:p}");
    
    let c = b; //复制引用 
    println!("stack上c的地址: {:p}", &c);
    println!("stack上c存储的b的数据（即a的地址）: {c:p}");

    let d = &b; 
    println!("stack上d的地址: {:p}", &d);
    println!("stack上d存储的b地址: {d:p}");
}
    



#[test]
fn testPrintCase01() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("b = {b}");
}

#[test]
fn testPrintCase02() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    println!("b = {b}");
    println!("a = {a}");
}

#[test]
fn testPrintCase03() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;

    //println!("a = {a}", &a);  //报错原因，可变引用与不可变引用不能同时存在！
    println!("b = {b}");
}

#[test]
fn testPrintCase04() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;
    let c = &a;

    //println!("b = {b}"); //b是可变引用，c是不可变引用，不能同时存在！
}

#[test]
fn testPrintCase05() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20;    //到此处a的可变引用 b 的作用域结束
    let c = &a;

    println!("c = {c}");
}

#[test]
fn testPrintCase06() {
    // let mut a = 10u32;
    // let c = &a;
    // let b = &mut a; //c 的作用域在方法最下面println方法，不能c不可变引用与b可变引用同时存在
    // *b = 20;
    

    // println!("c = {c}");
}

#[test]
fn testPrintCase07() {
    let mut a = 10u32;
    let c = &a;
    let b = &mut a;
    *b = 20;

    println!("b = {b}");
}

#[test]
fn testPrintCase08() {
    // let mut a = 10u32;
    // let b = &mut a;
    // *b = 20;
    // let d = &mut a;
    

    // println!("b = {b}");
}



#[test]
fn test_case_01() {
    let a = 10u32;
    let b = a;
    println!("{a}");
    println!("{b}")
}

#[test]
fn test_case_02 () {
    let s1 = String::from("I am a supperman.");
    //let s2 = s1;
    let s2 = s1.clone();
    println!("{s1}");
    println!("{s2}")
}

#[test]
fn test_case_03() {
    let a = 10u32;
    let b = &a;
    let c = &&&&&a;
    let d = &b;
    let e = b;
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
}

#[test]
fn test_case_04() {
    let s1 = String::from("I am a supperman");
    let s2 = &s1;
    let s3 = &&&&&s1;
    let s4 = &s2;
    let s5 = s2;
    println!("{s1}");
    println!("{s2}");
    println!("{s3}");
    println!("{s4}");
    println!("{s5}");
}


fn foo(s: &mut String) {
    s.push_str("You are batman.")
}

#[test]
fn test_case_05() {
    let mut s1 = String::from("I am a supperman.");
    println!("{s1}");
    foo(&mut s1);
    println!("{s1}");
}

#[test]
fn test_case_06() {
    let c = 0.3937f64;
    let cm = 66.0f64;
    let cun = c * cm;
    println!("{cun}");

}

#[test]
fn test_case_07() {
    let mut a = 10u32;
    let b = &mut a;
    *b = 20u32;

    println!("{}", a);
    //println!("{b}");


}

fn largest_u32(list:Vec<u32>)->u32 {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list:Vec<char>) -> char {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}



#[test]
fn test_case_08() {
    let number_list = vec![1,3,55,98,23,7];
    //
    let largest = largest_u32(number_list);
    println!("最大数：{}", largest);

    let char_list = vec!['y', 'a', 'U', 'z', 'Z'];
    let char_largest = largest_char(char_list);
    println!("最大字符：{}", char_largest);


}

fn largest<T: std::cmp::PartialOrd>(list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    for item in list {
        if *item > *largest {   //因为限定了T实现了PartialOrd 这个trait,
        //if item > largest {
            largest = item;
        }
    }
    largest
}

#[test]
fn test_case_09() {

    let number_list = vec![1,3,55,98,23,7];
    
    let largest = largest(&number_list);
    println!("最大数：{}", largest);

    let char_list = vec!['y', 'a', 'U', 'z', 'Z'];
    let largest = crate::largest(&char_list);
    println!("最大字符：{}", largest);
}

struct Point<T> {
    x:T,
    y:T,
}

#[test]
fn test_case_10() {
    let integer = Point{x:5, y:10};

    let xxx = Point{x:'a', y:'b'};
    
    let yyy = Point{x:1.1, y:1.0};
}

struct Point1<T, U> {
    x:T,
    y:U,
} 

#[test]
fn test_case_11() {
    let xxx = Point1{x:1, y:1.2};
    let yyy=  Point1{x:'a', y:1.23};
}

enum Option<T> {
    Some(T),
    None,
}
enum Result<T, U> {
    OK(T),
    ERROR(U),
}

struct Point2<T> {
    x:T,
    y:T
}

impl<T> Point2<T>  {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point2<f32> {
    fn x1(&self) -> f32 {
        println!("powi:{}", &self.x.powi(2));
        (&self.x.powi(2) + &self.y.powi(2)).sqrt()
    }
}

#[test]
fn test_case_12() {
    let point = Point2{x:8, y:3};
    println!("point2.x={}", point.x());
    //实现单态化方法
    let point2 = Point2{x:8.0, y:4.0};
    println!("x1={}", point2.x1());
}

struct Point3<X1, Y1> {
    x:X1,
    y:Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn minup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
    
}

#[test]
fn test_case_13() {
    let p = Point3{x:10, y:22};
    let p2 = Point3{x:"zhouzhou", y:"xiaoming"};
    let p3 = p.minup(p2);
    println!("p3.x={}, p3.y={}", p3.x, p3.y);
}


//--------------------
struct Pair<T> {
    x:T,
    y:T,
}

impl <T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self { x, y}
    }
}

impl<T:Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest memeber is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
}

//----------------
trait Atr {
    fn foo(&self);
}

struct Foo;

impl Atr for Foo {
    fn foo(&self) {
        println!("aaaa");
    }
}

#[test]
fn test_case_14() {
    let foo = Foo{};
    foo.foo();
}

//------------------
trait StreamingIterator {
    type  Item;
}

struct Foo1<T> 
    where T: StreamingIterator<Item = String> {
    x:T
}

struct A;
impl StreamingIterator for A {
    type  Item = String;
}

#[test]
fn test_case_15() {
    let a = Foo1::<A> {
        x:A,
    };
}

//-------------------------
trait Animal {
    fn talk(&self);
}

struct Cat {}
struct  Dog{}

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

//----------
trait Animal1 {
    fn nop(&self);
}

struct Cat1 {}

impl Animal1 for Cat1 {
    fn nop(&self) {
        
    }
}

#[test]
fn test_case_17() {
    let cat1 = Cat1{};
    let ct : &dyn Animal1 = &cat1;
    cat1.nop();
}

//-----------
struct Sheep {}

struct Cow{}

trait Animal2 {
    fn noise(&self) -> &'static str;
}

impl Animal2 for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

impl Animal2 for Cow {
    fn noise(&self) -> &'static str {
        "mooooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal2> {
    if random_number < 0.5 {
        Box::new(Sheep{})
    } else {
        Box::new(Cow{})
    }
}

#[test]
fn test_case_18() {
    let random_number = 0.25;
    let animal2 = random_animal(random_number);
    println!("You have randomly chosen an animal, and it says {}", animal2.noise());
}

//-------------