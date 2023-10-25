use errors::{PartialLocation, display_errors};
use token::Tokenizer;

use crate::errors::{Error, Location};
use std::io::prelude::*;

mod token;
mod errors;
mod ast;

struct Interpreter{

}

impl Interpreter{
    pub fn execute_line(line: String){

    }

    // TODO add run_file(file: _);

    pub fn run_stdio(&mut self){
        let mut line = 0;
        loop {
            line += 1;
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(err) => {
                    eprintln!("error while reading stdin: {err}");
                    break
                }
            };
            input = input.trim_end().to_string();
            
            let mut tokenizer = Tokenizer::new(&input, PartialLocation::stdin(line));
            tokenizer.tokenize();
            let res = tokenizer.result();
            match res {
                Ok(tokens) => {
                    println!("{:?}", tokens);

                }
                Err(errs) => {
                    display_errors(errs);
                }
            }

        }
    }
}

fn main(){
    let mut intepreter = Interpreter{};
    intepreter.run_stdio();
}