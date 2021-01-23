#[path = "./tokens.rs"]
mod tokens;
pub fn lex(file: std::vec::Vec<std::vec::Vec<&str>>,pos: usize, inside: String ) {
if inside == "start" {
let currentchar = &file[pos];
println!("{:?}",file);
}
}
