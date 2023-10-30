use errors::{PartialLocation, display_errors};
use token::Tokenizer;

mod token;
mod errors;
mod ast;

struct JIT{

}

impl JIT{

    /// run the Tokenizer
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
    let mut intepreter = JIT{};
    intepreter.run_stdio();
}