use css_parser::parser::{parse, stringify};
use std::fs::read_to_string;

fn main() {
    let file_path = "./samples/tmp.css";
    let contents = read_to_string(&file_path).expect("Something went wrong reading the file");

    println!("{}", stringify(parse(&contents)));
    // assert_eq!(contents, stringify(parse(&contents)), "stringify(parse())");
}
