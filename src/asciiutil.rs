
pub mod asciiprint;

/*
 * 循环打印从’a’~’Z’ 之间的所有字符
 */
pub fn print_ascii_froma2Z() -> Vec<char> {

    let mut char_vec = Vec::new();

    for c in ('Z'..='a').rev() {
        //print!("{c}\t");
        char_vec.push(c);
    }
    char_vec
}
