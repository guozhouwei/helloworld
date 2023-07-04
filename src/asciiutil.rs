
pub mod asciiprint;

/*
 * 循环打印从’a’~’Z’ 之间的所有字符
 */
pub fn printAsciiFroma2Z() -> Vec<char> {

    let mut charVec = Vec::new();

    for c in ('Z'..='a').rev() {
        //print!("{c}\t");
        charVec.push(c);
    }
    charVec
}
