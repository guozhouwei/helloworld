

/*
 * 循环打印从’A’~’z’ 之间的所有字符
 */
pub fn printAsciiFromA2z() {
    for ascii2 in 'A'..='z' {
        print!("{ascii2} \t");
    }
    println!("");
}