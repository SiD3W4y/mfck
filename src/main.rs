mod lexer;

use lexer::{Lexer,Token};

fn main() {
    let mut l = Lexer::new();
    l.process_string(String::from("a <- a + 2"));
}
