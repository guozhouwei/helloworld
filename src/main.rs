mod asciiutil;



fn main() {

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
    