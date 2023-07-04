

/*
 * 循环打印从’A’~’z’ 之间的所有字符
 */
pub fn printAsciiFromA2z() -> Vec<char> {

    let mut charVec = Vec::new();

    for c in 'A'..='z' {
        charVec.push(c)
    }
    charVec
}