#![allow(dead_code)]
use core::slice::Iter;
use std::iter::Peekable;

use crate::{token::Token, errors::{Error, CompilerResult, Location}};


/// all the operator.
/// Mul and Div won't be implement due to their complexity.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Operator{
    Plus,
    Minus,
    Mul,
    Div
}


/// A Node is value. A Node can be composed of a lot of other Node.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Node{
    IntLitteral(u64),
    StringLitteral(String),
    Identifier(String),
    BinaryExpr{
        l: Box<Node>,
        r: Box<Node>,
        opr: Operator
    }
}




/// A statement is a line of code that is not evaluable such as:
/// ```
/// var x = (25+25)
/// ```
/// This previous line can't be evalued, but it is composed of multiple node.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Statement{
    VarDeclaration{
        identifier: String,
        value: Node
    },
    Print{
        value: Node
    },
    VarEdit{
        identifier: String,
        value: Node
    },
    None
}

use Statement as ST;

/// contruct an Abstract Syntax Tree (AST) from a list of vectors
pub struct AbstractSyntaxTree<'a>{
    statement: Statement,
    tokens: Peekable<Iter<'a, Token>>,
    err: Vec<Error>,
    curr: Option<&'a Token>
}


impl<'a> AbstractSyntaxTree<'a>{

    pub fn new(tokens: &'a Vec<Token>) -> Self{
        Self { statement: ST::None, tokens: tokens.iter().peekable(), err: Vec::new(), curr: None}
    }

    fn advance(&mut self){
        self.curr = self.tokens.next();
    }

    fn make_var_statement(&mut self) -> Statement{
        // first token must be Keyword("var")
        // now we check that the second is an identifier

        todo!();
        // token[0] = Keyword
        // token[1] = Space
        // token[2] = "="
        /*
        self.advance();

        match self.curr {
            Some(token) => {
                
                Statement::VarDeclaration { identifier: id, value: val }
            },


            None => {
                self.err.push(Error::excepted_token(Location::new("stdin?", 0, 0), "line blabal", "Identifier"));
                Statement::None
            }
        }
         */
    }

    pub fn build_tree(&mut self){
        self.advance();

        if let Some(tk) = self.curr{
            if tk == &Token::Keyword("var".into()){
                self.make_var_statement();
            }
        }

    }

    pub fn result(self) -> CompilerResult<Statement>{
        if self.err.is_empty(){
            Ok(self.statement)
        }
        else {
            Err(self.err)
        }
    }

}

#[cfg(test)]
mod test{
    use crate::{token::Tokenizer, errors::PartialLocation};

    use super::*;

    #[test]
    #[ignore = "unfinished"]
    fn var_declaration(){
        let mut token = Tokenizer::new("var hello = 122", PartialLocation::testing(0));
        token.tokenize();
        let result = token.result().unwrap();

        let mut parser = AbstractSyntaxTree::new(&result);
        parser.build_tree();
        dbg!(parser.result());
    }

}