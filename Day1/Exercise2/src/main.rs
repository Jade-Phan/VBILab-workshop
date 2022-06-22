use std::io;
use regex::{RegexBuilder};

fn receive_input() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str);
    str.pop();
    str
}
fn count(sentence:&str) -> u32{
    let _sentence ="Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Justo laoreet sit amet cursus sit amet dictum sit. Ante metus dictum at tempor commodo ullamcorper a lacus. Ultricies mi eget mauris pharetra. Nisi scelerisque eu ultrices vitae auctor eu augue ut.";
    let re = RegexBuilder::new(sentence)
        .case_insensitive(true)
        .build()
        .expect("Invalid");
    re.find_iter(_sentence).count() as u32
}
fn main() {
    let mut str = String::new();
    println!("Enter string: ");
    str = receive_input();
    println!("{}",count(&str));
}