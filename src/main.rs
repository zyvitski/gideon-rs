mod compiler;
use compiler::parser::Parser;

use std::fs::File;
use std::io::prelude::*;
use std::io;

fn load_source(filename: &str) -> Result<Vec<char>, io::Error> {
    let mut input = String::new();
    match File::open(filename) {
        Ok(mut file) => {
            file.read_to_string(&mut input).expect(
                "Unable to read from source",
            );
            Ok(input.chars().collect())
        }
        Err(what) => Err(what),
    }
}
fn main() {
    let filename = "language/json.gideon";
    if let Ok(mut chars) = load_source(filename) {
        let parser = Parser::new(chars.as_mut_slice());
        let cst = parser.parse();

        println!("{:?}", cst);
    }
}
