mod asciiutil;


fn main() {

    /*
     * 作业
     * （1）添加一个一层子模块，循环打印从’a’~’Z’ 之间的所有字符
     * （2）添加一个二层子模块，循环打印从’A’~’z’ 之间的所有字符
     * （3）使用Cargo编译运行此工程
     * （需要自行发现其中的细节，一个考点是：ascii码字符的顺序）
     */
    println!("打印 Z~a:");
    asciiutil::printAsciiFroma2Z();
    
    println!("打印 A~z:");
    asciiutil::printAsciiFromA2z()
    

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
    let (charVec, charArray) = strConvertVecAndArray(&fix);
    println!("字符串转Vec<char>:{:?}", charVec);
    println!("");
    println!("字符串转[char]:{:?}", charArray);


}

/*
 * String 转 元组(vec, [char])
 */
fn strConvertVecAndArray(para: &str) -> (Vec<char>, [char; 58]) {
    //1. Vec<char>
    let charVec: Vec<char> = para.chars().collect::<Vec<_>>();

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
    (charVec, charArray)
}