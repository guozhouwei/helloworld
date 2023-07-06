

/*
 * 循环打印从’A’~’z’ 之间的所有字符
 */
pub fn print_ascii_fromA2z() -> Vec<char> {

    let mut char_vec = Vec::new();

    for c in 'A'..='z' {
        char_vec.push(c)
    }
    char_vec
}