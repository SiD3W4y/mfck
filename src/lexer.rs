use std::vec::Vec;
use std::fmt;
use std::str::FromStr;

pub enum Token {
    Symbol(String),
    Number(u32),
    Operator(String),
    Assign,
    Equal,
    If,
    While,
}

impl fmt::Display for Token {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Token::Operator(ref val) => write!(f,"OP = \"{}\"",val),
            Token::Symbol(ref val) => write!(f,"{}",val),
            Token::Assign => write!(f,"ASSIGN"),
            Token::Equal => write!(f,"EQ"),
            Token::If => write!(f,"IF"),
            Token::While => write!(f,"WHILE"),
            Token::Number(ref val) => write!(f,"NUMBER = {}",val)
        }
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: Vec::new() }
    }

    pub fn add_token(&mut self,t: Token) {
        self.tokens.push(t);
    }
    
    // Process string into Vec<String> 
    // We will then process them into tokens
    fn split_string(&mut self,data: String) -> Vec<String> {
        let mut current_tok = String::new();
        let mut chunks: Vec<String> = Vec::new();

        for c in data.chars() {
            match c {
                ' ' | '\t' | '\n' => {
                    if current_tok.len() != 0 {
                        chunks.push(current_tok);
                        current_tok = String::new();
                    }
                },
                _ => current_tok.push(c)
            }
        }

        if current_tok.len() != 0 {
            chunks.push(current_tok);
        }

        chunks
    }

    pub fn process_string(&mut self,data: String){
        let chunks = self.split_string(data);

        for chk in chunks {
            match chk.as_ref() {
                "<-" => self.add_token(Token::Assign),
                "+" | "-" | "*" => self.add_token(Token::Operator(chk.to_owned())),
                "if" => self.add_token(Token::If),
                "while" => self.add_token(Token::While),
                x if u32::from_str(x).is_ok() => self.add_token(Token::Number(u32::from_str(x).unwrap())), 
                _ => self.add_token(Token::Symbol(chk.to_owned()))
            }
        }
    }
}
