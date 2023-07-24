mod asciiutil;

use std::io::{self, stdout};
use std::collections::HashMap;
use std::string;

/*
 * 课程
 */
//#[derive(Debug)]
struct Course {
    name: String,
    compulsory: bool,  //true 必修课，false 选修课
    teacher_name: String,  //授课老师
}


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
        if("yes" == input1.trim()) {
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
        if("yes" == continue_create) {
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

    /*
     * step 3 班级
     * 院系，年级
     */

    /*
     * step 4 学生
     * 姓名，年龄，专业，
     * 加入2023年级计算机1班级
     * 加入吉他社，舞蹈社
     * 学习计算机课程，舞蹈课程
     * 删除课程再添加课程
     */

    
}


/*
 * 打印 HashMap
 */
fn print_courses(map : &HashMap<&String, &Course>) {
    println!("课程如下:");
    for (key, value) in &*map {
        print!("{} / {}", key, value.name);
    }
}

// pub fn create_course (courses: &mut HashMap) {

// }





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