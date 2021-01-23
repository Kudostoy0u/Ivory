/*
Time to put a lot of comments in this code because of the sheer complexity
*/
// Dependencies, filesystem and lexer module
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
mod lexer;

// Files to write and read to
const OUTPUT: &str = "./run/run.rs";
const FILENAME: &str = "./index.suk";

fn main() {
  // Read .suk file
  let contents = fs::read_to_string(FILENAME)
      .expect("Something went wrong reading the file");

  // Turn it into a family (lol) friendly format for lexing
  let mut file = contents.split("\n").map(|x| {
    let mut arr = x.split("").collect::<Vec<_>>();
    arr.remove(0);
    arr.pop();
    return arr;
    }).collect::<Vec<_>>();
    
  // Lex it (see lexer.rs)
  lexer::lex(file,0,"start".to_string());
}
