fn main() {
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
