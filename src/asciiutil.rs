
mod asciiprint;

/*
 * 循环打印从’a’~’Z’ 之间的所有字符
 */
pub fn printAsciiFroma2Z() {
    for ascii1 in ('Z'..='a').rev() {
        print!("{ascii1}\t");
    }
    println!("");
}


/*
 * 循环打印从’A’~’z’ 之间的所有字符
 */
pub fn printAsciiFromA2z() {
    asciiprint::printAsciiFromA2z();
}
