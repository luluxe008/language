#![allow(dead_code)]


use std::str::Chars;
use std::iter::Peekable;

use crate::errors::{Error, Location, CompilerResult, PartialLocation};



#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token{
    Plus,
    Minus,
    Mul,
    Div,
    Int(u64),
    String(String),
    Identifier(String),
    Keyword(
      String // FIXME maybe an enum instead of a string?
    ),
    Coma,
    LParen,
    RParen,
    Assign,
    Error // if error
}

use Token as T;

pub struct Tokenizer<'a>{
    tk: Vec<Token>,
    err: Vec<Error>,
    iter: Peekable<Chars<'a>>,
    curr: Option<char>,
    line: String,
    pl: PartialLocation,
    char_pos: i64, // here we need to use i64 instead of u32, because u32 can't handle negative number i32::MAX is smaller than u32::MAX
}


#[allow(dead_code)]
impl<'a> Tokenizer<'a>{

    /// create a new tokenizer with a string in input
    /// This string must live longer than Tokenizer 
    pub fn new(line: &'a str, pl: PartialLocation) -> Self{
        Tokenizer{
            tk: Vec::new(),
            err: Vec::new(),
            iter: line.chars().peekable(),
            curr: None,
            line: line.into(),
            pl: pl,
            char_pos: -1
        }
        
    }
    

    fn advance(&mut self){
        self.curr = self.iter.next();
        self.char_pos += 1
    }

    fn make_int(&mut self) -> Token{

        let mut string_number = String::default();

        while self.curr.is_some() && self.curr.unwrap().is_ascii_digit(){
            string_number.push(self.curr.unwrap());
            self.advance();
        }

        T::Int(string_number.parse().expect("An error occured while parsing a string to an int, in Tokenizer::make_int"))

    }

    fn make_identifier(&mut self) -> Token{
        let mut identifier = String::default();
        let keywords = ["if", "else", "var", "print"]; 

        while self.curr.is_some() && self.curr.unwrap().is_alphabetic(){
            identifier.push(self.curr.unwrap());
            self.advance();
        }
        if keywords.contains(&identifier.as_str()){
            T::Keyword(identifier)
        }
        else{
            T::Identifier(identifier)
        }
    }

    fn make_string(&mut self) -> Token{
        let mut res = String::new();
        let mut terminated = false;

        self.advance(); // skip the "

        while let Some(char) = self.curr{
            if char == '\"'{
                terminated = true;
                self.advance();
                break;
            }
            res.push(char);
            self.advance();
        }

        if terminated{ 
            T::String(res)
        }
        else {// the string was not closed
            self.err.push(Error::string_closing(Location::new("not specified", 0, 0), self.line.clone()));

            T::Error // indicate the error
        }

    }


    /// push an error is the current char cannot be after a litteral number or a litteral string
    /// +, -, *, /, ), SPACE, COMA are the only character that can be directly after a number or a string
    fn after_number_or_string(&mut self){
        match self.curr {
            Some('+' | '-'| '*'| '/'| ' ' | ',' | ')') => (),
            Some(_) => {
                self.err.push(
                    Error::illegal_character(
                        Location::from(self.pl.clone()).char(self.char_pos as u32),
                     self.line.clone(), self.curr.unwrap() as u64) // FIXME bad error
                );
                
            }
            None => {
                // EOF
            }
        }
    }

    /// tokenize the string
    pub fn tokenize(&mut self){
        self.advance();
        while self.curr.is_some(){
            
            if self.curr.unwrap().is_ascii_digit(){
                let tmp = self.make_int();
                self.tk.push(tmp);
                self.after_number_or_string();
                continue;
            }

            else if self.curr.unwrap().is_ascii_alphabetic(){
                let tmp = self.make_identifier();
                self.tk.push(tmp);
                continue;
            }

            else if self.curr.unwrap() == '\"'{
                let tmp = self.make_string();
                self.tk.push(tmp);
                self.after_number_or_string();
                continue;
            }

            else if self.curr.unwrap() == ','{
                self.tk.push(T::Coma);
            }

            else if self.curr.unwrap() == '('{
                self.tk.push(T::RParen);
            }

            else if self.curr.unwrap() == ')'{
                self.tk.push(T::LParen);
            }

            else if self.curr.unwrap() == '+'{
                self.tk.push(T::Plus);
            }

            else if self.curr.unwrap() == '-'{
                self.tk.push(T::Minus);
            }

            else if self.curr.unwrap() == '*'{
                self.tk.push(T::Mul);
            }

            else if self.curr.unwrap() == '/'{
                self.tk.push(T::Div);
            }
            
            else if self.curr.unwrap() == '='{
                self.tk.push(T::Assign);
            }

            else if self.curr.unwrap() == ' '{
                self.advance(); // skip the space
                continue;
            }

            else {
                self.err.push(
                    Error::illegal_character(
                        Location::from(self.pl.clone()).char(self.char_pos as u32),
                         self.line.clone(),
                        self.curr.unwrap() as u64) 
                );
            }

            self.advance();
        }
    }
    

    pub fn result(self) -> CompilerResult<Vec<Token>>{
        if self.err.is_empty(){
            Ok(self.tk)
        }
        else {
            Err(self.err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let string = String::from("225 var wolrd (25+25-6*8/5,5)");
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();

        let res = tok.result();
        assert!(res.is_ok(), "there were an error");
        
        let tk = res.unwrap();
        

        //TODO  add testing
    }

    #[test]
    fn check_after_number(){
        let string = String::from("1024better");
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();

        let res = tok.result();
        assert!(res.is_err());


        // we check if we can add char behind number
        let string = String::from("1024) 25+ 12- 5* 4/ 4, 4525 2");
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();

        let res = tok.result();
        assert!(res.is_ok());
    }

    #[test]
    fn check_after_string(){
        let string = String::from(r#""hello wolrd"crash"#);
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();

        let res = tok.result(); // should return an error
        assert!(res.is_err());

    }

    #[test]
    fn check_illegal_char(){
        let string = String::from("225 13 é");
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();

        let res = tok.result(); // should return an error
        assert!(res.is_err());

    }

    #[test]
    fn check_string(){
        let string = String::from(r#""hello world" "125""#);
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();
        let res = tok.result();
        assert!(res.is_ok());
        let unwrapped = res.unwrap();
        
        assert_eq!(unwrapped[0], Token::String("hello world".into()));
        assert_eq!(unwrapped[1], Token::String("125".into()));

    }

    #[test]
    fn check_error_string(){
        let string = String::from(r#""unclosed string..."#);
        let mut tok = Tokenizer::new(&string, PartialLocation::testing(0));
        tok.tokenize();
        let res = tok.result();
        
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(err.len(), 1);
    }
}
