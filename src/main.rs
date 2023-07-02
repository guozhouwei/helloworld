use std::string;

fn main() {
    /*
     * （1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
     * （2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
     * （3）使用Cargo编译运行此工程
     * （需要自行发现其中的细节，一个考点是：ascii码字符的顺序）
     */
    println!("打印 Z~a:");
    for ascii1 in ('Z'..='a').rev() {
        print!("{ascii1} ");
    }
    println!("");
    println!("打印 A~z:");
    
    for ascii2 in 'A'..='z' {
        print!("{ascii2}");
    }

}

#[test]
/*
* 思考题：
* 如何将字符串转换为字符数组?
* String -> [char] 或 Vec<char>
*/
fn test_string_convert_to_() {    
    let s = String::from("initial contents");
    let hello1 = String::from("عليكم السلام");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("こんにちは");
    let hello5 = String::from("안녕하세요");
    let hello6 = String::from("你好");

    let mut fix = String::from("$$$$");
    fix.push_str(&s);
    fix.push_str(&hello1);
    fix.push_str(&hello2);
    fix.push_str(&hello3);
    fix.push_str(&hello4);
    fix.push_str(&hello5);
    fix.push_str(&hello6);
    //主要为了各种字符组合后再转成char数组会是什么效果
    strConvertVecAndArray(&fix);
}

fn strConvertVecAndArray(para: &str) {

    //1. Vec<char>
    let charVec: Vec<char> = para.chars().collect::<Vec<_>>();
    println!("Vec<char>:{:?}", charVec);
    

    println!("");

    //2. [char] 代码不够精简，很啰嗦
    let length = para.chars().count();
    assert_eq!(length, 58);
    //
    let mut charArray:[char; 58] = ['0'; 58];
    //
    let chars = para.chars();
    let mut i = 0;
    for character in chars {
        charArray[i] = character;
        i += 1;
    }

    println!("[charArray]:{:?}", charArray);
}
